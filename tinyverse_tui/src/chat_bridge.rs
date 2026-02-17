use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use reqwest::Method;
use reqwest::blocking::Client;
use serde_json::{Value, json};

use crate::chat::{ChatMessage, ChatMessagePart, ChatMessageRole, ChatState};
use crate::logger::log_line;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4150";
const DEFAULT_SYNC_INTERVAL: Duration = Duration::from_secs(2);

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
pub struct ChatSessionSummary {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone)]
struct OpencodeSnapshot {
    sessions: Vec<ChatSessionSummary>,
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
        let mut active_session_id = preferred_session_id
            .and_then(|wanted| {
                sessions
                    .iter()
                    .any(|value| value.id == wanted)
                    .then_some(wanted)
            })
            .map(ToOwned::to_owned)
            .or_else(|| sessions.first().map(|value| value.id.clone()));

        let messages = if let Some(session_id) = active_session_id.clone() {
            match self.list_messages(&session_id) {
                Ok(messages) => messages,
                Err(error) => {
                    log_line(&format!(
                        "chat bridge session lost: {session_id}, error: {error}"
                    ));
                    active_session_id = None;
                    Vec::new()
                }
            }
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

    fn create_session(&self, title: &str) -> Result<ChatSessionSummary> {
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
        Ok(ChatSessionSummary {
            id,
            title,
            parent_id: None,
        })
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

    fn list_sessions(&self) -> Result<Vec<ChatSessionSummary>> {
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
                let record_directory = record
                    .get("directory")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned);
                if let Some(directory) = record_directory.as_deref()
                    && directory != self.directory
                {
                    return None;
                }

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
                let parent_id = record
                    .get("parentID")
                    .or_else(|| record.get("parent_id"))
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned);
                let updated_at = record
                    .get("time")
                    .and_then(Value::as_object)
                    .and_then(|time| {
                        time.get("updated")
                            .and_then(Value::as_i64)
                            .or_else(|| time.get("created").and_then(Value::as_i64))
                    })
                    .unwrap_or_default();

                Some((
                    ChatSessionSummary {
                        id,
                        title,
                        parent_id,
                    },
                    updated_at,
                ))
            })
            .collect::<Vec<_>>();

        sessions.sort_by(
            |(left_summary, left_updated), (right_summary, right_updated)| {
                right_updated
                    .cmp(left_updated)
                    .then_with(|| right_summary.id.cmp(&left_summary.id))
            },
        );

        Ok(sessions
            .into_iter()
            .map(|(summary, _updated)| summary)
            .collect())
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

                let mut parts = record
                    .get("parts")
                    .map(extract_message_parts)
                    .unwrap_or_default();
                if parts.is_empty()
                    && let Some(text) = record
                        .get("text")
                        .or_else(|| record.get("content"))
                        .and_then(Value::as_str)
                {
                    parts = split_text_and_code_parts(text);
                }

                if parts.is_empty() {
                    parts.push(ChatMessagePart::Text(String::from("(empty message)")));
                }
                let text = flatten_message_parts(&parts);

                ChatMessage {
                    id,
                    role,
                    text,
                    parts,
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
    sync_tx: Option<Sender<SyncWorkerCommand>>,
    sync_rx: Option<Receiver<SyncWorkerEvent>>,
    sync_in_flight: bool,
    sync_requested_while_busy: bool,
    next_sync_request_id: u64,
    latest_sync_request_id: u64,
    active_session_id: Option<String>,
    known_sessions: Vec<ChatSessionSummary>,
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
        let (sync_tx, sync_rx) = if let Some(opencode) = opencode.as_ref() {
            let (sync_tx, sync_rx) = spawn_sync_worker(opencode.clone());
            (Some(sync_tx), Some(sync_rx))
        } else {
            (None, None)
        };
        let mode = if opencode.is_some() {
            ChatConnectionMode::TmuxFallback
        } else {
            ChatConnectionMode::Offline
        };

        Self {
            opencode,
            sync_tx,
            sync_rx,
            sync_in_flight: false,
            sync_requested_while_busy: false,
            next_sync_request_id: 1,
            latest_sync_request_id: 0,
            active_session_id: None,
            known_sessions: Vec::new(),
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

    pub fn sessions(&self) -> &[ChatSessionSummary] {
        &self.known_sessions
    }

    pub fn active_session_id(&self) -> Option<&str> {
        self.active_session_id.as_deref()
    }

    pub fn opencode_base_url(&self) -> Option<String> {
        self.opencode.as_ref().map(|client| client.base_url.clone())
    }

    pub fn create_session_for_spawn(
        &mut self,
        chat: &mut ChatState,
        title: &str,
    ) -> Result<String> {
        let opencode = self
            .opencode
            .as_ref()
            .context("OpenCode API unavailable for spawn attach")?;
        let created = opencode.create_session(title)?;
        self.active_session_id = Some(created.id.clone());
        self.known_sessions.insert(0, created.clone());
        self.mode = ChatConnectionMode::OpencodeApi;
        self.mode_detail = format!("OpenCode session: {}", created.title);
        chat.clear_messages();
        self.request_sync(true);
        Ok(created.id)
    }

    pub fn set_directory(&mut self, directory: &str) {
        let Some(current_directory) = self.opencode.as_ref().map(|value| value.directory.clone())
        else {
            return;
        };

        let trimmed = directory.trim();
        if trimmed.is_empty() || current_directory == trimmed {
            return;
        }

        if let Some(opencode) = self.opencode.as_mut() {
            opencode.directory = trimmed.to_owned();
        }
        self.active_session_id = None;
        self.known_sessions.clear();
        self.last_sync_at = None;
        self.sync_requested_while_busy = true;
        log_line(&format!("chat bridge directory set: {trimmed}"));
        self.request_sync(true);
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        let Some(current_base_url) = self.opencode.as_ref().map(|value| value.base_url.clone())
        else {
            return;
        };

        let trimmed = base_url.trim().trim_end_matches('/');
        if trimmed.is_empty() || current_base_url == trimmed {
            return;
        }

        if let Some(opencode) = self.opencode.as_mut() {
            opencode.base_url = trimmed.to_owned();
        }
        self.active_session_id = None;
        self.known_sessions.clear();
        self.last_sync_at = None;
        self.sync_requested_while_busy = true;
        log_line(&format!("chat bridge base url set: {trimmed}"));
        self.request_sync(true);
    }

    pub fn set_active_session(&mut self, _chat: &mut ChatState, session_id: &str) -> bool {
        let wanted = session_id.trim();
        if wanted.is_empty() {
            return false;
        }

        self.active_session_id = Some(wanted.to_owned());
        self.request_sync(true);
        true
    }

    pub fn mark_session_unbound(&mut self, chat: &mut ChatState, detail: &str) {
        self.active_session_id = None;
        self.known_sessions.clear();
        self.sync_in_flight = false;
        self.sync_requested_while_busy = false;
        self.last_sync_at = None;
        self.mode = ChatConnectionMode::TmuxFallback;
        self.mode_detail = detail.to_owned();
        chat.clear_messages();
    }

    pub fn sync_if_due(&mut self, chat: &mut ChatState) {
        self.consume_sync_events(chat);

        if self.sync_requested_while_busy && !self.sync_in_flight {
            self.sync_requested_while_busy = false;
            self.request_sync(true);
            return;
        }

        self.request_sync(false);
    }

    pub fn sync_now(&mut self, chat: &mut ChatState) {
        self.consume_sync_events(chat);
        self.request_sync(true);
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
                self.sync_requested_while_busy = true;
                self.request_sync(true);
                DispatchOutcome {
                    via_opencode: true,
                    detail: String::from("Prompt sent via OpenCode API"),
                }
            }
            Err(error) => {
                self.mode = ChatConnectionMode::TmuxFallback;
                if is_session_missing_error(&error) {
                    self.active_session_id = None;
                    self.last_sync_at = None;
                    self.sync_requested_while_busy = true;
                }
                let summary = summarize_bridge_error(&error);
                self.mode_detail = format!("OpenCode send failed: {summary}");
                log_line(&format!("chat bridge send error: {error:#}"));
                DispatchOutcome {
                    via_opencode: false,
                    detail: format!("OpenCode send failed ({summary}); using tmux fallback"),
                }
            }
        }
    }

    fn request_sync(&mut self, force: bool) {
        if self.opencode.is_none() {
            self.mode = ChatConnectionMode::Offline;
            self.mode_detail = String::from("OpenCode API disabled");
            self.known_sessions.clear();
            return;
        }

        if self.sync_in_flight {
            self.sync_requested_while_busy = true;
            return;
        }

        if !force
            && self
                .last_sync_at
                .map(|instant| instant.elapsed() < self.sync_interval)
                .unwrap_or(false)
        {
            return;
        }

        let Some(opencode) = self.opencode.as_ref() else {
            return;
        };
        let Some(sync_tx) = self.sync_tx.as_ref() else {
            self.mode = ChatConnectionMode::TmuxFallback;
            self.mode_detail = String::from("sync worker unavailable");
            return;
        };

        let request_id = self.next_sync_request_id;
        self.next_sync_request_id = self.next_sync_request_id.saturating_add(1);
        self.latest_sync_request_id = request_id;

        let send_result = sync_tx.send(SyncWorkerCommand::Sync {
            request_id,
            directory: opencode.directory.clone(),
            preferred_session_id: self.active_session_id.clone(),
        });

        match send_result {
            Ok(()) => {
                self.sync_in_flight = true;
                self.last_sync_at = Some(Instant::now());
            }
            Err(error) => {
                self.sync_in_flight = false;
                self.mode = ChatConnectionMode::TmuxFallback;
                self.mode_detail = format!(
                    "sync worker failed: {}",
                    truncate_error_label(&error.to_string())
                );
                log_line(&format!("chat bridge sync worker send error: {error}"));
            }
        }
    }

    fn consume_sync_events(&mut self, chat: &mut ChatState) {
        let Some(sync_rx) = self.sync_rx.as_ref() else {
            return;
        };

        loop {
            match sync_rx.try_recv() {
                Ok(SyncWorkerEvent::Synced { request_id, result }) => {
                    self.sync_in_flight = false;
                    if request_id != self.latest_sync_request_id {
                        continue;
                    }
                    match result {
                        Ok(snapshot) => {
                            let previous_session = self.active_session_id.clone();
                            self.active_session_id = snapshot.active_session_id;
                            self.known_sessions = if let Some(active_id) = self.active_session_id.as_deref() {
                                snapshot
                                    .sessions
                                    .iter()
                                    .filter(|session| session.id == active_id)
                                    .cloned()
                                    .collect()
                            } else {
                                snapshot.sessions.clone()
                            };
                            chat.set_models(snapshot.models);
                            chat.set_agents(snapshot.agents);
                            chat.set_messages(snapshot.messages);
                            let session_label = snapshot
                                .sessions
                                .iter()
                                .find(|session| {
                                    self.active_session_id.as_deref() == Some(session.id.as_str())
                                })
                                .map(|session| session.title.clone())
                                .unwrap_or_else(|| String::from("unknown"));
                            self.mode = ChatConnectionMode::OpencodeApi;
                            self.mode_detail = format!("OpenCode session: {session_label}");

                            if previous_session != self.active_session_id {
                                log_line(&format!(
                                    "chat bridge session switched: {:?} -> {:?}",
                                    previous_session, self.active_session_id
                                ));
                            }
                        }
                        Err(error_text) => {
                            self.mode = ChatConnectionMode::TmuxFallback;
                            let summary = summarize_bridge_error_text(&error_text);
                            self.mode_detail = format!("OpenCode unavailable: {summary}");
                            log_line(&format!("chat bridge sync error: {error_text}"));
                        }
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.sync_in_flight = false;
                    self.mode = ChatConnectionMode::TmuxFallback;
                    self.mode_detail = String::from("sync worker disconnected");
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
enum SyncWorkerCommand {
    Sync {
        request_id: u64,
        directory: String,
        preferred_session_id: Option<String>,
    },
}

#[derive(Debug)]
enum SyncWorkerEvent {
    Synced {
        request_id: u64,
        result: Result<OpencodeSnapshot, String>,
    },
}

fn spawn_sync_worker(
    opencode: OpencodeClient,
) -> (Sender<SyncWorkerCommand>, Receiver<SyncWorkerEvent>) {
    let (command_tx, command_rx) = mpsc::channel::<SyncWorkerCommand>();
    let (event_tx, event_rx) = mpsc::channel::<SyncWorkerEvent>();

    thread::spawn(move || {
        let mut opencode = opencode;
        while let Ok(command) = command_rx.recv() {
            match command {
                SyncWorkerCommand::Sync {
                    request_id,
                    directory,
                    preferred_session_id,
                } => {
                    opencode.directory = directory;
                    let result = sync_snapshot_once(&opencode, preferred_session_id.as_deref())
                        .map_err(|error| format!("{error:#}"));
                    if event_tx
                        .send(SyncWorkerEvent::Synced { request_id, result })
                        .is_err()
                    {
                        break;
                    }
                }
            }
        }
    });

    (command_tx, event_rx)
}

fn sync_snapshot_once(
    opencode: &OpencodeClient,
    preferred_session_id: Option<&str>,
) -> Result<OpencodeSnapshot> {
    opencode.health()?;
    let mut snapshot = opencode.snapshot(preferred_session_id)?;
    if snapshot.active_session_id.is_none() {
        let created = opencode.create_session("tinyverse chat")?;
        snapshot.sessions.insert(0, created.clone());
        snapshot.active_session_id = Some(created.id.clone());
        snapshot.messages = opencode.list_messages(&created.id)?;
    }
    Ok(snapshot)
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

fn is_session_missing_error(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    (message.contains("404") || message.contains("not found")) && message.contains("session")
}

fn summarize_bridge_error(error: &anyhow::Error) -> String {
    summarize_bridge_error_text(&error.root_cause().to_string())
}

fn summarize_bridge_error_text(error_text: &str) -> String {
    let root = error_text.trim();
    let lower = root.to_ascii_lowercase();
    if lower.contains("refused") || lower.contains("connect") {
        return String::from("server unreachable");
    }
    if lower.contains("timed out") || lower.contains("timeout") {
        return String::from("request timed out");
    }
    if lower.contains("404") || lower.contains("not found") {
        return String::from("endpoint not found");
    }

    truncate_error_label(root)
}

fn truncate_error_label(value: &str) -> String {
    let trimmed = value.trim();
    let max = 48usize;
    if trimmed.chars().count() <= max {
        return trimmed.to_owned();
    }
    let mut output = String::new();
    for ch in trimmed.chars().take(max.saturating_sub(1)) {
        output.push(ch);
    }
    output.push('…');
    output
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

fn extract_message_parts(value: &Value) -> Vec<ChatMessagePart> {
    let Some(parts) = value.as_array() else {
        return Vec::new();
    };

    let mut output = Vec::new();
    for part in parts {
        let part_type = part
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("text")
            .trim()
            .to_ascii_lowercase();
        match part_type {
            _ if matches!(
                part_type.as_str(),
                "text" | "assistant" | "message" | "markdown"
            ) =>
            {
                if let Some(text) = part
                    .get("text")
                    .or_else(|| part.get("content"))
                    .or_else(|| part.get("markdown"))
                    .and_then(Value::as_str)
                {
                    output.extend(split_text_and_code_parts(text));
                }
            }
            _ if matches!(part_type.as_str(), "thinking" | "reasoning") => {
                if let Some(text) = part
                    .get("text")
                    .or_else(|| part.get("content"))
                    .and_then(Value::as_str)
                {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        output.push(ChatMessagePart::Thinking(trimmed.to_owned()));
                    }
                }
            }
            _ if matches!(part_type.as_str(), "tool" | "tool_call" | "toolcall") => {
                let name = part
                    .get("tool")
                    .or_else(|| part.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("tool")
                    .to_owned();
                let state = part.get("state").unwrap_or(part);

                let input_value = state.get("input").or_else(|| part.get("input"));
                let output_value = state.get("output").or_else(|| part.get("output"));

                if matches!(name.as_str(), "bash" | "shell" | "terminal") {
                    let command = input_value
                        .and_then(|value| {
                            value
                                .get("command")
                                .or_else(|| value.get("text"))
                                .or_else(|| value.get("cmd"))
                                .and_then(Value::as_str)
                        })
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(ToOwned::to_owned);
                    if let Some(command) = command {
                        output.push(ChatMessagePart::ShellCommand(command));
                    }

                    if let Some(output_value) = output_value {
                        let shell_output = pretty_json(output_value);
                        if !shell_output.trim().is_empty() {
                            let exit_code = output_value
                                .get("exitCode")
                                .or_else(|| output_value.get("code"))
                                .and_then(Value::as_i64);
                            output.push(ChatMessagePart::ShellOutput {
                                output: shell_output,
                                exit_code,
                            });
                        }
                    }
                } else {
                    let input = input_value
                        .map(pretty_json)
                        .filter(|value| !value.trim().is_empty());
                    let output_text = output_value
                        .map(pretty_json)
                        .filter(|value| !value.trim().is_empty());

                    output.push(ChatMessagePart::ToolCall {
                        name,
                        input,
                        output: output_text,
                    });
                }
            }
            _ if part_type == "step-finish" => {}
            _ if part_type == "patch" => {
                let files = part
                    .get("files")
                    .and_then(Value::as_array)
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(Value::as_str)
                            .map(ToOwned::to_owned)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let hash = part
                    .get("hash")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");

                output.push(ChatMessagePart::ToolCall {
                    name: String::from("patch"),
                    input: if files.is_empty() {
                        Some(format!("hash: {hash}"))
                    } else {
                        Some(format!("hash: {hash}\nfiles:\n- {}", files.join("\n- ")))
                    },
                    output: None,
                });
            }
            _ if matches!(
                part_type.as_str(),
                "tool_result" | "tool_results" | "tool_output" | "result"
            ) =>
            {
                let rendered = part
                    .get("output")
                    .or_else(|| part.get("text"))
                    .or_else(|| part.get("content"))
                    .map(pretty_json)
                    .unwrap_or_default();
                if !rendered.trim().is_empty() {
                    output.push(ChatMessagePart::ShellOutput {
                        output: rendered,
                        exit_code: part.get("exitCode").and_then(Value::as_i64),
                    });
                }
            }
            _ if matches!(part_type.as_str(), "command" | "shell" | "bash") => {
                let command = part
                    .get("command")
                    .or_else(|| part.get("text"))
                    .or_else(|| part.get("content"))
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned);
                if let Some(command) = command {
                    output.push(ChatMessagePart::ShellCommand(command));
                }
                let shell_output = part
                    .get("output")
                    .or_else(|| part.get("result"))
                    .map(pretty_json)
                    .unwrap_or_default();
                if !shell_output.trim().is_empty() {
                    output.push(ChatMessagePart::ShellOutput {
                        output: shell_output,
                        exit_code: part.get("exitCode").and_then(Value::as_i64),
                    });
                }
            }
            _ if matches!(part_type.as_str(), "code" | "code_block" | "patch" | "diff") => {
                let code = part
                    .get("code")
                    .or_else(|| part.get("text"))
                    .or_else(|| part.get("content"))
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned);
                if let Some(code) = code {
                    let language = part
                        .get("language")
                        .or_else(|| part.get("lang"))
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(ToOwned::to_owned);
                    output.push(ChatMessagePart::Code { language, code });
                }
            }
            _ if part_type == "error" => {
                let text = part
                    .get("text")
                    .or_else(|| part.get("message"))
                    .or_else(|| part.get("content"))
                    .map(pretty_json)
                    .unwrap_or_else(|| String::from("unknown error"));
                output.push(ChatMessagePart::Error(text));
            }
            _ => {
                if let Some(text) = part
                    .get("text")
                    .or_else(|| part.get("content"))
                    .or_else(|| part.get("output"))
                    .and_then(Value::as_str)
                {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        output.extend(split_text_and_code_parts(trimmed));
                    }
                }
            }
        }
    }

    output
}

fn split_text_and_code_parts(value: &str) -> Vec<ChatMessagePart> {
    let mut parts = Vec::new();
    let mut prose = String::new();
    let mut in_code_fence = false;
    let mut fence_language: Option<String> = None;
    let mut code = String::new();

    for line in value.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            let language = trimmed.trim_start_matches("```").trim().to_string();
            if in_code_fence {
                let finalized = code.trim_end().to_owned();
                if !finalized.is_empty() {
                    parts.push(ChatMessagePart::Code {
                        language: fence_language.clone(),
                        code: finalized,
                    });
                }
                in_code_fence = false;
                code.clear();
                fence_language = None;
            } else {
                push_text_or_markdown_part(&mut parts, &prose);
                prose.clear();
                in_code_fence = true;
                fence_language = if language.is_empty() {
                    None
                } else {
                    Some(language)
                };
            }
            continue;
        }

        if in_code_fence {
            if !code.is_empty() {
                code.push('\n');
            }
            code.push_str(line);
        } else {
            if !prose.is_empty() {
                prose.push('\n');
            }
            prose.push_str(line);
        }
    }

    if in_code_fence {
        let finalized = code.trim_end().to_owned();
        if !finalized.is_empty() {
            parts.push(ChatMessagePart::Code {
                language: fence_language,
                code: finalized,
            });
        }
    }

    push_text_or_markdown_part(&mut parts, &prose);
    parts
}

fn push_text_or_markdown_part(parts: &mut Vec<ChatMessagePart>, text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }
    if looks_like_markdown(trimmed) {
        parts.push(ChatMessagePart::Markdown(trimmed.to_owned()));
    } else {
        parts.push(ChatMessagePart::Text(trimmed.to_owned()));
    }
}

fn looks_like_markdown(value: &str) -> bool {
    value.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with('#')
            || trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || trimmed.starts_with("1. ")
            || trimmed.starts_with("```")
            || trimmed.contains("**")
            || trimmed.contains('`')
    })
}

fn flatten_message_parts(parts: &[ChatMessagePart]) -> String {
    let mut chunks = Vec::new();
    for part in parts {
        match part {
            ChatMessagePart::Text(value)
            | ChatMessagePart::Markdown(value)
            | ChatMessagePart::Thinking(value)
            | ChatMessagePart::ShellCommand(value)
            | ChatMessagePart::Error(value) => {
                chunks.push(value.trim().to_owned());
            }
            ChatMessagePart::Code { code, .. } => {
                chunks.push(code.trim().to_owned());
            }
            ChatMessagePart::ToolCall {
                name,
                input,
                output,
            } => {
                chunks.push(format!("[tool:{name}]"));
                if let Some(value) = input {
                    chunks.push(value.trim().to_owned());
                }
                if let Some(value) = output {
                    chunks.push(value.trim().to_owned());
                }
            }
            ChatMessagePart::ShellOutput { output, .. } => {
                chunks.push(output.trim().to_owned());
            }
        }
    }

    chunks
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn pretty_json(value: &Value) -> String {
    if let Some(text) = value.as_str() {
        return text.trim().to_owned();
    }
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
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
