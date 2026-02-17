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
    CloseMenu,
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
            Self::CloseMenu => "Close menu",
        }
    }
}

pub const MENU_ACTIONS: [MenuAction; 7] = [
    MenuAction::Refresh,
    MenuAction::ToggleInspector,
    MenuAction::AttachSession,
    MenuAction::SendToConsole,
    MenuAction::SpawnSession,
    MenuAction::KillSession,
    MenuAction::CloseMenu,
];

#[derive(Debug, Default, Clone)]
pub struct LayoutCache {
    pub card_rects: Vec<(usize, Rect)>,
    pub body_rect: Option<Rect>,
    pub divider_x: Option<u16>,
    pub action_menu_rect: Option<Rect>,
    pub confirm_rect: Option<Rect>,
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
            should_quit: false,
            status_message: String::from("Starting tinyverse TUI"),
            last_refresh_at: None,
            layout: LayoutCache::default(),
        }
    }

    pub fn refresh(&mut self, store: &mut SessionStore) -> Result<()> {
        self.sessions = store.list_sessions()?;
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
