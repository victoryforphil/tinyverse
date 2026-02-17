use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use reqwest::Method;
use reqwest::blocking::Client;
use serde_json::{Value, json};

use crate::chat::{ChatMessage, ChatMessageRole, ChatState};

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4096";
const DEFAULT_SYNC_INTERVAL: Duration = Duration::from_secs(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatConnectionMode {
    OpencodeApi,
    TmuxFallback,
    Offline,
}

impl ChatConnectionMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::OpencodeApi => "api",
            Self::TmuxFallback => "tmux-fallback",
            Self::Offline => "offline",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatBridgeStatus {
    pub mode: ChatConnectionMode,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct DispatchOutcome {
    pub via_opencode: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
struct OpencodeSession {
    id: String,
    title: String,
}

#[derive(Debug, Clone)]
struct OpencodeSnapshot {
    sessions: Vec<OpencodeSession>,
    active_session_id: Option<String>,
    messages: Vec<ChatMessage>,
    models: Vec<String>,
    agents: Vec<String>,
}

#[derive(Debug, Clone)]
struct OpencodeClient {
    base_url: String,
    directory: String,
    username: String,
    password: Option<String>,
    http: Client,
}

impl OpencodeClient {
    fn from_env(directory: String) -> Result<Option<Self>> {
        let disabled = std::env::var("TINYVERSE_CHAT_OPENCODE_DISABLE")
            .ok()
            .map(|value| value.trim() == "1")
            .unwrap_or(false);
        if disabled {
            return Ok(None);
        }

        let base_url = std::env::var("TINYVERSE_CHAT_OPENCODE_BASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| {
                std::env::var("DARK_CHAT_BASE_URL")
                    .ok()
                    .filter(|value| !value.trim().is_empty())
            })
            .unwrap_or_else(|| String::from(DEFAULT_BASE_URL));

        let username = std::env::var("OPENCODE_SERVER_USERNAME")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| String::from("opencode"));
        let password = std::env::var("OPENCODE_SERVER_PASSWORD")
            .ok()
            .filter(|value| !value.trim().is_empty());

        let http = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .context("failed to build OpenCode HTTP client")?;

        Ok(Some(Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
            directory,
            username,
            password,
            http,
        }))
    }

    fn health(&self) -> Result<()> {
        let _ = self.request_json_with_fallback(Method::GET, &["/", "/health"], None)?;
        Ok(())
    }

    fn snapshot(&self, preferred_session_id: Option<&str>) -> Result<OpencodeSnapshot> {
        let sessions = self.list_sessions()?;
        let active_session_id = preferred_session_id
            .and_then(|wanted| {
                sessions
                    .iter()
                    .any(|value| value.id == wanted)
                    .then_some(wanted)
            })
            .map(ToOwned::to_owned)
            .or_else(|| sessions.first().map(|value| value.id.clone()));

        let messages = if let Some(session_id) = active_session_id.as_deref() {
            self.list_messages(session_id)?
        } else {
            Vec::new()
        };

        Ok(OpencodeSnapshot {
            sessions,
            active_session_id,
            messages,
            models: self.list_models()?,
            agents: self.list_agents()?,
        })
    }

    fn create_session(&self, title: &str) -> Result<OpencodeSession> {
        let body = json!({ "title": title });
        let value =
            self.request_json_with_fallback(Method::POST, &["/session", "/session/"], Some(body))?;
        let data = unwrap_data(value);
        let id = data
            .get("id")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned)
            .context("created session missing id")?;
        let title = data
            .get("title")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| String::from("Untitled session"));
        Ok(OpencodeSession { id, title })
    }

    fn send_prompt(&self, session_id: &str, prompt: &str, model: &str, agent: &str) -> Result<()> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            bail!("prompt cannot be empty");
        }

        let mut body = json!({
            "noReply": false,
            "parts": [{
                "type": "text",
                "text": trimmed,
            }],
        });

        if let Some((provider_id, model_id)) = parse_model_selector(model) {
            body["model"] = json!({
                "providerID": provider_id,
                "modelID": model_id,
            });
        }

        if !agent.trim().is_empty() {
            body["agent"] = Value::String(agent.trim().to_owned());
        }

        let path_message = format!("/session/{session_id}/message");
        let path_prompt = format!("/session/{session_id}/prompt");
        let _ = self.request_json_with_fallback(
            Method::POST,
            &[path_message.as_str(), path_prompt.as_str()],
            Some(body),
        )?;
        Ok(())
    }

    fn list_sessions(&self) -> Result<Vec<OpencodeSession>> {
        let payload =
            self.request_json_with_fallback(Method::GET, &["/session", "/session/"], None)?;
        let data = unwrap_data(payload);
        let records = match data {
            Value::Array(items) => items,
            _ => Vec::new(),
        };

        let mut sessions = records
            .into_iter()
            .filter_map(|record| {
                let id = record
                    .get("id")
                    .and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())?
                    .to_owned();
                let title = record
                    .get("title")
                    .or_else(|| record.get("name"))
                    .and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| String::from("Untitled session"));
                Some(OpencodeSession { id, title })
            })
            .collect::<Vec<_>>();

        sessions.sort_by(|left, right| right.id.cmp(&left.id));
        Ok(sessions)
    }

    fn list_messages(&self, session_id: &str) -> Result<Vec<ChatMessage>> {
        let path_message = format!("/session/{session_id}/message");
        let path_messages = format!("/session/{session_id}/messages");
        let payload = self.request_json_with_fallback(
            Method::GET,
            &[path_message.as_str(), path_messages.as_str()],
            None,
        )?;
        let data = unwrap_data(payload);

        let records = match data {
            Value::Array(items) => items,
            _ => Vec::new(),
        };

        let mut messages = records
            .into_iter()
            .map(|record| {
                let info = record.get("info").cloned().unwrap_or(Value::Null);
                let role = info
                    .get("role")
                    .and_then(Value::as_str)
                    .map(ChatMessageRole::from_wire)
                    .unwrap_or(ChatMessageRole::Assistant);
                let id = info
                    .get("id")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                let created_at = info
                    .get("createdAt")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
                    .or_else(|| {
                        info.get("time")
                            .and_then(|value| value.get("created"))
                            .and_then(parse_unix)
                            .map(unix_label)
                    })
                    .unwrap_or_else(now_label);

                let text = record
                    .get("parts")
                    .map(extract_message_text)
                    .filter(|value| !value.trim().is_empty())
                    .or_else(|| {
                        record
                            .get("text")
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned)
                    })
                    .unwrap_or_else(|| String::from("(empty message)"));

                ChatMessage {
                    id,
                    role,
                    text,
                    created_at,
                }
            })
            .collect::<Vec<_>>();

        messages.sort_by(|left, right| left.created_at.cmp(&right.created_at));
        Ok(messages)
    }

    fn list_agents(&self) -> Result<Vec<String>> {
        let payload = self.request_json_with_fallback(Method::GET, &["/agent", "/agent/"], None)?;
        let data = unwrap_data(payload);

        let mut items = extract_string_options(&data, &["id", "name", "key", "slug", "value"]);
        items.sort();
        items.dedup();
        Ok(items)
    }

    fn list_models(&self) -> Result<Vec<String>> {
        let payload = self.request_json_with_fallback(
            Method::GET,
            &["/config/providers", "/config/providers/"],
            None,
        )?;
        let data = unwrap_data(payload);
        let mut models = Vec::new();

        if let Some(defaults) = data
            .get("default")
            .or_else(|| data.get("defaults"))
            .and_then(Value::as_object)
        {
            for value in defaults.values() {
                if let Some(model) = value.as_str() {
                    let trimmed = model.trim();
                    if !trimmed.is_empty() {
                        models.push(trimmed.to_owned());
                    }
                }
            }
        }

        if let Some(providers) = data.get("providers").and_then(Value::as_array) {
            for provider in providers {
                let provider_id = provider
                    .get("id")
                    .or_else(|| provider.get("key"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim()
                    .to_owned();

                if let Some(model_entries) = provider.get("models") {
                    if let Some(entries) = model_entries.as_array() {
                        for model in entries {
                            let label = extract_model_label(model, None);
                            push_model_label(&mut models, &provider_id, label);
                        }
                    } else if let Some(entries) = model_entries.as_object() {
                        for (key, model) in entries {
                            let label = extract_model_label(model, Some(key));
                            push_model_label(&mut models, &provider_id, label);
                        }
                    }
                }
            }
        }

        models.sort();
        models.dedup();
        Ok(models)
    }

    fn request_json_with_fallback(
        &self,
        method: Method,
        paths: &[&str],
        body: Option<Value>,
    ) -> Result<Value> {
        let mut first_error: Option<anyhow::Error> = None;

        for path in paths {
            let url = format!("{}{}", self.base_url, normalize_path(path));
            let mut request = self
                .http
                .request(method.clone(), &url)
                .query(&[("directory", &self.directory)]);

            if let Some(password) = self.password.as_ref() {
                request = request.basic_auth(&self.username, Some(password));
            }

            if let Some(payload) = body.clone() {
                request = request.json(&payload);
            }

            let response = request
                .send()
                .with_context(|| format!("OpenCode request failed: {path}"))?;
            let status = response.status();
            let text = response
                .text()
                .with_context(|| format!("OpenCode response read failed: {path}"))?;

            if status.as_u16() == 404 {
                continue;
            }

            let parsed = parse_response_body(&text);
            if !status.is_success() {
                let error =
                    anyhow::anyhow!("OpenCode request error {} at {}", status.as_u16(), path);
                if first_error.is_none() {
                    first_error = Some(error);
                }
                continue;
            }

            return Ok(parsed);
        }

        if let Some(error) = first_error {
            return Err(error);
        }

        bail!("OpenCode fallback paths returned 404")
    }
}

pub struct ChatBridge {
    opencode: Option<OpencodeClient>,
    active_session_id: Option<String>,
    mode: ChatConnectionMode,
    mode_detail: String,
    last_sync_at: Option<Instant>,
    sync_interval: Duration,
}

impl ChatBridge {
    pub fn from_env() -> Self {
        let directory = std::env::current_dir()
            .ok()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("."));

        let opencode = OpencodeClient::from_env(directory).ok().flatten();
        let mode = if opencode.is_some() {
            ChatConnectionMode::TmuxFallback
        } else {
            ChatConnectionMode::Offline
        };

        Self {
            opencode,
            active_session_id: None,
            mode,
            mode_detail: String::from("waiting for sync"),
            last_sync_at: None,
            sync_interval: DEFAULT_SYNC_INTERVAL,
        }
    }

    pub fn status(&self) -> ChatBridgeStatus {
        ChatBridgeStatus {
            mode: self.mode,
            detail: self.mode_detail.clone(),
        }
    }

    pub fn sync_if_due(&mut self, chat: &mut ChatState) {
        if self
            .last_sync_at
            .map(|instant| instant.elapsed() < self.sync_interval)
            .unwrap_or(false)
        {
            return;
        }
        self.sync_now(chat);
    }

    pub fn sync_now(&mut self, chat: &mut ChatState) {
        self.last_sync_at = Some(Instant::now());
        let Some(opencode) = self.opencode.as_ref() else {
            self.mode = ChatConnectionMode::Offline;
            self.mode_detail = String::from("OpenCode API disabled");
            return;
        };

        let sync_result = (|| -> Result<OpencodeSnapshot> {
            opencode.health()?;
            let mut snapshot = opencode.snapshot(self.active_session_id.as_deref())?;
            if snapshot.active_session_id.is_none() {
                let created = opencode.create_session("tinyverse chat")?;
                snapshot.sessions.insert(0, created.clone());
                snapshot.active_session_id = Some(created.id.clone());
                snapshot.messages = opencode.list_messages(&created.id)?;
            }
            Ok(snapshot)
        })();

        match sync_result {
            Ok(snapshot) => {
                self.active_session_id = snapshot.active_session_id;
                chat.set_models(snapshot.models);
                chat.set_agents(snapshot.agents);
                chat.set_messages(snapshot.messages);
                let session_label = snapshot
                    .sessions
                    .iter()
                    .find(|session| self.active_session_id.as_deref() == Some(session.id.as_str()))
                    .map(|session| session.title.clone())
                    .unwrap_or_else(|| String::from("unknown"));
                self.mode = ChatConnectionMode::OpencodeApi;
                self.mode_detail = format!("OpenCode session: {session_label}");
            }
            Err(error) => {
                self.mode = ChatConnectionMode::TmuxFallback;
                self.mode_detail = format!("OpenCode unavailable: {error}");
            }
        }
    }

    pub fn send_prompt(
        &mut self,
        prompt: &str,
        active_model: &str,
        active_agent: &str,
    ) -> DispatchOutcome {
        let Some(opencode) = self.opencode.as_ref() else {
            self.mode = ChatConnectionMode::Offline;
            self.mode_detail = String::from("OpenCode API disabled");
            return DispatchOutcome {
                via_opencode: false,
                detail: String::from("OpenCode disabled; using tmux fallback"),
            };
        };

        let Some(session_id) = self.active_session_id.as_deref() else {
            self.mode = ChatConnectionMode::TmuxFallback;
            self.mode_detail = String::from("no OpenCode session selected");
            return DispatchOutcome {
                via_opencode: false,
                detail: String::from("No OpenCode session; using tmux fallback"),
            };
        };

        match opencode.send_prompt(session_id, prompt, active_model, active_agent) {
            Ok(()) => {
                self.mode = ChatConnectionMode::OpencodeApi;
                self.mode_detail = String::from("prompt sent via OpenCode API");
                DispatchOutcome {
                    via_opencode: true,
                    detail: String::from("Prompt sent via OpenCode API"),
                }
            }
            Err(error) => {
                self.mode = ChatConnectionMode::TmuxFallback;
                self.mode_detail = format!("OpenCode send failed: {error}");
                DispatchOutcome {
                    via_opencode: false,
                    detail: format!("OpenCode send failed ({error}); using tmux fallback"),
                }
            }
        }
    }
}

fn unwrap_data(value: Value) -> Value {
    value.get("data").cloned().unwrap_or(value)
}

fn parse_response_body(text: &str) -> Value {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Value::Object(Default::default());
    }
    serde_json::from_str(trimmed).unwrap_or_else(|_| json!({ "message": trimmed }))
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{path}")
    }
}

fn parse_model_selector(value: &str) -> Option<(String, String)> {
    let trimmed = value.trim();
    let (provider, model) = trimmed.split_once('/')?;
    let provider = provider.trim();
    let model = model.trim();
    if provider.is_empty() || model.is_empty() {
        return None;
    }
    Some((provider.to_owned(), model.to_owned()))
}

fn extract_model_label(model: &Value, fallback: Option<&str>) -> String {
    model
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .or_else(|| {
            model
                .get("id")
                .or_else(|| model.get("model"))
                .or_else(|| model.get("name"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        })
        .or_else(|| {
            fallback
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        })
        .unwrap_or_default()
}

fn push_model_label(models: &mut Vec<String>, provider_id: &str, label: String) {
    if label.is_empty() {
        return;
    }
    if label.contains('/') || provider_id.is_empty() {
        models.push(label);
    } else {
        models.push(format!("{provider_id}/{label}"));
    }
}

fn extract_string_options(value: &Value, keys: &[&str]) -> Vec<String> {
    let mut output = Vec::new();

    if let Some(list) = value.as_array() {
        for item in list {
            if let Some(text) = item.as_str() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    output.push(trimmed.to_owned());
                }
                continue;
            }

            if let Some(object) = item.as_object() {
                for key in keys {
                    if let Some(text) = object.get(*key).and_then(Value::as_str) {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            output.push(trimmed.to_owned());
                            break;
                        }
                    }
                }
            }
        }
    }

    output
}

fn extract_message_text(value: &Value) -> String {
    let Some(parts) = value.as_array() else {
        return String::new();
    };

    let mut lines = Vec::new();
    for part in parts {
        let part_type = part.get("type").and_then(Value::as_str).unwrap_or("text");
        match part_type {
            "text" => {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        lines.push(trimmed.to_owned());
                    }
                }
            }
            "thinking" => {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        lines.push(format!("[thinking] {trimmed}"));
                    }
                }
            }
            "tool_call" => {
                let tool = part
                    .get("tool")
                    .or_else(|| part.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("tool");
                lines.push(format!("[tool:{tool}]"));
            }
            _ => {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        lines.push(trimmed.to_owned());
                    }
                }
            }
        }
    }

    lines.join("\n")
}

fn parse_unix(value: &Value) -> Option<i64> {
    if let Some(number) = value.as_i64() {
        return Some(number / 1000);
    }
    value.as_f64().map(|number| (number / 1000.0) as i64)
}

fn unix_label(seconds: i64) -> String {
    format!("unix:{seconds}")
}

fn now_label() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}
