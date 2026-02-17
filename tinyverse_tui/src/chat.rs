use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_MODELS: [&str; 6] = [
    "openai/gpt-5.3-codex",
    "openai/gpt-5.3",
    "anthropic/claude-sonnet-4",
    "anthropic/claude-opus-4",
    "google/gemini-2.5-pro",
    "xai/grok-code-fast-1",
];

const DEFAULT_AGENTS: [&str; 8] = [
    "general",
    "developer_senior",
    "developer_jr",
    "explore",
    "planner",
    "gitter",
    "tsc-fixer",
    "reflector",
];

const DEFAULT_SLASH_COMMANDS: [(&str, &str); 8] = [
    ("help", "show local command help"),
    ("refresh", "refresh session index"),
    ("new", "open spawn dialog"),
    ("clear", "clear local chat log"),
    ("sessions", "print loaded session count"),
    ("agent", "set active agent"),
    ("model", "set active model"),
    ("grep", "search workspace with rg"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageRole {
    System,
    User,
    Assistant,
}

impl ChatMessageRole {
    pub fn label(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }

    pub fn from_wire(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "system" => Self::System,
            _ => Self::Assistant,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: Option<String>,
    pub role: ChatMessageRole,
    pub text: String,
    pub parts: Vec<ChatMessagePart>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub enum ChatMessagePart {
    Text(String),
    Markdown(String),
    Thinking(String),
    Code {
        language: Option<String>,
        code: String,
    },
    ToolCall {
        name: String,
        input: Option<String>,
        output: Option<String>,
    },
    ShellCommand(String),
    ShellOutput {
        output: String,
        exit_code: Option<i64>,
    },
    Error(String),
}

impl ChatMessagePart {
    pub fn preview_line(&self) -> Option<&str> {
        match self {
            Self::Text(value)
            | Self::Markdown(value)
            | Self::Thinking(value)
            | Self::ShellCommand(value)
            | Self::Error(value) => first_nonempty_line(value),
            Self::Code { code, .. } => first_nonempty_line(code),
            Self::ToolCall {
                name,
                input,
                output,
            } => {
                if let Some(value) = input.as_deref().and_then(first_nonempty_line) {
                    return Some(value);
                }
                if let Some(value) = output.as_deref().and_then(first_nonempty_line) {
                    return Some(value);
                }
                Some(name)
            }
            Self::ShellOutput { output, .. } => first_nonempty_line(output),
        }
    }
}

impl ChatMessage {
    pub fn preview_line(&self) -> String {
        if let Some(line) = self
            .parts
            .iter()
            .find_map(ChatMessagePart::preview_line)
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            return line.to_owned();
        }

        self.text
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or(self.text.as_str())
            .trim()
            .to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct ComposerAutocompleteItem {
    pub label: String,
    pub insert: String,
    pub tag: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerAutocompleteMode {
    Slash,
    File,
}

#[derive(Debug, Clone, Default)]
pub struct ComposerAutocomplete {
    pub open: bool,
    pub mode: Option<ComposerAutocompleteMode>,
    pub query: String,
    pub selected: usize,
    pub token_start: usize,
    pub items: Vec<ComposerAutocompleteItem>,
}

#[derive(Debug, Clone, Default)]
pub struct ItemSelector {
    pub open: bool,
    pub query: String,
    pub selected: usize,
    pub raw_mode: bool,
    pub raw_input: String,
    pub anchor_col: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub messages: Vec<ChatMessage>,
    pub draft: String,
    pub draft_cursor: usize,
    pub composing: bool,
    pub scroll_lines: u16,
    pub models: Vec<String>,
    pub agents: Vec<String>,
    pub active_model: String,
    pub active_agent: String,
    pub model_selector: ItemSelector,
    pub agent_selector: ItemSelector,
    pub autocomplete: ComposerAutocomplete,
    workspace_root: PathBuf,
    workspace_file_cache: Vec<String>,
    workspace_file_cache_loaded: bool,
}

impl Default for ChatState {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatState {
    pub fn new() -> Self {
        let models = DEFAULT_MODELS
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>();
        let agents = DEFAULT_AGENTS
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>();

        let active_model = models
            .first()
            .cloned()
            .unwrap_or_else(|| String::from("openai/gpt-5.3-codex"));
        let active_agent = agents
            .first()
            .cloned()
            .unwrap_or_else(|| String::from("general"));

        Self {
            messages: Vec::new(),
            draft: String::new(),
            draft_cursor: 0,
            composing: false,
            scroll_lines: 0,
            models,
            agents,
            active_model,
            active_agent,
            model_selector: ItemSelector::default(),
            agent_selector: ItemSelector::default(),
            autocomplete: ComposerAutocomplete::default(),
            workspace_root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            workspace_file_cache: Vec::new(),
            workspace_file_cache_loaded: false,
        }
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn is_model_selector_open(&self) -> bool {
        self.model_selector.open
    }

    pub fn is_agent_selector_open(&self) -> bool {
        self.agent_selector.open
    }

    pub fn is_autocomplete_open(&self) -> bool {
        self.autocomplete.open
    }

    pub fn autocomplete_mode(&self) -> Option<ComposerAutocompleteMode> {
        self.autocomplete.mode
    }

    pub fn open_composer(&mut self) {
        self.composing = true;
        self.draft_cursor = self.draft.chars().count();
        self.refresh_autocomplete();
    }

    pub fn cancel_composer(&mut self) {
        self.composing = false;
        self.close_autocomplete();
    }

    pub fn clear_after_send(&mut self) {
        self.draft.clear();
        self.draft_cursor = 0;
        self.composing = false;
        self.close_autocomplete();
        self.scroll_lines = 0;
    }

    pub fn take_prompt(&self) -> Option<String> {
        if !self.composing {
            return None;
        }
        let trimmed = self.draft.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.to_owned())
    }

    pub fn insert_char(&mut self, value: char) {
        if !self.composing {
            return;
        }
        insert_char_at_cursor(&mut self.draft, self.draft_cursor, value);
        self.draft_cursor = self.draft_cursor.saturating_add(1);
        self.refresh_autocomplete();
    }

    pub fn backspace_char(&mut self) {
        if !self.composing || self.draft_cursor == 0 {
            return;
        }

        self.draft_cursor = self.draft_cursor.saturating_sub(1);
        remove_char_at_cursor(&mut self.draft, self.draft_cursor);
        self.refresh_autocomplete();
    }

    pub fn delete_char(&mut self) {
        if !self.composing {
            return;
        }

        remove_char_at_cursor(&mut self.draft, self.draft_cursor);
        self.refresh_autocomplete();
    }

    pub fn move_cursor_left(&mut self) {
        if !self.composing {
            return;
        }
        self.draft_cursor = self.draft_cursor.saturating_sub(1);
        self.refresh_autocomplete();
    }

    pub fn move_cursor_right(&mut self) {
        if !self.composing {
            return;
        }
        let len = self.draft.chars().count();
        self.draft_cursor = (self.draft_cursor + 1).min(len);
        self.refresh_autocomplete();
    }

    pub fn move_cursor_home(&mut self) {
        if !self.composing {
            return;
        }
        self.draft_cursor = 0;
        self.refresh_autocomplete();
    }

    pub fn move_cursor_end(&mut self) {
        if !self.composing {
            return;
        }
        self.draft_cursor = self.draft.chars().count();
        self.refresh_autocomplete();
    }

    pub fn clear_draft(&mut self) {
        if !self.composing {
            return;
        }
        self.draft.clear();
        self.draft_cursor = 0;
        self.refresh_autocomplete();
    }

    pub fn draft_with_cursor(&self) -> String {
        with_cursor_tail(&self.draft, self.draft_cursor)
    }

    pub fn push_message(&mut self, role: ChatMessageRole, text: impl Into<String>) {
        let text = text.into();
        if text.trim().is_empty() {
            return;
        }
        self.messages.push(ChatMessage {
            id: None,
            role,
            text: text.clone(),
            parts: vec![ChatMessagePart::Text(text)],
            created_at: now_label(),
        });
        self.scroll_lines = 0;
    }

    pub fn set_messages(&mut self, messages: Vec<ChatMessage>) {
        self.messages = messages;
        self.scroll_lines = 0;
    }

    pub fn set_models(&mut self, mut models: Vec<String>) {
        models.retain(|value| !value.trim().is_empty());
        models.sort();
        models.dedup();
        if models.is_empty() {
            return;
        }

        self.models = models;
        if !self.models.iter().any(|value| value == &self.active_model)
            && let Some(first) = self.models.first().cloned()
        {
            self.active_model = first;
        }
    }

    pub fn set_agents(&mut self, mut agents: Vec<String>) {
        agents.retain(|value| !value.trim().is_empty());
        agents.sort();
        agents.dedup();
        if agents.is_empty() {
            return;
        }

        self.agents = agents;
        if !self.agents.iter().any(|value| value == &self.active_agent)
            && let Some(first) = self.agents.first().cloned()
        {
            self.active_agent = first;
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.scroll_lines = 0;
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_lines = self.scroll_lines.saturating_add(amount.max(1));
    }

    pub fn scroll_down(&mut self, amount: u16) {
        self.scroll_lines = self.scroll_lines.saturating_sub(amount.max(1));
    }

    pub fn open_model_selector(&mut self) {
        self.model_selector.open = true;
        self.model_selector.query.clear();
        self.model_selector.selected = 0;
        self.model_selector.raw_mode = false;
        self.model_selector.raw_input = self.active_model.clone();
        self.model_selector.anchor_col = None;
        self.agent_selector.open = false;
    }

    pub fn open_model_selector_at(&mut self, anchor_col: u16) {
        self.open_model_selector();
        self.model_selector.anchor_col = Some(anchor_col);
    }

    pub fn close_model_selector(&mut self) {
        self.model_selector = ItemSelector::default();
    }

    pub fn open_agent_selector(&mut self) {
        self.agent_selector.open = true;
        self.agent_selector.query.clear();
        self.agent_selector.selected = 0;
        self.agent_selector.raw_mode = false;
        self.agent_selector.raw_input.clear();
        self.agent_selector.anchor_col = None;
        self.model_selector.open = false;
    }

    pub fn open_agent_selector_at(&mut self, anchor_col: u16) {
        self.open_agent_selector();
        self.agent_selector.anchor_col = Some(anchor_col);
    }

    pub fn close_agent_selector(&mut self) {
        self.agent_selector = ItemSelector::default();
    }

    pub fn model_selector_items(&self) -> Vec<String> {
        filter_items(&self.models, &self.model_selector.query)
    }

    pub fn agent_selector_items(&self) -> Vec<String> {
        filter_items(&self.agents, &self.agent_selector.query)
    }

    pub fn model_selector_move_up(&mut self) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector.selected = 0;
            return;
        }
        self.model_selector.selected = previous_index(self.model_selector.selected, len);
    }

    pub fn model_selector_move_down(&mut self) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector.selected = 0;
            return;
        }
        self.model_selector.selected = next_index(self.model_selector.selected, len);
    }

    pub fn model_selector_set_selected(&mut self, index: usize) {
        let len = self.model_selector_items().len();
        if len == 0 {
            self.model_selector.selected = 0;
            return;
        }
        self.model_selector.selected = index.min(len.saturating_sub(1));
    }

    pub fn model_selector_insert_char(&mut self, value: char) {
        if self.model_selector.raw_mode {
            self.model_selector.raw_input.push(value);
            return;
        }
        self.model_selector.query.push(value);
        self.model_selector.selected = 0;
    }

    pub fn model_selector_backspace(&mut self) {
        if self.model_selector.raw_mode {
            self.model_selector.raw_input.pop();
            return;
        }
        self.model_selector.query.pop();
        self.model_selector.selected = 0;
    }

    pub fn model_selector_clear(&mut self) {
        if self.model_selector.raw_mode {
            self.model_selector.raw_input.clear();
            return;
        }
        self.model_selector.query.clear();
        self.model_selector.selected = 0;
    }

    pub fn model_selector_toggle_raw_mode(&mut self) {
        self.model_selector.raw_mode = !self.model_selector.raw_mode;
        if self.model_selector.raw_mode && self.model_selector.raw_input.trim().is_empty() {
            self.model_selector.raw_input = self.active_model.clone();
        }
    }

    pub fn confirm_model_selector(&mut self) -> Option<String> {
        if self.model_selector.raw_mode {
            let value = self.model_selector.raw_input.trim();
            if value.is_empty() {
                return None;
            }
            let selected = value.to_owned();
            self.set_active_model(&selected);
            self.close_model_selector();
            return Some(selected);
        }

        let items = self.model_selector_items();
        let selected = items.get(self.model_selector.selected)?.clone();
        self.set_active_model(&selected);
        self.close_model_selector();
        Some(selected)
    }

    pub fn agent_selector_move_up(&mut self) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector.selected = 0;
            return;
        }
        self.agent_selector.selected = previous_index(self.agent_selector.selected, len);
    }

    pub fn agent_selector_move_down(&mut self) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector.selected = 0;
            return;
        }
        self.agent_selector.selected = next_index(self.agent_selector.selected, len);
    }

    pub fn agent_selector_set_selected(&mut self, index: usize) {
        let len = self.agent_selector_items().len();
        if len == 0 {
            self.agent_selector.selected = 0;
            return;
        }
        self.agent_selector.selected = index.min(len.saturating_sub(1));
    }

    pub fn agent_selector_insert_char(&mut self, value: char) {
        self.agent_selector.query.push(value);
        self.agent_selector.selected = 0;
    }

    pub fn agent_selector_backspace(&mut self) {
        self.agent_selector.query.pop();
        self.agent_selector.selected = 0;
    }

    pub fn agent_selector_clear(&mut self) {
        self.agent_selector.query.clear();
        self.agent_selector.selected = 0;
    }

    pub fn confirm_agent_selector(&mut self) -> Option<String> {
        let items = self.agent_selector_items();
        let selected = items.get(self.agent_selector.selected)?.clone();
        self.set_active_agent(&selected)?;
        self.close_agent_selector();
        Some(selected)
    }

    pub fn set_active_model(&mut self, value: &str) {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return;
        }

        if !self.models.iter().any(|model| model == trimmed) {
            self.models.push(trimmed.to_owned());
            self.models.sort();
            self.models.dedup();
        }

        self.active_model = trimmed.to_owned();
    }

    pub fn set_active_agent(&mut self, value: &str) -> Option<()> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return None;
        }

        if !self.agents.iter().any(|agent| agent == trimmed) {
            return None;
        }

        self.active_agent = trimmed.to_owned();
        Some(())
    }

    pub fn close_autocomplete(&mut self) {
        self.autocomplete = ComposerAutocomplete::default();
    }

    pub fn autocomplete_move_up(&mut self) {
        let len = self.autocomplete.items.len();
        if len == 0 {
            self.autocomplete.selected = 0;
            return;
        }
        self.autocomplete.selected = previous_index(self.autocomplete.selected, len);
    }

    pub fn autocomplete_move_down(&mut self) {
        let len = self.autocomplete.items.len();
        if len == 0 {
            self.autocomplete.selected = 0;
            return;
        }
        self.autocomplete.selected = next_index(self.autocomplete.selected, len);
    }

    pub fn autocomplete_set_selected(&mut self, index: usize) {
        let len = self.autocomplete.items.len();
        if len == 0 {
            self.autocomplete.selected = 0;
            return;
        }

        self.autocomplete.selected = index.min(len.saturating_sub(1));
    }

    pub fn autocomplete_anchor_position(&self) -> Option<(usize, usize)> {
        if !self.autocomplete.open {
            return None;
        }

        Some(row_col_from_cursor_index(
            &self.draft,
            self.autocomplete
                .token_start
                .min(self.draft_cursor.max(self.autocomplete.token_start)),
        ))
    }

    pub fn apply_autocomplete_selection(&mut self) -> Option<String> {
        let index = self
            .autocomplete
            .selected
            .min(self.autocomplete.items.len().saturating_sub(1));
        let selected = self.autocomplete.items.get(index)?.clone();

        let mut chars = self.draft.chars().collect::<Vec<_>>();
        let start = self.autocomplete.token_start.min(chars.len());
        let end = self.draft_cursor.min(chars.len());
        chars.splice(start..end, selected.insert.chars());
        self.draft = chars.into_iter().collect();
        self.draft_cursor = start + selected.insert.chars().count();
        self.close_autocomplete();
        Some(selected.label)
    }

    pub fn refresh_autocomplete(&mut self) {
        if !self.composing {
            self.close_autocomplete();
            return;
        }

        let Some((trigger, query, token_start)) =
            current_composer_trigger(&self.draft, self.draft_cursor)
        else {
            self.close_autocomplete();
            return;
        };

        let items = match trigger {
            '/' => slash_autocomplete_items(&query),
            '@' => {
                self.ensure_workspace_file_cache();
                file_autocomplete_items(&self.workspace_file_cache, &query)
            }
            _ => Vec::new(),
        };

        if items.is_empty() {
            self.close_autocomplete();
            return;
        }

        self.autocomplete.open = true;
        self.autocomplete.mode = match trigger {
            '/' => Some(ComposerAutocompleteMode::Slash),
            '@' => Some(ComposerAutocompleteMode::File),
            _ => None,
        };
        self.autocomplete.query = query;
        self.autocomplete.token_start = token_start;
        self.autocomplete.items = items;
        self.autocomplete.selected = self
            .autocomplete
            .selected
            .min(self.autocomplete.items.len().saturating_sub(1));
    }

    fn ensure_workspace_file_cache(&mut self) {
        if self.workspace_file_cache_loaded {
            return;
        }

        self.workspace_file_cache = collect_workspace_files(&self.workspace_root, 2000, 6);
        self.workspace_file_cache_loaded = true;
    }
}

fn filter_items(items: &[String], query: &str) -> Vec<String> {
    let needle = query.trim().to_ascii_lowercase();
    let mut output = items.to_owned();
    if needle.is_empty() {
        return output;
    }

    output.retain(|item| item.to_ascii_lowercase().contains(&needle));
    output
}

fn next_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (current + 1) % len
}

fn previous_index(current: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if current == 0 { len - 1 } else { current - 1 }
}

fn with_cursor_tail(value: &str, cursor: usize) -> String {
    let mut out = String::new();
    let mut inserted = false;

    for (index, ch) in value.chars().enumerate() {
        if !inserted && index == cursor {
            out.push('_');
            inserted = true;
        }
        out.push(ch);
    }

    if !inserted {
        out.push('_');
    }

    out
}

fn first_nonempty_line(value: &str) -> Option<&str> {
    value.lines().find(|line| !line.trim().is_empty())
}

fn current_composer_trigger(value: &str, cursor_index: usize) -> Option<(char, String, usize)> {
    let mut cursor_byte = value.len();
    if cursor_index < value.chars().count() {
        cursor_byte = value
            .char_indices()
            .nth(cursor_index)
            .map(|(byte, _)| byte)
            .unwrap_or(value.len());
    }

    let prefix = &value[..cursor_byte];
    let token_start_byte = prefix
        .rfind(|ch: char| ch.is_whitespace())
        .map(|index| index + 1)
        .unwrap_or(0);
    let token = &prefix[token_start_byte..];

    let (trigger, query) = if let Some(query) = token.strip_prefix('/') {
        ('/', query)
    } else if let Some(query) = token.strip_prefix('@') {
        ('@', query)
    } else {
        return None;
    };

    let token_start = value[..token_start_byte].chars().count();
    Some((trigger, query.to_string(), token_start))
}

fn slash_autocomplete_items(query: &str) -> Vec<ComposerAutocompleteItem> {
    let needle = query.trim().to_ascii_lowercase();
    let mut items = DEFAULT_SLASH_COMMANDS
        .into_iter()
        .filter(|(name, _)| needle.is_empty() || name.contains(&needle))
        .map(|(name, desc)| ComposerAutocompleteItem {
            label: format!("/{name}"),
            insert: if matches!(name, "agent" | "model" | "grep") {
                format!("/{name} ")
            } else {
                format!("/{name}")
            },
            tag: desc.to_string(),
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| left.label.cmp(&right.label));
    items
}

fn file_autocomplete_items(paths: &[String], query: &str) -> Vec<ComposerAutocompleteItem> {
    let needle = query.trim().to_ascii_lowercase();
    paths
        .iter()
        .filter(|path| needle.is_empty() || path.to_ascii_lowercase().contains(&needle))
        .take(80)
        .map(|path| ComposerAutocompleteItem {
            label: format!("@{path}"),
            insert: format!("@{path}"),
            tag: String::from("file"),
        })
        .collect()
}

fn collect_workspace_files(root: &Path, limit: usize, max_depth: usize) -> Vec<String> {
    let mut output = Vec::new();
    collect_workspace_files_recursive(root, root, 0, max_depth, limit, &mut output);
    output.sort();
    output
}

fn collect_workspace_files_recursive(
    root: &Path,
    current: &Path,
    depth: usize,
    max_depth: usize,
    limit: usize,
    output: &mut Vec<String>,
) {
    if output.len() >= limit || depth > max_depth {
        return;
    }

    let Ok(entries) = fs::read_dir(current) else {
        return;
    };

    for entry in entries.flatten() {
        if output.len() >= limit {
            break;
        }

        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if name.starts_with('.') || matches!(name.as_ref(), "target" | "node_modules" | "generated")
        {
            continue;
        }

        if path.is_dir() {
            collect_workspace_files_recursive(root, &path, depth + 1, max_depth, limit, output);
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if let Ok(relative) = path.strip_prefix(root) {
            output.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn row_col_from_cursor_index(value: &str, cursor_index: usize) -> (usize, usize) {
    let mut row = 0usize;
    let mut col = 0usize;

    for (index, ch) in value.chars().enumerate() {
        if index == cursor_index {
            break;
        }
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (row, col)
}

fn insert_char_at_cursor(buffer: &mut String, cursor: usize, value: char) {
    let mut chars = buffer.chars().collect::<Vec<_>>();
    let index = cursor.min(chars.len());
    chars.insert(index, value);
    *buffer = chars.into_iter().collect();
}

fn remove_char_at_cursor(buffer: &mut String, cursor: usize) {
    let mut chars = buffer.chars().collect::<Vec<_>>();
    if cursor >= chars.len() {
        return;
    }

    chars.remove(cursor);
    *buffer = chars.into_iter().collect();
}

fn now_label() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}
