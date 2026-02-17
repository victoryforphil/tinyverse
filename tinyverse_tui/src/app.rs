use std::collections::HashMap;
use std::time::Instant;

use anyhow::Result;
use ratatui::layout::Rect;
use tinyverse_lib::{SessionStore, StoredSession};

use crate::TuiRunOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    ActionMenu,
    ConfirmKill,
    ConfirmKillAll,
    SendInput,
    SpawnInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Refresh,
    ToggleInspector,
    AttachSession,
    SendToConsole,
    SpawnSession,
    KillSession,
    KillAllSessions,
    CloseMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FooterHotkeyAction {
    Quit,
    Navigate,
    Refresh,
    ToggleInspector,
    OpenActions,
    Attach,
    Spawn,
    Kill,
    FormNextField,
    FormSubmit,
    FormCancel,
    FormEditPrompt,
    Confirm,
    Cancel,
}

impl FooterHotkeyAction {
    pub fn key(self) -> &'static str {
        match self {
            Self::Quit => "q",
            Self::Navigate => "arrows/hjkl",
            Self::Refresh => "r",
            Self::ToggleInspector => "i",
            Self::OpenActions => "enter",
            Self::Attach => "a",
            Self::Spawn => "s",
            Self::Kill => "x",
            Self::FormNextField => "tab",
            Self::FormSubmit => "enter",
            Self::FormCancel => "esc",
            Self::FormEditPrompt => "e",
            Self::Confirm => "y/enter",
            Self::Cancel => "n/esc",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::Navigate => "navigate",
            Self::Refresh => "refresh",
            Self::ToggleInspector => "inspector",
            Self::OpenActions => "actions",
            Self::Attach => "attach",
            Self::Spawn => "spawn",
            Self::Kill => "kill",
            Self::FormNextField => "next field",
            Self::FormSubmit => "submit",
            Self::FormCancel => "cancel",
            Self::FormEditPrompt => "edit prompt",
            Self::Confirm => "confirm",
            Self::Cancel => "cancel",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct OverlayLayoutCache {
    pub dialog_rect: Option<Rect>,
    pub field_rects: Vec<Rect>,
    pub prompt_editor_rect: Option<Rect>,
}

#[derive(Debug, Clone)]
pub struct PanePreview {
    pub console: String,
    pub agent: String,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::Refresh => "Refresh sessions",
            Self::ToggleInspector => "Toggle inspector",
            Self::AttachSession => "Attach selected session",
            Self::SendToConsole => "Send command to console pane",
            Self::SpawnSession => "Spawn new session",
            Self::KillSession => "Kill selected session",
            Self::KillAllSessions => "Kill all sessions",
            Self::CloseMenu => "Close menu",
        }
    }
}

pub const MENU_ACTIONS: [MenuAction; 8] = [
    MenuAction::Refresh,
    MenuAction::ToggleInspector,
    MenuAction::AttachSession,
    MenuAction::SendToConsole,
    MenuAction::SpawnSession,
    MenuAction::KillSession,
    MenuAction::KillAllSessions,
    MenuAction::CloseMenu,
];

#[derive(Debug, Clone)]
pub struct SpawnForm {
    pub session_name: String,
    pub agent_type: String,
    pub model: String,
    pub prompt: String,
    pub active_field: usize,
}

impl Default for SpawnForm {
    fn default() -> Self {
        Self {
            session_name: String::new(),
            agent_type: String::from("opencode"),
            model: String::new(),
            prompt: String::new(),
            active_field: 0,
        }
    }
}

impl SpawnForm {
    pub fn next_field(&mut self) {
        self.active_field = (self.active_field + 1) % 4;
    }

    pub fn prev_field(&mut self) {
        self.active_field = if self.active_field == 0 {
            3
        } else {
            self.active_field - 1
        };
    }

    pub fn active_field_mut(&mut self) -> &mut String {
        match self.active_field {
            0 => &mut self.session_name,
            1 => &mut self.agent_type,
            2 => &mut self.model,
            _ => &mut self.prompt,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LayoutCache {
    pub card_rects: Vec<(usize, Rect)>,
    pub body_rect: Option<Rect>,
    pub divider_x: Option<u16>,
    pub action_menu_rect: Option<Rect>,
    pub confirm_rect: Option<Rect>,
    pub footer_rect: Option<Rect>,
    pub overlay: OverlayLayoutCache,
}

pub struct App {
    pub options: TuiRunOptions,
    pub sessions: Vec<StoredSession>,
    pub selected_index: usize,
    pub scroll_row: usize,
    pub inspector_visible: bool,
    pub inspector_ratio: u16,
    pub dragging_divider: bool,
    pub mode: AppMode,
    pub action_menu_index: usize,
    pub action_menu_anchor: Option<(u16, u16)>,
    pub input_buffer: String,
    pub spawn_form: SpawnForm,
    pub pane_preview_cache: HashMap<String, PanePreview>,
    pub footer_hover_action: Option<FooterHotkeyAction>,
    pub should_quit: bool,
    pub status_message: String,
    pub last_refresh_at: Option<Instant>,
    pub layout: LayoutCache,
}

impl App {
    pub fn new(options: TuiRunOptions) -> Self {
        Self {
            options,
            sessions: Vec::new(),
            selected_index: 0,
            scroll_row: 0,
            inspector_visible: true,
            inspector_ratio: 68,
            dragging_divider: false,
            mode: AppMode::Normal,
            action_menu_index: 0,
            action_menu_anchor: None,
            input_buffer: String::new(),
            spawn_form: SpawnForm::default(),
            pane_preview_cache: HashMap::new(),
            footer_hover_action: None,
            should_quit: false,
            status_message: String::from("Starting tinyverse TUI"),
            last_refresh_at: None,
            layout: LayoutCache::default(),
        }
    }

    pub fn refresh(&mut self, store: &mut SessionStore) -> Result<()> {
        store.reconcile_now()?;
        self.sessions = store.list_sessions()?;
        self.pane_preview_cache.clear();
        if self.sessions.is_empty() {
            self.selected_index = 0;
            self.scroll_row = 0;
            self.status_message = String::from("No sessions found");
        } else {
            if self.selected_index >= self.sessions.len() {
                self.selected_index = self.sessions.len() - 1;
            }
            self.status_message = format!("Loaded {} session(s)", self.sessions.len());
        }
        self.last_refresh_at = Some(Instant::now());
        Ok(())
    }

    pub fn select_next(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.sessions.len();
    }

    pub fn select_prev(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        self.selected_index = if self.selected_index == 0 {
            self.sessions.len() - 1
        } else {
            self.selected_index - 1
        };
    }

    pub fn selected_session(&self) -> Option<&StoredSession> {
        self.sessions.get(self.selected_index)
    }

    pub fn toggle_inspector(&mut self) {
        self.inspector_visible = !self.inspector_visible;
        if self.inspector_visible {
            self.status_message = String::from("Inspector opened");
        } else {
            self.status_message = String::from("Inspector closed");
        }
    }

    pub fn open_action_menu(&mut self) {
        self.mode = AppMode::ActionMenu;
        self.action_menu_index = 0;
        self.action_menu_anchor = None;
    }

    pub fn reset_spawn_form(&mut self) {
        let agent = self.spawn_form.agent_type.clone();
        let model = self.spawn_form.model.clone();
        self.spawn_form = SpawnForm::default();
        self.spawn_form.agent_type = agent;
        self.spawn_form.model = model;
    }

    pub fn close_action_menu(&mut self) {
        self.mode = AppMode::Normal;
        self.action_menu_anchor = None;
    }

    pub fn action_menu_next(&mut self) {
        self.action_menu_index = (self.action_menu_index + 1) % MENU_ACTIONS.len();
    }

    pub fn action_menu_prev(&mut self) {
        self.action_menu_index = if self.action_menu_index == 0 {
            MENU_ACTIONS.len() - 1
        } else {
            self.action_menu_index - 1
        };
    }

    pub fn selected_menu_action(&self) -> MenuAction {
        MENU_ACTIONS[self.action_menu_index]
    }
}
