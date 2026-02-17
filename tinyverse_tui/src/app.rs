use std::collections::HashMap;
use std::time::Instant;

use anyhow::Result;
use ratatui::layout::Rect;
use tinyverse_lib::{SessionStore, StoredSession};

use crate::TuiRunOptions;
use crate::chat::ChatState;
use crate::chat_bridge::ChatBridge;
use crate::theme::UiTheme;

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
    SidebarTab,
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
            Self::SidebarTab => "1-3/lr",
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
            Self::SidebarTab => "tabs",
            Self::FormNextField => "next field",
            Self::FormSubmit => "submit",
            Self::FormCancel => "cancel",
            Self::FormEditPrompt => "edit prompt",
            Self::Confirm => "confirm",
            Self::Cancel => "cancel",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarTab {
    Console,
    Agent,
    Chat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerDrag {
    Vertical,
    Horizontal,
}

impl SidebarTab {
    pub fn title(self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Agent => "Agent",
            Self::Chat => "Chat",
        }
    }

    pub fn all() -> [Self; 3] {
        [Self::Console, Self::Agent, Self::Chat]
    }

    pub fn hotkey_index(self) -> usize {
        match self {
            Self::Console => 1,
            Self::Agent => 2,
            Self::Chat => 3,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Console => Self::Agent,
            Self::Agent => Self::Chat,
            Self::Chat => Self::Console,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Console => Self::Chat,
            Self::Agent => Self::Console,
            Self::Chat => Self::Agent,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct OverlayLayoutCache {
    pub dialog_rect: Option<Rect>,
    pub field_rects: Vec<Rect>,
    pub prompt_editor_rect: Option<Rect>,
}

#[derive(Debug, Default, Clone)]
pub struct ChatLayoutCache {
    pub root_rect: Option<Rect>,
    pub messages_rect: Option<Rect>,
    pub composer_rect: Option<Rect>,
    pub composer_input_rect: Option<Rect>,
    pub model_chip_rect: Option<Rect>,
    pub agent_chip_rect: Option<Rect>,
    pub model_selector_rect: Option<Rect>,
    pub model_selector_list_rect: Option<Rect>,
    pub model_selector_query_rect: Option<Rect>,
    pub model_selector_list_start: usize,
    pub agent_selector_rect: Option<Rect>,
    pub agent_selector_list_rect: Option<Rect>,
    pub agent_selector_query_rect: Option<Rect>,
    pub agent_selector_list_start: usize,
    pub autocomplete_rect: Option<Rect>,
    pub autocomplete_list_rect: Option<Rect>,
    pub autocomplete_list_start: usize,
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

    pub fn hotkey(self) -> char {
        match self {
            Self::SpawnSession => 's',
            Self::Refresh => 'r',
            Self::ToggleInspector => 'i',
            Self::AttachSession => 'a',
            Self::SendToConsole => 'c',
            Self::KillSession => 'x',
            Self::KillAllSessions => 'k',
            Self::CloseMenu => 'q',
        }
    }

    pub fn from_hotkey(key: char) -> Option<Self> {
        let normalized = key.to_ascii_lowercase();
        MENU_ACTIONS
            .iter()
            .copied()
            .find(|action| action.hotkey() == normalized)
    }
}

pub const MENU_ACTIONS: [MenuAction; 8] = [
    MenuAction::SpawnSession,
    MenuAction::Refresh,
    MenuAction::ToggleInspector,
    MenuAction::AttachSession,
    MenuAction::SendToConsole,
    MenuAction::KillSession,
    MenuAction::KillAllSessions,
    MenuAction::CloseMenu,
];

pub const ACTION_MENU_DANGER_SPLIT_AFTER: usize = 4;

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
    pub card_kill_rects: Vec<(usize, Rect)>,
    pub body_rect: Option<Rect>,
    pub divider_x: Option<u16>,
    pub divider_y: Option<u16>,
    pub action_menu_rect: Option<Rect>,
    pub confirm_rect: Option<Rect>,
    pub footer_rect: Option<Rect>,
    pub sidebar_tab_rects: Vec<(SidebarTab, Rect)>,
    pub sidebar_preview_rect: Option<Rect>,
    pub overlay: OverlayLayoutCache,
    pub chat: ChatLayoutCache,
}

pub struct App {
    pub options: TuiRunOptions,
    pub theme: UiTheme,
    pub sessions: Vec<StoredSession>,
    pub selected_index: usize,
    pub scroll_row: usize,
    pub inspector_visible: bool,
    pub inspector_ratio: u16,
    pub inspector_height: u16,
    pub dragging_divider: Option<DividerDrag>,
    pub mode: AppMode,
    pub action_menu_index: usize,
    pub action_menu_anchor: Option<(u16, u16)>,
    pub input_buffer: String,
    pub spawn_form: SpawnForm,
    pub sidebar_tab: SidebarTab,
    pub chat: ChatState,
    pub chat_bridge: ChatBridge,
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
            theme: UiTheme::default(),
            sessions: Vec::new(),
            selected_index: 0,
            scroll_row: 0,
            inspector_visible: true,
            inspector_ratio: 58,
            inspector_height: 8,
            dragging_divider: None,
            mode: AppMode::Normal,
            action_menu_index: 0,
            action_menu_anchor: None,
            input_buffer: String::new(),
            spawn_form: SpawnForm::default(),
            sidebar_tab: SidebarTab::Console,
            chat: ChatState::default(),
            chat_bridge: ChatBridge::from_env(),
            pane_preview_cache: HashMap::new(),
            footer_hover_action: None,
            should_quit: false,
            status_message: String::new(),
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

    pub fn next_sidebar_tab(&mut self) {
        self.sidebar_tab = self.sidebar_tab.next();
        if self.sidebar_tab == SidebarTab::Chat {
            self.chat_bridge.sync_now(&mut self.chat);
        }
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
    }

    pub fn prev_sidebar_tab(&mut self) {
        self.sidebar_tab = self.sidebar_tab.prev();
        if self.sidebar_tab == SidebarTab::Chat {
            self.chat_bridge.sync_now(&mut self.chat);
        }
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
    }

    pub fn set_sidebar_tab(&mut self, tab: SidebarTab) {
        self.sidebar_tab = tab;
        if self.sidebar_tab == SidebarTab::Chat {
            self.chat_bridge.sync_now(&mut self.chat);
        }
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
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
