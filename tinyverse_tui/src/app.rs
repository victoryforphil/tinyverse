use std::collections::{HashMap, HashSet};
use std::time::Instant;

use anyhow::Result;
use ratatui::layout::Rect;
use ratatui::text::Line;
use tinyverse_lib::{SessionStore, StoredSession};

use crate::chat::ChatState;
use crate::chat_bridge::{ChatBridge, ChatSessionSummary};
use crate::theme::UiTheme;
use crate::TuiRunOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    ActionMenu,
    ConfirmKill,
    ConfirmKillAll,
    SendInput,
    SpawnInput,
    PaneFocus,
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
    SessionView,
    SidebarTab,
    FormNextField,
    FormSubmit,
    FormCancel,
    FormEditPrompt,
    Confirm,
    Cancel,
    PaneFocus,
    PaneFocusExit,
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
            Self::SessionView => "v",
            Self::SidebarTab => "1-3/lr",
            Self::FormNextField => "tab",
            Self::FormSubmit => "enter",
            Self::FormCancel => "esc",
            Self::FormEditPrompt => "e",
            Self::Confirm => "y/enter",
            Self::Cancel => "n/esc",
            Self::PaneFocus => "f",
            Self::PaneFocusExit => "esc esc",
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
            Self::SessionView => "view",
            Self::SidebarTab => "tabs",
            Self::FormNextField => "next field",
            Self::FormSubmit => "submit",
            Self::FormCancel => "cancel",
            Self::FormEditPrompt => "edit prompt",
            Self::Confirm => "confirm",
            Self::Cancel => "cancel",
            Self::PaneFocus => "toggle live",
            Self::PaneFocusExit => "exit live",
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
pub enum SessionsViewMode {
    Graphical,
    Tree,
}

impl SessionsViewMode {
    pub fn title(self) -> &'static str {
        match self {
            Self::Graphical => "Graph",
            Self::Tree => "Tree",
        }
    }

    pub fn all() -> [Self; 2] {
        [Self::Graphical, Self::Tree]
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Graphical => Self::Tree,
            Self::Tree => Self::Graphical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionTreeNode {
    SessionRoot {
        session_index: usize,
    },
    SidebarPane {
        session_index: usize,
        tab: SidebarTab,
    },
    ChatSession {
        session_index: usize,
        chat_session_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct SessionTreeRow {
    pub node: SessionTreeNode,
    pub label: String,
    pub depth: usize,
    pub is_last: bool,
    pub ancestors_are_last: Vec<bool>,
}

#[derive(Debug, Clone)]
struct SessionTreeBuildNode {
    node: SessionTreeNode,
    label: String,
    children: Vec<SessionTreeBuildNode>,
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
    pub detail_modal_rect: Option<Rect>,
    pub detail_modal_body_rect: Option<Rect>,
    pub part_toggle_hitboxes: Vec<ChatPartToggleHitbox>,
}

#[derive(Debug, Clone)]
pub struct ChatPartToggleHitbox {
    pub rect: Rect,
    pub part_key: String,
}

#[derive(Debug, Clone)]
pub struct PanePreview {
    pub console: String,
    pub agent: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRenderSignature {
    pub message_count: usize,
    pub last_message_id: Option<String>,
    pub last_message_created_at: Option<String>,
    pub last_part_count: usize,
    pub collapse_verbose_parts: bool,
    pub focused_part_key: Option<String>,
}

pub type CachedChatLine = (Line<'static>, Option<String>);

#[derive(Debug, Clone)]
pub struct ChatRenderCache {
    pub width: u16,
    pub signature: ChatRenderSignature,
    pub lines: Vec<CachedChatLine>,
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
    pub sessions_view_tab_rects: Vec<(SessionsViewMode, Rect)>,
    pub sessions_header_rect: Option<Rect>,
    pub session_tree_row_rects: Vec<(usize, Rect)>,
    pub sidebar_header_rect: Option<Rect>,
    pub sidebar_tab_rects: Vec<(SidebarTab, Rect)>,
    pub sidebar_preview_rect: Option<Rect>,
    pub overlay: OverlayLayoutCache,
    pub chat: ChatLayoutCache,
}

pub struct App {
    pub options: TuiRunOptions,
    pub theme: UiTheme,
    pub repo_name: Option<String>,
    pub git_branch: Option<String>,
    pub sessions: Vec<StoredSession>,
    pub selected_index: usize,
    pub scroll_row: usize,
    pub inspector_visible: bool,
    pub inspector_ratio: u16,
    pub sessions_minimized: bool,
    pub sessions_ratio_before_minimize: u16,
    pub sidebar_minimized: bool,
    pub sidebar_ratio_before_minimize: u16,
    pub inspector_height: u16,
    pub dragging_divider: Option<DividerDrag>,
    pub mode: AppMode,
    pub action_menu_index: usize,
    pub action_menu_anchor: Option<(u16, u16)>,
    pub input_buffer: String,
    pub spawn_form: SpawnForm,
    pub sessions_view_mode: SessionsViewMode,
    pub session_tree_rows: Vec<SessionTreeRow>,
    pub session_tree_cursor: usize,
    pub session_tree_scroll: usize,
    pub sidebar_tab: SidebarTab,
    pub chat: ChatState,
    pub chat_bridge: ChatBridge,
    pub chat_hint_session_key: Option<String>,
    pub chat_hint_directory: Option<String>,
    pub chat_hint_base_url: Option<String>,
    pub chat_hint_session_id: Option<String>,
    pub chat_hint_refreshed_at: Option<Instant>,
    pub spawned_chat_session_ids: HashMap<String, String>,
    pub chat_render_cache: Option<ChatRenderCache>,
    pub pane_preview_cache: HashMap<String, PanePreview>,
    pub show_card_preview_on_all_cards: bool,
    pub footer_hover_action: Option<FooterHotkeyAction>,
    pub should_quit: bool,
    pub status_message: String,
    pub last_refresh_at: Option<Instant>,
    pub last_esc_at: Option<Instant>,
    pub layout: LayoutCache,
}

impl App {
    pub fn new(options: TuiRunOptions) -> Self {
        Self {
            options,
            theme: UiTheme::default(),
            repo_name: None,
            git_branch: None,
            sessions: Vec::new(),
            selected_index: 0,
            scroll_row: 0,
            inspector_visible: true,
            inspector_ratio: 50,
            sessions_minimized: false,
            sessions_ratio_before_minimize: 50,
            sidebar_minimized: false,
            sidebar_ratio_before_minimize: 50,
            inspector_height: 8,
            dragging_divider: None,
            mode: AppMode::Normal,
            action_menu_index: 0,
            action_menu_anchor: None,
            input_buffer: String::new(),
            spawn_form: SpawnForm::default(),
            sessions_view_mode: SessionsViewMode::Graphical,
            session_tree_rows: Vec::new(),
            session_tree_cursor: 0,
            session_tree_scroll: 0,
            sidebar_tab: SidebarTab::Console,
            chat: ChatState::default(),
            chat_bridge: ChatBridge::from_env(),
            chat_hint_session_key: None,
            chat_hint_directory: None,
            chat_hint_base_url: None,
            chat_hint_session_id: None,
            chat_hint_refreshed_at: None,
            spawned_chat_session_ids: HashMap::new(),
            chat_render_cache: None,
            pane_preview_cache: HashMap::new(),
            show_card_preview_on_all_cards: false,
            footer_hover_action: None,
            should_quit: false,
            status_message: String::new(),
            last_refresh_at: None,
            last_esc_at: None,
            layout: LayoutCache::default(),
        }
    }

    pub fn refresh(&mut self, store: &mut SessionStore) -> Result<()> {
        store.reconcile_now()?;
        self.sessions = store.list_sessions()?;
        self.chat_hint_session_key = None;
        self.chat_hint_directory = None;
        self.chat_hint_base_url = None;
        self.chat_hint_session_id = None;
        self.chat_hint_refreshed_at = None;
        self.pane_preview_cache.clear();
        if self.sessions.is_empty() {
            self.selected_index = 0;
            self.scroll_row = 0;
            self.session_tree_rows.clear();
            self.session_tree_cursor = 0;
            self.session_tree_scroll = 0;
            self.status_message = String::from("No sessions found");
        } else {
            if self.selected_index >= self.sessions.len() {
                self.selected_index = self.sessions.len() - 1;
            }
            self.status_message = format!("Loaded {} session(s)", self.sessions.len());
            if self.sessions_view_mode == SessionsViewMode::Tree {
                self.rebuild_tree_rows_preserving_cursor();
            } else {
                self.sync_tree_cursor_to_active_target();
            }
        }
        self.last_refresh_at = Some(Instant::now());
        Ok(())
    }

    pub fn select_next(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.sessions.len();
        self.sync_tree_cursor_to_active_target();
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
        self.sync_tree_cursor_to_active_target();
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
        self.sync_tree_cursor_to_active_target();
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
    }

    pub fn prev_sidebar_tab(&mut self) {
        self.sidebar_tab = self.sidebar_tab.prev();
        if self.sidebar_tab == SidebarTab::Chat {
            self.chat_bridge.sync_now(&mut self.chat);
        }
        self.sync_tree_cursor_to_active_target();
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
    }

    pub fn set_sidebar_tab(&mut self, tab: SidebarTab) {
        self.sidebar_tab = tab;
        if self.sidebar_tab == SidebarTab::Chat {
            self.chat_bridge.sync_now(&mut self.chat);
        }
        self.sync_tree_cursor_to_active_target();
        self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
    }

    pub fn toggle_sessions_view_mode(&mut self) {
        let next = self.sessions_view_mode.toggle();
        self.set_sessions_view_mode(next);
    }

    pub fn toggle_sessions_minimized(&mut self) {
        if self.sessions_minimized {
            self.sessions_minimized = false;
            self.inspector_ratio = self.sessions_ratio_before_minimize.clamp(40, 80);
            self.status_message = String::from("Sessions panel restored");
        } else {
            self.sidebar_minimized = false;
            self.sessions_ratio_before_minimize = self.inspector_ratio;
            self.sessions_minimized = true;
            self.status_message = String::from("Sessions panel minimized");
        }
    }

    pub fn toggle_sidebar_minimized(&mut self) {
        if self.sidebar_minimized {
            self.sidebar_minimized = false;
            self.inspector_ratio = self.sidebar_ratio_before_minimize.clamp(40, 80);
            self.status_message = String::from("Sidebar panel restored");
        } else {
            self.sessions_minimized = false;
            self.sidebar_ratio_before_minimize = self.inspector_ratio;
            self.sidebar_minimized = true;
            self.status_message = String::from("Sidebar panel minimized");
        }
    }

    pub fn set_sessions_view_mode(&mut self, mode: SessionsViewMode) {
        self.sessions_view_mode = mode;
        self.sync_tree_cursor_to_active_target();
        self.status_message = format!("Sessions view: {}", self.sessions_view_mode.title());
    }

    pub fn rebuild_tree_rows_preserving_cursor(&mut self) {
        let previous_node = self
            .session_tree_rows
            .get(self.session_tree_cursor)
            .map(|row| row.node.clone());
        self.session_tree_rows = self.build_session_tree_rows();
        self.session_tree_cursor = previous_node
            .as_ref()
            .and_then(|target| self.find_tree_row_index(target))
            .unwrap_or_else(|| {
                self.session_tree_cursor
                    .min(self.session_tree_rows.len().saturating_sub(1))
            });
        self.session_tree_scroll = self
            .session_tree_scroll
            .min(self.session_tree_rows.len().saturating_sub(1));
    }

    pub fn move_tree_cursor_up(&mut self) {
        if self.session_tree_rows.is_empty() {
            return;
        }
        self.session_tree_cursor = if self.session_tree_cursor == 0 {
            self.session_tree_rows.len() - 1
        } else {
            self.session_tree_cursor - 1
        };
    }

    pub fn move_tree_cursor_down(&mut self) {
        if self.session_tree_rows.is_empty() {
            return;
        }
        self.session_tree_cursor = (self.session_tree_cursor + 1) % self.session_tree_rows.len();
    }

    pub fn set_tree_cursor(&mut self, index: usize) {
        if self.session_tree_rows.is_empty() {
            self.session_tree_cursor = 0;
            return;
        }
        self.session_tree_cursor = index.min(self.session_tree_rows.len() - 1);
    }

    pub fn activate_tree_cursor(&mut self) {
        let Some(row) = self
            .session_tree_rows
            .get(self.session_tree_cursor)
            .cloned()
        else {
            return;
        };

        match row.node {
            SessionTreeNode::SessionRoot { session_index } => {
                if session_index < self.sessions.len() {
                    self.selected_index = session_index;
                    self.status_message = format!(
                        "Selected session: {}",
                        self.sessions[session_index].session_name
                    );
                    self.rebuild_tree_rows_preserving_cursor();
                }
            }
            SessionTreeNode::SidebarPane { session_index, tab } => {
                if session_index < self.sessions.len() {
                    self.selected_index = session_index;
                    self.sidebar_tab = tab;
                    if self.sidebar_tab == SidebarTab::Chat {
                        self.chat_bridge.sync_now(&mut self.chat);
                    }
                    self.status_message = format!("Sidebar tab: {}", self.sidebar_tab.title());
                }
            }
            SessionTreeNode::ChatSession {
                session_index,
                chat_session_id,
            } => {
                if session_index < self.sessions.len() {
                    self.selected_index = session_index;
                    self.sidebar_tab = SidebarTab::Chat;
                    if self
                        .chat_bridge
                        .set_active_session(&mut self.chat, &chat_session_id)
                    {
                        self.status_message = format!("Chat session: {chat_session_id}");
                    } else {
                        self.status_message =
                            format!("Unable to switch chat session: {chat_session_id}");
                    }
                }
            }
        }
    }

    pub fn sync_tree_cursor_to_active_target(&mut self) {
        self.session_tree_rows = self.build_session_tree_rows();
        if self.session_tree_rows.is_empty() {
            self.session_tree_cursor = 0;
            self.session_tree_scroll = 0;
            return;
        }

        let selected_index = self
            .selected_index
            .min(self.sessions.len().saturating_sub(1));
        let target = if self.sidebar_tab == SidebarTab::Chat {
            if let Some(chat_session_id) = self.chat_bridge.active_session_id() {
                SessionTreeNode::ChatSession {
                    session_index: selected_index,
                    chat_session_id: chat_session_id.to_owned(),
                }
            } else {
                SessionTreeNode::SidebarPane {
                    session_index: selected_index,
                    tab: SidebarTab::Chat,
                }
            }
        } else {
            SessionTreeNode::SidebarPane {
                session_index: selected_index,
                tab: self.sidebar_tab,
            }
        };

        self.session_tree_cursor = self
            .find_tree_row_index(&target)
            .or_else(|| {
                self.find_tree_row_index(&SessionTreeNode::SessionRoot {
                    session_index: selected_index,
                })
            })
            .unwrap_or(0);
        self.session_tree_scroll = self
            .session_tree_scroll
            .min(self.session_tree_rows.len().saturating_sub(1));
    }

    fn build_session_tree_rows(&self) -> Vec<SessionTreeRow> {
        let mut roots = Vec::new();
        let selected_session_index = self
            .selected_index
            .min(self.sessions.len().saturating_sub(1));

        for (session_index, session) in self.sessions.iter().enumerate() {
            let is_selected_session = session_index == selected_session_index;

            // Collapse non-selected sessions: only expand children for the
            // selected session to reduce tree noise.
            let children = if is_selected_session {
                let visible_chat_sessions = filter_chat_sessions_for_session(
                    session,
                    self.chat_bridge.sessions(),
                    self.chat_bridge.active_session_id(),
                );
                let chat_children = build_chat_session_nodes(
                    session_index,
                    &visible_chat_sessions,
                    self.chat_bridge.active_session_id(),
                );

                let mut children = vec![
                    SessionTreeBuildNode {
                        node: SessionTreeNode::SidebarPane {
                            session_index,
                            tab: SidebarTab::Console,
                        },
                        label: String::from("console"),
                        children: Vec::new(),
                    },
                    SessionTreeBuildNode {
                        node: SessionTreeNode::SidebarPane {
                            session_index,
                            tab: SidebarTab::Agent,
                        },
                        label: String::from("agent"),
                        children: Vec::new(),
                    },
                ];
                children.extend(chat_children);
                children
            } else {
                Vec::new()
            };

            roots.push(SessionTreeBuildNode {
                node: SessionTreeNode::SessionRoot { session_index },
                label: session.session_name.clone(),
                children,
            });
        }

        let mut rows = Vec::new();
        flatten_tree_nodes(&roots, 0, &[], &mut rows);
        rows
    }

    fn find_tree_row_index(&self, target: &SessionTreeNode) -> Option<usize> {
        self.session_tree_rows
            .iter()
            .position(|row| row.node == *target)
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

fn flatten_tree_nodes(
    nodes: &[SessionTreeBuildNode],
    depth: usize,
    ancestors_are_last: &[bool],
    rows: &mut Vec<SessionTreeRow>,
) {
    for (position, node) in nodes.iter().enumerate() {
        let is_last = position + 1 == nodes.len();
        rows.push(SessionTreeRow {
            node: node.node.clone(),
            label: node.label.clone(),
            depth,
            is_last,
            ancestors_are_last: ancestors_are_last.to_vec(),
        });

        if !node.children.is_empty() {
            let mut next_ancestors = ancestors_are_last.to_vec();
            next_ancestors.push(is_last);
            flatten_tree_nodes(&node.children, depth + 1, &next_ancestors, rows);
        }
    }
}

fn build_chat_session_nodes(
    session_index: usize,
    chat_sessions: &[ChatSessionSummary],
    active_session_id: Option<&str>,
) -> Vec<SessionTreeBuildNode> {
    chat_sessions
        .iter()
        .map(|session| SessionTreeBuildNode {
            node: SessionTreeNode::ChatSession {
                session_index,
                chat_session_id: session.id.clone(),
            },
            label: if session.title.trim().is_empty() {
                if active_session_id == Some(session.id.as_str()) {
                    format!("* {}", session.id)
                } else {
                    session.id.clone()
                }
            } else {
                let compact = tinyverse_tui_components::compact_text(&session.title, 44);
                if active_session_id == Some(session.id.as_str()) {
                    format!("* {compact}")
                } else {
                    compact
                }
            },
            children: Vec::new(),
        })
        .collect()
}

fn filter_chat_sessions_for_session(
    session: &StoredSession,
    chat_sessions: &[ChatSessionSummary],
    active_session_id: Option<&str>,
) -> Vec<ChatSessionSummary> {
    let index_by_id = chat_sessions
        .iter()
        .enumerate()
        .map(|(index, item)| (item.id.as_str(), index))
        .collect::<HashMap<_, _>>();

    chat_sessions
        .iter()
        .filter(|chat_session| {
            chat_session_belongs_to_session(session, chat_session)
                || chat_session_related_to_active(
                    chat_session,
                    active_session_id,
                    chat_sessions,
                    &index_by_id,
                )
        })
        .cloned()
        .collect()
}

fn chat_session_belongs_to_session(
    session: &StoredSession,
    chat_session: &ChatSessionSummary,
) -> bool {
    let title = chat_session.title.trim();
    if title.is_empty() {
        return false;
    }
    let title_lower = title.to_ascii_lowercase();

    [
        session.session_name.as_str(),
        session.session_key.as_str(),
        session.tmux_session_name.as_str(),
    ]
    .into_iter()
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .map(str::to_ascii_lowercase)
    .any(|needle| title_lower.contains(&needle))
}

fn chat_session_related_to_active(
    chat_session: &ChatSessionSummary,
    active_session_id: Option<&str>,
    chat_sessions: &[ChatSessionSummary],
    index_by_id: &HashMap<&str, usize>,
) -> bool {
    let Some(active_id) = active_session_id else {
        return false;
    };
    if chat_session.id == active_id {
        return true;
    }

    let mut cursor = Some(active_id);
    let mut seen = HashSet::new();
    while let Some(current_id) = cursor {
        if !seen.insert(current_id) {
            break;
        }
        if current_id == chat_session.id {
            return true;
        }
        cursor = index_by_id
            .get(current_id)
            .and_then(|index| chat_sessions.get(*index))
            .and_then(|entry| entry.parent_id.as_deref());
    }

    let mut cursor = chat_session.parent_id.as_deref();
    let mut seen = HashSet::new();
    while let Some(current_id) = cursor {
        if !seen.insert(current_id) {
            break;
        }
        if current_id == active_id {
            return true;
        }
        cursor = index_by_id
            .get(current_id)
            .and_then(|index| chat_sessions.get(*index))
            .and_then(|entry| entry.parent_id.as_deref());
    }

    false
}
