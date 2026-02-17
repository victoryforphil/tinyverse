use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tinyverse_lib::{
    CapturePaneOptions, CreateSessionInput, PaneTarget, PanelRole, SendKeysOptions, SessionStore,
    SessionTarget, SpawnSessionOptions, StoredSession, TmuxClient, load_tmux_spawn_layout,
    resolve_session_name,
};

use crate::app::{
    ACTION_MENU_DANGER_SPLIT_AFTER, App, AppMode, DividerDrag, FooterHotkeyAction, MENU_ACTIONS,
    MenuAction, PanePreview, SessionsViewMode, SidebarTab,
};
use crate::chat::ChatMessageRole;
use crate::prefs::{self, TuiPrefs};
use tinyverse_tui_components::rect_contains;

use super::render;
use super::{restore_terminal, setup_terminal};

const CHAT_HINT_REFRESH_INTERVAL: Duration = Duration::from_secs(3);

pub(crate) fn handle_event(
    event: Event,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            handle_key_event(key, terminal, app, store)?
        }
        Event::Mouse(mouse) => handle_mouse_event(mouse, terminal, app, store)?,
        Event::Resize(_, _) => {
            app.layout.card_rects.clear();
            app.layout.card_kill_rects.clear();
        }
        _ => {}
    }

    Ok(())
}

pub(crate) fn refresh_chat_bridge(app: &mut App, force: bool) -> bool {
    let before_active_session_id = app.chat_bridge.active_session_id().map(ToOwned::to_owned);
    let before_known_session_count = app.chat_bridge.sessions().len();
    let before_status = app.chat_bridge.status();
    let before_message_count = app.chat.messages.len();

    let selected_session_key = app
        .selected_session()
        .map(|session| session.session_key.clone());
    let has_pinned_spawn_session = selected_session_key
        .as_deref()
        .and_then(|key| app.spawned_chat_session_ids.get(key))
        .is_some();
    let should_refresh_hints = force
        || selected_session_key != app.chat_hint_session_key
        || (!has_pinned_spawn_session
            && app
                .chat_hint_refreshed_at
                .map(|instant| instant.elapsed() >= CHAT_HINT_REFRESH_INTERVAL)
                .unwrap_or(true));

    if should_refresh_hints {
        app.chat_hint_directory = preferred_chat_directory_for_selected_session(app);
        app.chat_hint_base_url = preferred_chat_base_url_for_selected_session(app);
        app.chat_hint_session_id = preferred_chat_session_id_for_selected_session(app);
        app.chat_hint_session_key = selected_session_key;
        app.chat_hint_refreshed_at = Some(Instant::now());
    }

    if let Some(directory) = app.chat_hint_directory.as_deref() {
        app.chat_bridge.set_directory(directory);
    }

    if let Some(base_url) = app.chat_hint_base_url.as_deref() {
        app.chat_bridge.set_base_url(base_url);
    }

    if let Some(preferred_session_id) = app.chat_hint_session_id.as_deref() {
        let current_session_id = app.chat_bridge.active_session_id();
        if current_session_id != Some(preferred_session_id) {
            app.chat_bridge
                .set_active_session(&mut app.chat, preferred_session_id);
        }
    }

    if force {
        app.chat_bridge.sync_now(&mut app.chat);
    } else {
        app.chat_bridge.sync_if_due(&mut app.chat);
    }

    let after_active_session_id = app.chat_bridge.active_session_id().map(ToOwned::to_owned);
    let after_known_session_count = app.chat_bridge.sessions().len();
    let after_status = app.chat_bridge.status();
    let after_message_count = app.chat.messages.len();

    let bridge_changed = before_active_session_id != after_active_session_id
        || before_known_session_count != after_known_session_count
        || before_status.mode != after_status.mode
        || before_status.detail != after_status.detail
        || before_message_count != after_message_count;

    if bridge_changed || force {
        if app.sessions_view_mode == SessionsViewMode::Tree {
            app.rebuild_tree_rows_preserving_cursor();
        } else {
            app.sync_tree_cursor_to_active_target();
        }
    }

    bridge_changed
}

fn preferred_chat_directory_for_selected_session(app: &App) -> Option<String> {
    let session = app.selected_session()?;
    let pane_candidates = [
        session.agent_pane_id.as_deref(),
        session.console_pane_id.as_deref(),
    ];

    pane_candidates
        .into_iter()
        .flatten()
        .find_map(resolve_pane_current_path)
}

fn resolve_pane_current_path(pane_id: &str) -> Option<String> {
    let pane_id = pane_id.trim();
    if pane_id.is_empty() {
        return None;
    }

    let output = Command::new("tmux")
        .args([
            "display-message",
            "-p",
            "-t",
            pane_id,
            "#{pane_current_path}",
        ])
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if path.is_empty() { None } else { Some(path) }
}

fn preferred_chat_session_id_for_selected_session(app: &App) -> Option<String> {
    let session = app.selected_session()?;
    if let Some(pinned) = app.spawned_chat_session_ids.get(&session.session_key) {
        return Some(pinned.clone());
    }

    resolve_chat_session_id_from_agent_process(session)
}

fn preferred_chat_base_url_for_selected_session(app: &App) -> Option<String> {
    let session = app.selected_session()?;
    let pane_id = session.agent_pane_id.as_deref()?.trim();
    if pane_id.is_empty() {
        return None;
    }

    let pane_pid = pane_pid_for(pane_id)?;
    let candidate_pids = collect_process_tree_pids(&pane_pid);
    let mut commands = candidate_pids
        .iter()
        .filter_map(|pid| command_with_env_for_pid(pid).map(|command| (pid.as_str(), command)))
        .collect::<Vec<_>>();

    commands.sort_by_key(|(pid, command)| {
        let is_opencode = command.contains("/opencode") || command.contains(" opencode");
        (!is_opencode, *pid)
    });

    for (_pid, command) in commands {
        if let Some(port) = extract_opencode_port_from_command(&command) {
            return Some(format!("http://127.0.0.1:{port}"));
        }
    }

    None
}

fn resolve_chat_session_id_from_agent_process(session: &StoredSession) -> Option<String> {
    let pane_id = session.agent_pane_id.as_deref()?.trim();
    if pane_id.is_empty() {
        return None;
    }

    let Some(pane_pid) = pane_pid_for(pane_id) else {
        return None;
    };

    let candidate_pids = collect_process_tree_pids(&pane_pid);
    let mut commands = candidate_pids
        .iter()
        .filter_map(|pid| command_with_env_for_pid(pid).map(|command| (pid.as_str(), command)))
        .collect::<Vec<_>>();

    commands.sort_by_key(|(pid, command)| {
        let is_opencode = command.contains("/opencode") || command.contains(" opencode");
        (!is_opencode, *pid)
    });

    for (_pid, command) in commands {
        if let Some(session_id) = extract_any_chat_session_env(&command) {
            return Some(session_id);
        }
    }

    None
}

fn pane_pid_for(pane_id: &str) -> Option<String> {
    let output = Command::new("tmux")
        .args(["display-message", "-p", "-t", pane_id, "#{pane_pid}"])
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let pane_pid = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (!pane_pid.is_empty()).then_some(pane_pid)
}

fn collect_process_tree_pids(root_pid: &str) -> Vec<String> {
    let mut out = vec![root_pid.to_owned()];
    let mut cursor = 0usize;
    while cursor < out.len() {
        let parent = out[cursor].clone();
        cursor += 1;

        let children = child_pids(&parent);
        for child in children {
            if !out.contains(&child) {
                out.push(child);
            }
        }
    }

    out
}

fn child_pids(parent_pid: &str) -> Vec<String> {
    let output = Command::new("pgrep")
        .args(["-P", parent_pid])
        .stdin(Stdio::null())
        .output();
    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn command_with_env_for_pid(pid: &str) -> Option<String> {
    let output = Command::new("ps")
        .args(["eww", "-p", pid, "-o", "command="])
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let command = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (!command.is_empty()).then_some(command)
}

fn extract_any_chat_session_env(command: &str) -> Option<String> {
    extract_env_value(command, "OPENCODE_SESSION_ID")
        .or_else(|| extract_env_value(command, "OPENCODE_SESSION"))
        .or_else(|| extract_env_value(command, "DARK_CHAT_SESSION_ID"))
        .or_else(|| extract_session_id_from_command_args(command))
}

fn extract_opencode_port_from_command(command: &str) -> Option<u16> {
    let tokens = command.split_whitespace().collect::<Vec<_>>();
    let is_opencode_command = tokens
        .iter()
        .any(|token| token.contains("opencode") && !token.starts_with("OPENCODE_"));
    if !is_opencode_command {
        return None;
    }

    for (index, token) in tokens.iter().enumerate() {
        if let Some(value) = token.strip_prefix("--port=")
            && let Ok(port) = value.trim().parse::<u16>()
            && port > 0
        {
            return Some(port);
        }

        if *token == "--port"
            && let Some(value) = tokens.get(index + 1)
            && let Ok(port) = value.trim().parse::<u16>()
            && port > 0
        {
            return Some(port);
        }
    }

    None
}

fn extract_session_id_from_command_args(command: &str) -> Option<String> {
    let tokens = command.split_whitespace().collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if let Some(value) = token.strip_prefix("--session=")
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }

        if (*token == "--session" || *token == "-s")
            && let Some(value) = tokens.get(index + 1)
            && !value.starts_with('-')
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }
    }

    None
}

fn extract_env_value(command: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    command
        .split_whitespace()
        .find_map(|token| token.strip_prefix(&prefix).map(ToOwned::to_owned))
        .filter(|value| !value.trim().is_empty())
}

const DOUBLE_ESC_WINDOW_MS: u128 = 400;

fn handle_key_event(
    key: KeyEvent,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match app.mode {
        AppMode::PaneFocus => handle_pane_focus_key(key, app),
        AppMode::Normal => match key.code {
            _ if app.sidebar_tab == SidebarTab::Chat
                && app.sessions_view_mode != SessionsViewMode::Tree
                && handle_chat_key_event(key.code, app, store)? => {}
            KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if app.sessions_view_mode == SessionsViewMode::Tree {
                    app.move_tree_cursor_up();
                    app.activate_tree_cursor();
                } else {
                    app.select_prev();
                }
                refresh_selected_preview(app);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.sessions_view_mode == SessionsViewMode::Tree {
                    app.move_tree_cursor_down();
                    app.activate_tree_cursor();
                } else {
                    app.select_next();
                }
                refresh_selected_preview(app);
            }
            KeyCode::Char('h') => {
                if app.sessions_view_mode == SessionsViewMode::Tree {
                    app.move_tree_cursor_up();
                    app.activate_tree_cursor();
                } else {
                    app.select_prev();
                }
                refresh_selected_preview(app);
            }
            KeyCode::Char('l') => {
                if app.sessions_view_mode == SessionsViewMode::Tree {
                    app.move_tree_cursor_down();
                    app.activate_tree_cursor();
                } else {
                    app.select_next();
                }
                refresh_selected_preview(app);
            }
            KeyCode::Left => app.prev_sidebar_tab(),
            KeyCode::Right => app.next_sidebar_tab(),
            KeyCode::Char('r') => refresh_sessions_and_preview(app, store)?,
            KeyCode::Char('i') | KeyCode::Tab => app.toggle_inspector(),
            KeyCode::Char('v') => app.toggle_sessions_view_mode(),
            KeyCode::Char(']') => app.next_sidebar_tab(),
            KeyCode::Char('[') => app.prev_sidebar_tab(),
            KeyCode::Char('1') => app.set_sidebar_tab(SidebarTab::Console),
            KeyCode::Char('2') => app.set_sidebar_tab(SidebarTab::Agent),
            KeyCode::Char('3') => app.set_sidebar_tab(SidebarTab::Chat),
            KeyCode::Char('f') => enter_pane_focus(app),
            KeyCode::Enter => {
                if app.sessions_view_mode == SessionsViewMode::Tree {
                    app.activate_tree_cursor();
                    refresh_selected_preview(app);
                } else {
                    app.open_action_menu();
                }
            }
            KeyCode::Char('a') => attach_selected_session(terminal, app)?,
            KeyCode::Char('s') => {
                app.reset_spawn_form();
                app.mode = AppMode::SpawnInput;
            }
            KeyCode::Char('x') => {
                if app.selected_session().is_some() {
                    app.mode = AppMode::ConfirmKill;
                } else {
                    app.status_message = String::from("No session selected");
                }
            }
            _ => {}
        },
        AppMode::ActionMenu => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => app.close_action_menu(),
            KeyCode::Up | KeyCode::Char('k') => app.action_menu_prev(),
            KeyCode::Down | KeyCode::Char('j') => app.action_menu_next(),
            KeyCode::Enter => {
                execute_menu_action(app.selected_menu_action(), terminal, app, store)?
            }
            KeyCode::Char(c) => {
                if let Some(action) = MenuAction::from_hotkey(c) {
                    execute_menu_action(action, terminal, app, store)?;
                }
            }
            _ => {}
        },
        AppMode::ConfirmKill => match key.code {
            KeyCode::Esc | KeyCode::Char('n') => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill canceled");
            }
            KeyCode::Enter | KeyCode::Char('y') => {
                kill_selected_session(app, store)?;
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::ConfirmKillAll => match key.code {
            KeyCode::Esc | KeyCode::Char('n') => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill all canceled");
            }
            KeyCode::Enter | KeyCode::Char('y') => {
                kill_all_sessions(app, store)?;
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::SendInput => match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
                app.input_buffer.clear();
            }
            KeyCode::Enter => send_console_input(app),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            KeyCode::Char(c) => app.input_buffer.push(c),
            _ => {}
        },
        AppMode::SpawnInput => match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
                app.reset_spawn_form();
            }
            KeyCode::Enter => spawn_session_from_input(app, store)?,
            KeyCode::Char('e') => open_prompt_in_editor(terminal, app)?,
            KeyCode::Tab => app.spawn_form.next_field(),
            KeyCode::BackTab => app.spawn_form.prev_field(),
            KeyCode::Backspace => {
                app.spawn_form.active_field_mut().pop();
            }
            KeyCode::Char(c) => app.spawn_form.active_field_mut().push(c),
            _ => {}
        },
    }

    Ok(())
}

fn handle_mouse_event(
    mouse: MouseEvent,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    let x = mouse.column;
    let y = mouse.row;
    app.footer_hover_action = render::footer_hotkey_hit_test(app, x, y);

    if mouse.kind == MouseEventKind::Down(MouseButton::Left)
        && let Some(action) = render::footer_hotkey_hit_test(app, x, y)
    {
        execute_footer_action(action, terminal, app, store)?;
        return Ok(());
    }

    if app.mode == AppMode::ActionMenu {
        if let Some(menu_rect) = app.layout.action_menu_rect {
            match mouse.kind {
                MouseEventKind::Moved => {
                    if rect_contains(menu_rect, x, y)
                        && let Some(index) = action_menu_index_from_click(menu_rect, y)
                    {
                        app.action_menu_index = index;
                    }
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    if rect_contains(menu_rect, x, y) {
                        if let Some(index) = action_menu_index_from_click(menu_rect, y) {
                            app.action_menu_index = index;
                            execute_menu_action(MENU_ACTIONS[index], terminal, app, store)?;
                        }
                    } else {
                        app.close_action_menu();
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.mode == AppMode::ConfirmKill {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(confirm_rect) = app.layout.confirm_rect {
                if !rect_contains(confirm_rect, x, y) {
                    app.mode = AppMode::ActionMenu;
                    app.status_message = String::from("Kill canceled");
                }
            }
        }
        return Ok(());
    }

    if app.mode == AppMode::ConfirmKillAll {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(confirm_rect) = app.layout.confirm_rect {
                if !rect_contains(confirm_rect, x, y) {
                    app.mode = AppMode::ActionMenu;
                    app.status_message = String::from("Kill all canceled");
                }
            }
        }
        return Ok(());
    }

    if app.mode == AppMode::SendInput {
        return Ok(());
    }

    if app.mode == AppMode::SpawnInput {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(dialog_rect) = app.layout.overlay.dialog_rect
                && !rect_contains(dialog_rect, x, y)
            {
                app.mode = AppMode::Normal;
                app.reset_spawn_form();
                return Ok(());
            }

            if let Some(editor_rect) = app.layout.overlay.prompt_editor_rect
                && rect_contains(editor_rect, x, y)
            {
                open_prompt_in_editor(terminal, app)?;
                return Ok(());
            }

            if let Some(index) = app
                .layout
                .overlay
                .field_rects
                .iter()
                .position(|rect| rect_contains(*rect, x, y))
            {
                app.spawn_form.active_field = index;
            }
        }
        return Ok(());
    }

    if mouse.kind == MouseEventKind::Down(MouseButton::Left)
        && let Some(header_rect) = app.layout.sessions_header_rect
        && rect_contains(header_rect, x, y)
    {
        app.toggle_sessions_minimized();
        return Ok(());
    }

    if mouse.kind == MouseEventKind::Down(MouseButton::Left)
        && let Some(header_rect) = app.layout.sidebar_header_rect
        && rect_contains(header_rect, x, y)
    {
        app.toggle_sidebar_minimized();
        return Ok(());
    }

    if app.sidebar_tab == SidebarTab::Chat && handle_chat_mouse_event(mouse, app)? {
        return Ok(());
    }

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(mode) = sessions_view_mode_from_position(x, y, app) {
                app.set_sessions_view_mode(mode);
                refresh_selected_preview(app);
                return Ok(());
            }
            if app.sessions_view_mode == SessionsViewMode::Tree
                && let Some(row_index) = tree_row_index_from_position(x, y, app)
            {
                app.set_tree_cursor(row_index);
                app.activate_tree_cursor();
                refresh_selected_preview(app);
                return Ok(());
            }
            if let Some(index) = card_kill_index_from_position(x, y, app) {
                app.selected_index = index;
                refresh_selected_preview(app);
                app.mode = AppMode::ConfirmKill;
                return Ok(());
            }
            if let Some(tab) = sidebar_tab_from_position(x, y, app) {
                app.set_sidebar_tab(tab);
                refresh_selected_preview(app);
                return Ok(());
            }
            if app.inspector_visible && is_near_vertical_divider(x, app) {
                app.dragging_divider = Some(DividerDrag::Vertical);
                return Ok(());
            }
            if app.inspector_visible && is_near_horizontal_divider(y, app) {
                app.dragging_divider = Some(DividerDrag::Horizontal);
                return Ok(());
            }
            if let Some(index) = card_index_from_position(x, y, app) {
                app.selected_index = index;
                refresh_selected_preview(app);
            }
        }
        MouseEventKind::Down(MouseButton::Right) => {
            if app.sessions_view_mode == SessionsViewMode::Tree
                && let Some(row_index) = tree_row_index_from_position(x, y, app)
            {
                app.set_tree_cursor(row_index);
                app.activate_tree_cursor();
                refresh_selected_preview(app);
            } else if let Some(index) = card_index_from_position(x, y, app) {
                app.selected_index = index;
                refresh_selected_preview(app);
            }
            app.mode = AppMode::ActionMenu;
            app.action_menu_index = 0;
            app.action_menu_anchor = Some((x, y));
        }
        MouseEventKind::Drag(MouseButton::Left) => match app.dragging_divider {
            Some(DividerDrag::Vertical) => update_vertical_divider_ratio(x, app),
            Some(DividerDrag::Horizontal) => update_horizontal_divider_height(y, app),
            None => {}
        },
        MouseEventKind::Up(MouseButton::Left) => {
            app.dragging_divider = None;
        }
        MouseEventKind::ScrollDown => {
            if app.sessions_view_mode == SessionsViewMode::Tree {
                app.move_tree_cursor_down();
                app.activate_tree_cursor();
            } else {
                app.select_next();
            }
        }
        MouseEventKind::ScrollUp => {
            if app.sessions_view_mode == SessionsViewMode::Tree {
                app.move_tree_cursor_up();
                app.activate_tree_cursor();
            } else {
                app.select_prev();
            }
        }
        _ => {}
    }

    if matches!(
        mouse.kind,
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp
    ) {
        refresh_selected_preview(app);
    }

    Ok(())
}

fn handle_chat_key_event(key: KeyCode, app: &mut App, store: &mut SessionStore) -> Result<bool> {
    if app.chat.is_detail_modal_open() {
        match key {
            KeyCode::Enter | KeyCode::Char('q') => {
                app.chat.close_detail_modal();
                app.status_message = String::from("Detail popup closed");
            }
            _ => {}
        }
        return Ok(true);
    }

    if app.chat.is_model_selector_open() {
        match key {
            KeyCode::Tab => {
                app.chat.model_selector_toggle_raw_mode();
                app.status_message = if app.chat.model_selector.raw_mode {
                    String::from("Model selector raw mode")
                } else {
                    String::from("Model selector filter mode")
                };
            }
            KeyCode::Up => app.chat.model_selector_move_up(),
            KeyCode::Down => app.chat.model_selector_move_down(),
            KeyCode::Backspace => app.chat.model_selector_backspace(),
            KeyCode::Enter => {
                if let Some(selected) = app.chat.confirm_model_selector() {
                    app.status_message = format!("Model selected: {selected}");
                } else {
                    app.chat.close_model_selector();
                    app.status_message = String::from("No model selected");
                }
            }
            KeyCode::Char(c) => app.chat.model_selector_insert_char(c),
            _ => {}
        }
        return Ok(true);
    }

    if app.chat.is_agent_selector_open() {
        match key {
            KeyCode::Up => app.chat.agent_selector_move_up(),
            KeyCode::Down => app.chat.agent_selector_move_down(),
            KeyCode::Backspace => app.chat.agent_selector_backspace(),
            KeyCode::Enter => {
                if let Some(selected) = app.chat.confirm_agent_selector() {
                    app.status_message = format!("Agent selected: {selected}");
                } else {
                    app.chat.close_agent_selector();
                    app.status_message = String::from("No agent selected");
                }
            }
            KeyCode::Char(c) => app.chat.agent_selector_insert_char(c),
            _ => {}
        }
        return Ok(true);
    }

    if app.chat.is_autocomplete_open() {
        match key {
            KeyCode::Up => app.chat.autocomplete_move_up(),
            KeyCode::Down => app.chat.autocomplete_move_down(),
            KeyCode::Tab | KeyCode::Enter => {
                let _ = app.chat.apply_autocomplete_selection();
            }
            _ => {}
        }
        return Ok(true);
    }

    if app.chat.composing {
        match key {
            KeyCode::Esc => {
                app.chat.cancel_composer();
                app.status_message = String::from("Compose cancelled");
            }
            KeyCode::Delete => app.chat.delete_char(),
            KeyCode::Left => app.chat.move_cursor_left(),
            KeyCode::Right => app.chat.move_cursor_right(),
            KeyCode::Home => app.chat.move_cursor_home(),
            KeyCode::End => app.chat.move_cursor_end(),
            KeyCode::Enter => submit_chat_prompt(app, store)?,
            KeyCode::Backspace => app.chat.backspace_char(),
            KeyCode::Char('u') => app.chat.clear_draft(),
            KeyCode::Char(c) => app.chat.insert_char(c),
            _ => {}
        }
        return Ok(true);
    }

    match key {
        KeyCode::Char('c') => {
            app.chat.open_composer();
            app.status_message = String::from("Compose mode enabled");
            Ok(true)
        }
        KeyCode::Char('m') => {
            app.chat.open_model_selector();
            app.status_message = String::from("Model selector opened");
            Ok(true)
        }
        KeyCode::Char('g') => {
            app.chat.open_agent_selector();
            app.status_message = String::from("Agent selector opened");
            Ok(true)
        }
        KeyCode::Char('z') => {
            app.status_message = String::from("Inline detail dropdowns disabled; use popups");
            Ok(true)
        }
        KeyCode::Enter => {
            if app.chat.open_detail_modal_for_focused() {
                app.status_message = String::from("Opened detail popup");
                return Ok(true);
            }
            if let Some(first) = app.layout.chat.part_toggle_hitboxes.first() {
                app.chat.open_detail_modal_for_part(first.part_key.clone());
                app.status_message = String::from("Opened detail popup");
                return Ok(true);
            }
            Ok(false)
        }
        KeyCode::Tab => {
            if focus_next_chat_part(app) {
                app.status_message = String::from("Focused next detail section");
                return Ok(true);
            }
            Ok(false)
        }
        KeyCode::BackTab => {
            if focus_prev_chat_part(app) {
                app.status_message = String::from("Focused previous detail section");
                return Ok(true);
            }
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.chat.scroll_up(2);
            Ok(true)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.chat.scroll_down(2);
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn handle_chat_mouse_event(mouse: MouseEvent, app: &mut App) -> Result<bool> {
    let x = mouse.column;
    let y = mouse.row;

    if app.chat.is_detail_modal_open() {
        if let Some(popup) = app.layout.chat.detail_modal_rect {
            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if !rect_contains(popup, x, y) {
                        app.chat.close_detail_modal();
                        app.status_message = String::from("Detail popup closed");
                    }
                    return Ok(true);
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    app.chat.close_detail_modal();
                    app.status_message = String::from("Detail popup closed");
                    return Ok(true);
                }
                MouseEventKind::ScrollUp => {
                    app.chat.detail_scroll_up(2);
                    return Ok(true);
                }
                MouseEventKind::ScrollDown => {
                    app.chat.detail_scroll_down(2);
                    return Ok(true);
                }
                _ => return Ok(true),
            }
        }
    }

    if mouse.kind == MouseEventKind::Down(MouseButton::Left)
        && let Some(tab) = sidebar_tab_from_position(x, y, app)
    {
        app.set_sidebar_tab(tab);
        return Ok(true);
    }

    if app.chat.is_model_selector_open() {
        if let Some(popup) = app.layout.chat.model_selector_rect {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                if !rect_contains(popup, x, y) {
                    app.chat.close_model_selector();
                    app.status_message = String::from("Model selector closed");
                    return Ok(true);
                }

                if let Some(query_rect) = app.layout.chat.model_selector_query_rect
                    && rect_contains(query_rect, x, y)
                    && !app.chat.model_selector.raw_mode
                {
                    app.chat.model_selector_toggle_raw_mode();
                    app.status_message = String::from("Model selector raw mode");
                    return Ok(true);
                }

                if let Some(list_rect) = app.layout.chat.model_selector_list_rect
                    && rect_contains(list_rect, x, y)
                {
                    let row = y.saturating_sub(list_rect.y) as usize;
                    let index = app.layout.chat.model_selector_list_start + row;
                    app.chat.model_selector_set_selected(index);
                    if let Some(selected) = app.chat.confirm_model_selector() {
                        app.status_message = format!("Model selected: {selected}");
                    }
                }
            }
            if mouse.kind == MouseEventKind::ScrollUp {
                app.chat.model_selector_move_up();
            }
            if mouse.kind == MouseEventKind::ScrollDown {
                app.chat.model_selector_move_down();
            }
            return Ok(true);
        }
    }

    if app.chat.is_agent_selector_open() {
        if let Some(popup) = app.layout.chat.agent_selector_rect {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                if !rect_contains(popup, x, y) {
                    app.chat.close_agent_selector();
                    app.status_message = String::from("Agent selector closed");
                    return Ok(true);
                }

                if let Some(list_rect) = app.layout.chat.agent_selector_list_rect
                    && rect_contains(list_rect, x, y)
                {
                    let row = y.saturating_sub(list_rect.y) as usize;
                    let index = app.layout.chat.agent_selector_list_start + row;
                    app.chat.agent_selector_set_selected(index);
                    if let Some(selected) = app.chat.confirm_agent_selector() {
                        app.status_message = format!("Agent selected: {selected}");
                    }
                }
            }
            if mouse.kind == MouseEventKind::ScrollUp {
                app.chat.agent_selector_move_up();
            }
            if mouse.kind == MouseEventKind::ScrollDown {
                app.chat.agent_selector_move_down();
            }
            return Ok(true);
        }
    }

    if app.chat.is_autocomplete_open() {
        if let Some(popup) = app.layout.chat.autocomplete_rect {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                if !rect_contains(popup, x, y) {
                    app.chat.close_autocomplete();
                    return Ok(true);
                }

                if let Some(list_rect) = app.layout.chat.autocomplete_list_rect
                    && rect_contains(list_rect, x, y)
                {
                    let row = y.saturating_sub(list_rect.y) as usize;
                    let index = app.layout.chat.autocomplete_list_start + row;
                    app.chat.autocomplete_set_selected(index);
                    let _ = app.chat.apply_autocomplete_selection();
                }
            }
            if mouse.kind == MouseEventKind::ScrollUp {
                app.chat.autocomplete_move_up();
            }
            if mouse.kind == MouseEventKind::ScrollDown {
                app.chat.autocomplete_move_down();
            }
            return Ok(true);
        }
    }

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Down(MouseButton::Right) => {
            if let Some(model_rect) = app.layout.chat.model_chip_rect
                && rect_contains(model_rect, x, y)
            {
                app.chat.open_model_selector_at(x);
                app.status_message = String::from("Model selector opened");
                return Ok(true);
            }
            if let Some(agent_rect) = app.layout.chat.agent_chip_rect
                && rect_contains(agent_rect, x, y)
            {
                app.chat.open_agent_selector_at(x);
                app.status_message = String::from("Agent selector opened");
                return Ok(true);
            }
            if let Some(input_rect) = app.layout.chat.composer_input_rect
                && rect_contains(input_rect, x, y)
            {
                app.chat.open_composer();
                return Ok(true);
            }
            for hitbox in &app.layout.chat.part_toggle_hitboxes {
                if rect_contains(hitbox.rect, x, y) {
                    app.chat.set_focused_part_key(Some(hitbox.part_key.clone()));
                    app.chat.open_detail_modal_for_part(hitbox.part_key.clone());
                    app.status_message = String::from("Opened detail popup");
                    return Ok(true);
                }
            }
            if let Some(messages_rect) = app.layout.chat.messages_rect
                && rect_contains(messages_rect, x, y)
            {
                return Ok(true);
            }
            Ok(false)
        }
        MouseEventKind::ScrollUp => {
            app.chat.scroll_up(2);
            Ok(true)
        }
        MouseEventKind::ScrollDown => {
            app.chat.scroll_down(2);
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn submit_chat_prompt(app: &mut App, store: &mut SessionStore) -> Result<()> {
    let Some(prompt) = app.chat.take_prompt() else {
        app.status_message = String::from("Prompt is empty");
        return Ok(());
    };

    if let Some(local) = parse_local_chat_command(&prompt) {
        app.chat.clear_after_send();
        execute_local_chat_command(local, app, store)?;
        return Ok(());
    }

    app.chat.push_message(ChatMessageRole::User, prompt.clone());

    let outcome =
        app.chat_bridge
            .send_prompt(&prompt, &app.chat.active_model, &app.chat.active_agent);
    if outcome.via_opencode {
        app.chat
            .push_message(ChatMessageRole::System, outcome.detail.clone());
        app.chat.clear_after_send();
        app.status_message = outcome.detail;
        refresh_chat_bridge(app, true);
        return Ok(());
    }

    let Some(session) = app.selected_session().cloned() else {
        app.chat.push_message(
            ChatMessageRole::System,
            format!(
                "{} No session selected; chat message kept local.",
                outcome.detail
            ),
        );
        app.chat.clear_after_send();
        app.status_message = String::from("Chat message stored locally");
        return Ok(());
    };

    let mut options = SendKeysOptions::new(session.tmux_session_name.clone(), prompt);
    options.pane = Some(PaneTarget::Role(PanelRole::Agent));

    match TmuxClient::new().send_keys(options) {
        Ok(()) => {
            app.chat.push_message(
                ChatMessageRole::Assistant,
                format!(
                    "Dispatched to `{}` agent pane. Live stream binding is not wired yet.",
                    session.session_name
                ),
            );
            app.status_message = String::from("Prompt sent to agent pane");
        }
        Err(error) => {
            app.chat
                .push_message(ChatMessageRole::System, format!("Dispatch failed: {error}"));
            app.status_message = format!("Prompt dispatch failed: {error}");
        }
    }

    app.chat.clear_after_send();
    Ok(())
}

#[derive(Debug, Clone)]
enum LocalChatCommand {
    Help,
    Refresh,
    New,
    Clear,
    Sessions,
    Agent(String),
    Model(String),
    Grep(String),
}

fn parse_local_chat_command(prompt: &str) -> Option<LocalChatCommand> {
    let trimmed = prompt.trim();
    let command = trimmed.strip_prefix('/')?.trim();
    if command.is_empty() {
        return None;
    }

    let mut parts = command.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim().to_ascii_lowercase();
    let arg = parts.next().map(str::trim).unwrap_or("");

    match name.as_str() {
        "help" => Some(LocalChatCommand::Help),
        "refresh" => Some(LocalChatCommand::Refresh),
        "new" | "new-session" => Some(LocalChatCommand::New),
        "clear" => Some(LocalChatCommand::Clear),
        "sessions" => Some(LocalChatCommand::Sessions),
        "agent" if !arg.is_empty() => Some(LocalChatCommand::Agent(arg.to_owned())),
        "model" if !arg.is_empty() => Some(LocalChatCommand::Model(arg.to_owned())),
        "grep" if !arg.is_empty() => Some(LocalChatCommand::Grep(arg.to_owned())),
        _ => None,
    }
}

fn execute_local_chat_command(
    command: LocalChatCommand,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match command {
        LocalChatCommand::Help => {
            app.chat.push_message(
                ChatMessageRole::System,
                "Local commands: /help /refresh /new /clear /sessions /agent <name> /model <name> /grep <pattern>",
            );
            app.status_message = String::from("Displayed local chat help");
        }
        LocalChatCommand::Refresh => {
            refresh_sessions_and_preview(app, store)?;
            refresh_chat_bridge(app, true);
            app.status_message = String::from("Refreshed sessions");
        }
        LocalChatCommand::New => {
            app.reset_spawn_form();
            app.mode = AppMode::SpawnInput;
            app.status_message = String::from("Spawn dialog opened");
        }
        LocalChatCommand::Clear => {
            app.chat.clear_messages();
            app.status_message = String::from("Cleared local chat messages");
        }
        LocalChatCommand::Sessions => {
            app.chat.push_message(
                ChatMessageRole::System,
                format!("Loaded session records: {}", app.sessions.len()),
            );
            app.status_message = String::from("Printed session count");
        }
        LocalChatCommand::Agent(agent) => {
            if app.chat.set_active_agent(&agent).is_some() {
                app.status_message = format!("Agent selected: {agent}");
            } else {
                app.status_message = format!("Unknown agent: {agent}");
            }
        }
        LocalChatCommand::Model(model) => {
            app.chat.set_active_model(&model);
            app.status_message = format!("Model selected: {model}");
        }
        LocalChatCommand::Grep(pattern) => {
            match run_local_grep_summary(app.chat.workspace_root(), &pattern) {
                Ok(summary) => {
                    app.chat
                        .push_message(ChatMessageRole::System, summary.clone());
                    app.status_message = summary;
                }
                Err(error) => {
                    app.status_message = format!("Grep failed: {error}");
                }
            }
        }
    }

    Ok(())
}

fn run_local_grep_summary(root: &Path, pattern: &str) -> Result<String> {
    let output = Command::new("rg")
        .arg("--line-number")
        .arg("--max-count")
        .arg("12")
        .arg(pattern)
        .arg(root)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let hits = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(3)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    let status_code = output.status.code().unwrap_or_default();
    if status_code != 0 && status_code != 1 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Ok(format!("/grep failed ({status_code}): {}", stderr.trim()));
    }

    if hits.is_empty() {
        Ok(format!("/grep no matches for `{pattern}`"))
    } else {
        Ok(format!("/grep {}", hits.join(" | ")))
    }
}

fn sidebar_tab_from_position(x: u16, y: u16, app: &App) -> Option<SidebarTab> {
    app.layout
        .sidebar_tab_rects
        .iter()
        .find_map(|(tab, rect)| rect_contains(*rect, x, y).then_some(*tab))
}

fn sessions_view_mode_from_position(x: u16, y: u16, app: &App) -> Option<SessionsViewMode> {
    app.layout
        .sessions_view_tab_rects
        .iter()
        .find_map(|(mode, rect)| rect_contains(*rect, x, y).then_some(*mode))
}

fn tree_row_index_from_position(x: u16, y: u16, app: &App) -> Option<usize> {
    app.layout
        .session_tree_row_rects
        .iter()
        .find_map(|(row_index, rect)| rect_contains(*rect, x, y).then_some(*row_index))
}

fn execute_menu_action(
    action: MenuAction,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match action {
        MenuAction::Refresh => {
            refresh_sessions_and_preview(app, store)?;
            app.mode = AppMode::Normal;
        }
        MenuAction::ToggleInspector => {
            app.toggle_inspector();
            app.mode = AppMode::Normal;
        }
        MenuAction::AttachSession => {
            attach_selected_session(terminal, app)?;
            app.mode = AppMode::Normal;
        }
        MenuAction::SendToConsole => {
            app.input_buffer.clear();
            app.mode = AppMode::SendInput;
        }
        MenuAction::SpawnSession => {
            app.reset_spawn_form();
            app.mode = AppMode::SpawnInput;
        }
        MenuAction::KillSession => {
            if app.selected_session().is_none() {
                app.status_message = String::from("No session selected");
                app.mode = AppMode::Normal;
            } else {
                app.mode = AppMode::ConfirmKill;
            }
        }
        MenuAction::CloseMenu => {
            app.mode = AppMode::Normal;
        }
        MenuAction::KillAllSessions => {
            if app.sessions.is_empty() {
                app.status_message = String::from("No sessions available");
                app.mode = AppMode::Normal;
            } else {
                app.mode = AppMode::ConfirmKillAll;
            }
        }
    }

    Ok(())
}

fn attach_selected_session(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let Some(session) = app.selected_session().cloned() else {
        app.status_message = String::from("No session selected");
        return Ok(());
    };

    let in_tmux = std::env::var_os("TMUX").is_some();
    let args = build_tmux_attach_args(in_tmux, &session.tmux_session_name);

    restore_terminal(terminal)?;
    let status_result = Command::new("tmux")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    *terminal = setup_terminal()?;

    match status_result {
        Ok(status) if status.success() => {
            app.status_message = format!("Returned from {}", session.session_name);
        }
        Ok(status) => {
            app.status_message = format!(
                "Attach failed for {} (exit {:?})",
                session.session_name,
                status.code()
            );
        }
        Err(error) => {
            app.status_message = format!("Attach failed for {}: {}", session.session_name, error);
        }
    }

    Ok(())
}

fn send_console_input(app: &mut App) {
    let command = app.input_buffer.trim().to_owned();
    if command.is_empty() {
        app.status_message = String::from("Command cannot be empty");
        return;
    }

    let Some(session) = app.selected_session().cloned() else {
        app.status_message = String::from("No session selected");
        return;
    };

    let mut options = SendKeysOptions::new(session.tmux_session_name.clone(), command);
    options.pane = Some(PaneTarget::Role(PanelRole::Console));

    match TmuxClient::new().send_keys(options) {
        Ok(()) => {
            app.status_message = format!("Sent command to {} console", session.session_name);
            app.input_buffer.clear();
            app.mode = AppMode::Normal;
        }
        Err(error) => {
            app.status_message = format!("Send failed: {error}");
        }
    }
}

fn spawn_session_from_input(app: &mut App, store: &mut SessionStore) -> Result<()> {
    let mut session_name = app.spawn_form.session_name.trim().to_owned();
    if session_name.is_empty() || session_name == "tinyverse_" {
        session_name = resolve_session_name(None, store)?;
    }

    let agent_type = if app.spawn_form.agent_type.trim().is_empty() {
        String::from("opencode")
    } else {
        app.spawn_form.agent_type.trim().to_owned()
    };

    let prompt = app.spawn_form.prompt.trim().to_owned();
    let model = app.spawn_form.model.trim().to_owned();
    save_spawn_prefs(app);

    let tmux_session_name = session_name.clone();
    let mut attach_url: Option<String> = None;
    let mut attach_session_id: Option<String> = None;
    if agent_type.eq_ignore_ascii_case("opencode") {
        let title = format!("tinyverse: {session_name}");
        match app.chat_bridge.create_session_for_spawn(&mut app.chat, &title) {
            Ok(session_id) => {
                attach_url = app.chat_bridge.opencode_base_url();
                attach_session_id = Some(session_id);
            }
            Err(error) => {
                app.status_message = format!("Spawn failed: unable to create chat session: {error}");
                return Ok(());
            }
        }
    }

    let mut spawn_options = SpawnSessionOptions::new(&tmux_session_name);
    let tmux_layout = load_tmux_spawn_layout();
    spawn_options.initial_window_width = Some(tmux_layout.initial_window_width);
    spawn_options.initial_window_height = Some(tmux_layout.initial_window_height);
    spawn_options.split_direction = tmux_layout.split_direction;
    spawn_options.primary_role = tmux_layout.primary_role;
    spawn_options.secondary_size_percent = Some(tmux_layout.secondary_size_percent);
    let resolved_prompt = resolve_prompt_input(&prompt);
    spawn_options.agent_command = Some(build_agent_command(
        &tmux_session_name,
        &agent_type,
        &model,
        &resolved_prompt,
        attach_url.as_deref(),
        attach_session_id.as_deref(),
    ));
    let spawn_result = TmuxClient::new().spawn_session(spawn_options);

    let spawned = match spawn_result {
        Ok(value) => value,
        Err(error) => {
            app.status_message = format!("Spawn failed: {error}");
            return Ok(());
        }
    };

    let created = store.create_session(&CreateSessionInput {
        session_name: session_name.clone(),
        agent_type: agent_type.clone(),
        description: Some(String::from("Spawned from tinyverse tui")),
        tmux_session_name,
        tmux_session_id: None,
        console_pane_id: Some(spawned.console_pane_id),
        agent_pane_id: Some(spawned.agent_pane_id),
    });

    match created {
        Ok(record) => {
            if let Some(session_id) = attach_session_id.as_deref() {
                app.spawned_chat_session_ids
                    .insert(record.session_key.clone(), session_id.to_owned());
                app.chat_bridge
                    .set_active_session(&mut app.chat, session_id);
            }
            refresh_sessions_and_preview(app, store)?;
            if let Some(index) = app
                .sessions
                .iter()
                .position(|session| session.session_key == record.session_key)
            {
                app.selected_index = index;
            }
            refresh_selected_preview(app);
            app.status_message = format!("Spawned {} ({})", record.session_name, agent_type);
            app.reset_spawn_form();
            app.mode = AppMode::Normal;
        }
        Err(error) => {
            app.status_message = format!("Spawned tmux session but DB create failed: {error}");
        }
    }

    Ok(())
}

fn kill_selected_session(app: &mut App, store: &mut SessionStore) -> Result<()> {
    let Some(session) = app.selected_session().cloned() else {
        app.status_message = String::from("No session selected");
        return Ok(());
    };

    let tmux_client = TmuxClient::new();
    let tmux_result = tmux_client.kill_session(SessionTarget::new(session.tmux_session_name));
    let db_deleted = store.delete_session_by_key(&session.session_key)?;
    refresh_sessions_and_preview(app, store)?;

    app.status_message = match (tmux_result, db_deleted) {
        (Ok(()), true) => format!("Killed {}", session.session_name),
        (Ok(()), false) => format!("tmux killed {}; DB record missing", session.session_name),
        (Err(error), true) => {
            format!(
                "Deleted {} from DB; tmux kill failed: {error}",
                session.session_name
            )
        }
        (Err(error), false) => format!("Kill failed for {}: {error}", session.session_name),
    };

    Ok(())
}

fn kill_all_sessions(app: &mut App, store: &mut SessionStore) -> Result<()> {
    let sessions = app.sessions.clone();
    let tmux_client = TmuxClient::new();

    let mut deleted_count = 0usize;
    let mut tmux_failures = 0usize;

    for session in sessions {
        if tmux_client
            .kill_session(SessionTarget::new(session.tmux_session_name.clone()))
            .is_err()
        {
            tmux_failures += 1;
        }
        if store.delete_session_by_key(&session.session_key)? {
            deleted_count += 1;
        }
    }

    refresh_sessions_and_preview(app, store)?;
    app.status_message = if tmux_failures == 0 {
        format!("Killed {deleted_count} session(s)")
    } else {
        format!("Killed {deleted_count} from DB; {tmux_failures} tmux kill(s) failed")
    };

    Ok(())
}

fn refresh_sessions_and_preview(app: &mut App, store: &mut SessionStore) -> Result<()> {
    app.refresh(store)?;
    refresh_selected_preview(app);
    Ok(())
}

pub(crate) fn refresh_selected_preview(app: &mut App) {
    let client = TmuxClient::new();

    if app.show_card_preview_on_all_cards {
        for session in app.sessions.iter().cloned() {
            let session_target = SessionTarget::new(session.tmux_session_name);
            let console = capture_preview_for_role(&client, &session_target, PanelRole::Console);
            let agent = capture_preview_for_role(&client, &session_target, PanelRole::Agent);

            app.pane_preview_cache
                .insert(session.session_key, PanePreview { console, agent });
        }
        return;
    }

    let Some(session) = app.selected_session().cloned() else {
        return;
    };

    let session_target = SessionTarget::new(session.tmux_session_name);
    let console = capture_preview_for_role(&client, &session_target, PanelRole::Console);
    let agent = capture_preview_for_role(&client, &session_target, PanelRole::Agent);

    app.pane_preview_cache
        .insert(session.session_key, PanePreview { console, agent });
}

fn capture_preview_for_role(
    client: &TmuxClient,
    session_target: &SessionTarget,
    role: PanelRole,
) -> String {
    let pane_request = PaneTarget::Role(role);
    let pane_id = match client.resolve_pane_id_for(session_target, Some(&pane_request)) {
        Ok(id) => id,
        Err(error) => return format!("Preview unavailable: {error}"),
    };

    let preview_text = {
        let mut options = CapturePaneOptions::new(session_target.clone());
        options.pane = Some(PaneTarget::PaneId(pane_id.clone()));
        options.preserve_ansi = true;
        options.join_wrapped_lines = role != PanelRole::Agent;
        if role == PanelRole::Agent {
            options.start_line = None;
            options.end_line = None;
            options.include_alternate_screen = false;
        } else {
            options.start_line = Some(-220);
        }
        if role != PanelRole::Agent {
            options.include_alternate_screen = true;
        }
        match client.capture_pane(options) {
            Ok(captured) => sanitize_preview_text(captured.text),
            Err(error) => format!("Preview unavailable: {error}"),
        }
    };

    if preview_text.trim().is_empty() {
        let mut fallback = CapturePaneOptions::new(session_target.clone());
        fallback.pane = Some(PaneTarget::PaneId(pane_id));
        fallback.preserve_ansi = true;
        fallback.join_wrapped_lines = role != PanelRole::Agent;
        if role == PanelRole::Agent {
            fallback.include_alternate_screen = true;
            fallback.start_line = None;
            fallback.end_line = None;
        } else {
            fallback.start_line = Some(-500);
            fallback.include_alternate_screen = true;
        }
        return match client.capture_pane(fallback) {
            Ok(captured) => sanitize_preview_text(captured.text),
            Err(_) => preview_text,
        };
    }

    preview_text
}

fn sanitize_preview_text(text: String) -> String {
    const PATH_WARN_PREFIX: &str = "(eval):1: no such file or directory:";
    const TOOLBOX_SEGMENT: &str = "Support/JetBrains/Toolbox/scripts";

    let filtered = text
        .lines()
        .filter(|line| !(line.contains(PATH_WARN_PREFIX) && line.contains(TOOLBOX_SEGMENT)))
        .collect::<Vec<_>>();

    if filtered.is_empty() {
        text
    } else {
        filtered.join("\n")
    }
}

fn execute_footer_action(
    action: FooterHotkeyAction,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match action {
        FooterHotkeyAction::Quit => app.should_quit = true,
        FooterHotkeyAction::Navigate => {}
        FooterHotkeyAction::SidebarTab => app.next_sidebar_tab(),
        FooterHotkeyAction::Refresh => refresh_sessions_and_preview(app, store)?,
        FooterHotkeyAction::ToggleInspector => app.toggle_inspector(),
        FooterHotkeyAction::OpenActions => app.open_action_menu(),
        FooterHotkeyAction::Attach => attach_selected_session(terminal, app)?,
        FooterHotkeyAction::Spawn => {
            app.reset_spawn_form();
            app.mode = AppMode::SpawnInput;
        }
        FooterHotkeyAction::Kill => {
            if app.selected_session().is_some() {
                app.mode = AppMode::ConfirmKill;
            } else {
                app.status_message = String::from("No session selected");
            }
        }
        FooterHotkeyAction::SessionView => app.toggle_sessions_view_mode(),
        FooterHotkeyAction::FormNextField => {
            if app.mode == AppMode::SpawnInput {
                app.spawn_form.next_field();
            }
        }
        FooterHotkeyAction::FormSubmit => match app.mode {
            AppMode::ActionMenu => {
                execute_menu_action(app.selected_menu_action(), terminal, app, store)?
            }
            AppMode::ConfirmKill => {
                kill_selected_session(app, store)?;
                app.mode = AppMode::Normal;
            }
            AppMode::ConfirmKillAll => {
                kill_all_sessions(app, store)?;
                app.mode = AppMode::Normal;
            }
            AppMode::SendInput => send_console_input(app),
            AppMode::SpawnInput => spawn_session_from_input(app, store)?,
            _ => {}
        },
        FooterHotkeyAction::FormCancel => match app.mode {
            AppMode::ActionMenu => app.close_action_menu(),
            AppMode::ConfirmKill => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill canceled");
            }
            AppMode::ConfirmKillAll => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill all canceled");
            }
            AppMode::SendInput => {
                app.mode = AppMode::Normal;
                app.input_buffer.clear();
            }
            AppMode::SpawnInput => {
                app.mode = AppMode::Normal;
                app.reset_spawn_form();
            }
            _ => {}
        },
        FooterHotkeyAction::FormEditPrompt => {
            if app.mode == AppMode::SpawnInput {
                open_prompt_in_editor(terminal, app)?;
            }
        }
        FooterHotkeyAction::Confirm => match app.mode {
            AppMode::ConfirmKill => {
                kill_selected_session(app, store)?;
                app.mode = AppMode::Normal;
            }
            AppMode::ConfirmKillAll => {
                kill_all_sessions(app, store)?;
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        FooterHotkeyAction::Cancel => match app.mode {
            AppMode::ConfirmKill => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill canceled");
            }
            AppMode::ConfirmKillAll => {
                app.mode = AppMode::ActionMenu;
                app.status_message = String::from("Kill all canceled");
            }
            _ => {}
        },
        FooterHotkeyAction::PaneFocus => enter_pane_focus(app),
        FooterHotkeyAction::PaneFocusExit => {
            if app.mode == AppMode::PaneFocus {
                app.mode = AppMode::Normal;
                app.last_esc_at = None;
                app.status_message = String::from("Exited focus mode");
                refresh_selected_preview(app);
            }
        }
    }

    Ok(())
}

fn build_agent_command(
    tmux_session_name: &str,
    agent: &str,
    model: &str,
    prompt: &str,
    attach_url: Option<&str>,
    attach_session_id: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    let mut attached_to_session = false;

    if agent.eq_ignore_ascii_case("opencode") {
        let safe_name = sanitize_opencode_dir_segment(tmux_session_name);
        parts.push(format!(
            "XDG_DATA_HOME=$HOME/.local/share/tinyverse-opencode/{safe_name}"
        ));
        parts.push(format!(
            "XDG_STATE_HOME=$HOME/.local/state/tinyverse-opencode/{safe_name}"
        ));
        if let (Some(url), Some(session_id)) = (attach_url, attach_session_id) {
            parts.push(agent.to_owned());
            parts.push(String::from("attach"));
            parts.push(url.to_owned());
            parts.push(String::from("--session"));
            parts.push(session_id.to_owned());
            attached_to_session = true;
        } else {
            parts.push(agent.to_owned());
        }
    } else {
        parts.push(agent.to_owned());
    }

    if !attached_to_session && !model.is_empty() {
        parts.push(String::from("--model"));
        parts.push(shell_escape(model));
    }
    if !attached_to_session && !prompt.is_empty() {
        parts.push(String::from("--prompt"));
        parts.push(shell_escape(prompt));
    }

    parts.join(" ")
}

fn sanitize_opencode_dir_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }

    if out.is_empty() {
        String::from("default")
    } else {
        out
    }
}

fn shell_escape(value: &str) -> String {
    if value.is_empty() {
        return String::from("''");
    }
    let escaped = value.replace('\'', "'\"'\"'");
    format!("'{escaped}'")
}

fn resolve_prompt_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if Path::new(trimmed).is_file()
        && let Ok(contents) = std::fs::read_to_string(trimmed)
    {
        return contents.trim().to_owned();
    }

    trimmed.to_owned()
}

fn focus_next_chat_part(app: &mut App) -> bool {
    let keys = app
        .layout
        .chat
        .part_toggle_hitboxes
        .iter()
        .map(|hitbox| hitbox.part_key.as_str())
        .collect::<Vec<_>>();
    if keys.is_empty() {
        return false;
    }

    let current = app.chat.focused_part_key();
    let next = if let Some(current) = current {
        keys.iter()
            .position(|key| *key == current)
            .map(|index| (index + 1) % keys.len())
            .unwrap_or(0)
    } else {
        0
    };

    app.chat.set_focused_part_key(Some(keys[next].to_owned()));
    true
}

fn focus_prev_chat_part(app: &mut App) -> bool {
    let keys = app
        .layout
        .chat
        .part_toggle_hitboxes
        .iter()
        .map(|hitbox| hitbox.part_key.as_str())
        .collect::<Vec<_>>();
    if keys.is_empty() {
        return false;
    }

    let current = app.chat.focused_part_key();
    let prev = if let Some(current) = current {
        keys.iter()
            .position(|key| *key == current)
            .map(|index| {
                if index == 0 {
                    keys.len() - 1
                } else {
                    index - 1
                }
            })
            .unwrap_or(0)
    } else {
        0
    };

    app.chat.set_focused_part_key(Some(keys[prev].to_owned()));
    true
}

fn save_spawn_prefs(app: &App) {
    let prefs = TuiPrefs {
        spawn_agent: Some(app.spawn_form.agent_type.trim().to_owned()),
        spawn_model: Some(app.spawn_form.model.trim().to_owned()),
        show_card_preview_on_all_cards: Some(app.show_card_preview_on_all_cards),
    };

    let _ = prefs::save(&prefs);
}

fn open_prompt_in_editor(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let prompt_seed = app.spawn_form.prompt.trim();
    let existing_path = Path::new(prompt_seed);

    let path = if !prompt_seed.is_empty() && existing_path.is_file() {
        existing_path.to_path_buf()
    } else {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let path = std::env::temp_dir().join(format!("tinyverse_prompt_{stamp}.md"));
        let initial = if prompt_seed.is_empty() {
            String::from("# Prompt\n\n")
        } else {
            prompt_seed.to_owned()
        };
        std::fs::write(&path, initial)?;
        path
    };

    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::env::var("EDITOR")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .unwrap_or_else(|| String::from("vi"));

    restore_terminal(terminal)?;
    let status_result = Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} \"{}\"", path.display()))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    *terminal = setup_terminal()?;

    match status_result {
        Ok(status) if status.success() => {
            app.spawn_form.prompt = path.display().to_string();
            app.spawn_form.active_field = 3;
            app.status_message = format!("Prompt edited at {}", path.display());
        }
        Ok(status) => {
            app.status_message = format!("Editor exited with {:?}", status.code());
        }
        Err(error) => {
            app.status_message = format!("Failed to launch editor: {error}");
        }
    }

    Ok(())
}

fn action_menu_index_from_click(menu_rect: ratatui::layout::Rect, y: u16) -> Option<usize> {
    if y <= menu_rect.y || y >= menu_rect.bottom() {
        return None;
    }

    let row = y.saturating_sub(menu_rect.y + 1) as usize;
    let separator_row = ACTION_MENU_DANGER_SPLIT_AFTER + 1;
    if row == separator_row {
        return None;
    }
    let action_index = if row > separator_row { row - 1 } else { row };
    (action_index < MENU_ACTIONS.len()).then_some(action_index)
}

fn card_index_from_position(x: u16, y: u16, app: &App) -> Option<usize> {
    app.layout
        .card_rects
        .iter()
        .find_map(|(index, rect)| rect_contains(*rect, x, y).then_some(*index))
}

fn card_kill_index_from_position(x: u16, y: u16, app: &App) -> Option<usize> {
    app.layout
        .card_kill_rects
        .iter()
        .find_map(|(index, rect)| rect_contains(*rect, x, y).then_some(*index))
}

fn is_near_vertical_divider(x: u16, app: &App) -> bool {
    let Some(divider_x) = app.layout.divider_x else {
        return false;
    };
    x.abs_diff(divider_x) <= 1
}

fn is_near_horizontal_divider(y: u16, app: &App) -> bool {
    let Some(divider_y) = app.layout.divider_y else {
        return false;
    };
    y.abs_diff(divider_y) <= 1
}

fn update_vertical_divider_ratio(x: u16, app: &mut App) {
    let Some(body_rect) = app.layout.body_rect else {
        return;
    };
    if body_rect.width == 0 {
        return;
    }

    let relative = x.saturating_sub(body_rect.x) as f32;
    let ratio = ((relative / body_rect.width as f32) * 100.0) as u16;
    app.inspector_ratio = ratio.clamp(40, 80);
}

fn update_horizontal_divider_height(y: u16, app: &mut App) {
    let Some(body_rect) = app.layout.body_rect else {
        return;
    };
    if body_rect.height < 12 {
        return;
    }

    let pointer = y.clamp(
        body_rect.y.saturating_add(6),
        body_rect.bottom().saturating_sub(6),
    );
    let top_height = pointer.saturating_sub(body_rect.y);
    let inspector_height = body_rect.height.saturating_sub(top_height);
    app.inspector_height = inspector_height.clamp(6, body_rect.height.saturating_sub(6).max(6));
}

fn enter_pane_focus(app: &mut App) {
    if app.sidebar_tab == SidebarTab::Chat {
        app.status_message = String::from("Focus mode unavailable for Chat tab");
        return;
    }
    if app.selected_session().is_none() {
        app.status_message = String::from("No session selected");
        return;
    }
    app.mode = AppMode::PaneFocus;
    app.last_esc_at = None;
    app.status_message = format!(
        "Live: {} pane (f toggles, Esc Esc exits)",
        app.sidebar_tab.title()
    );
}

fn handle_pane_focus_key(key: KeyEvent, app: &mut App) {
    if key.code == KeyCode::Char('f') && key.modifiers.is_empty() {
        app.mode = AppMode::Normal;
        app.last_esc_at = None;
        app.status_message = String::from("Live mode off");
        refresh_selected_preview(app);
        return;
    }

    if key.code == KeyCode::Char('?') && key.modifiers.is_empty() {
        app.status_message = String::from("Focus mode: press f to toggle off, or Esc Esc");
        return;
    }

    if key.code == KeyCode::Esc {
        let now = Instant::now();
        if let Some(prev) = app.last_esc_at {
            if now.duration_since(prev).as_millis() < DOUBLE_ESC_WINDOW_MS {
                app.mode = AppMode::Normal;
                app.last_esc_at = None;
                app.status_message = String::from("Exited focus mode");
                refresh_selected_preview(app);
                return;
            }
        }
        app.last_esc_at = Some(now);
        send_tmux_key(app, "Escape");
        return;
    }

    app.last_esc_at = None;

    let has_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char(c) if has_ctrl => {
            let ctrl_char = (c as u8).wrapping_sub(b'a').wrapping_add(1);
            if ctrl_char <= 26 {
                let tmux_key = format!("C-{c}");
                send_tmux_key(app, &tmux_key);
            }
        }
        KeyCode::Char(c) => {
            send_tmux_literal(app, &c.to_string());
        }
        KeyCode::Enter => send_tmux_key(app, "Enter"),
        KeyCode::Backspace => send_tmux_key(app, "BSpace"),
        KeyCode::Delete => send_tmux_key(app, "DC"),
        KeyCode::Tab => send_tmux_key(app, "Tab"),
        KeyCode::Up => send_tmux_key(app, "Up"),
        KeyCode::Down => send_tmux_key(app, "Down"),
        KeyCode::Left => send_tmux_key(app, "Left"),
        KeyCode::Right => send_tmux_key(app, "Right"),
        KeyCode::Home => send_tmux_key(app, "Home"),
        KeyCode::End => send_tmux_key(app, "End"),
        KeyCode::PageUp => send_tmux_key(app, "PPage"),
        KeyCode::PageDown => send_tmux_key(app, "NPage"),
        _ => {}
    }

    refresh_selected_preview(app);
}

fn send_tmux_key(app: &mut App, tmux_key_name: &str) {
    let Some(session) = app.selected_session().cloned() else {
        return;
    };
    let role = match app.sidebar_tab {
        SidebarTab::Console => PanelRole::Console,
        SidebarTab::Agent => PanelRole::Agent,
        SidebarTab::Chat => return,
    };
    let client = TmuxClient::new();
    let session_target = SessionTarget::new(session.tmux_session_name);
    let pane_request = PaneTarget::Role(role);
    let pane_id = match client.resolve_pane_id_for(&session_target, Some(&pane_request)) {
        Ok(id) => id,
        Err(_) => return,
    };
    let _ = Command::new("tmux")
        .args(["send-keys", "-t", &pane_id, tmux_key_name])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

fn send_tmux_literal(app: &mut App, text: &str) {
    let Some(session) = app.selected_session().cloned() else {
        return;
    };
    let role = match app.sidebar_tab {
        SidebarTab::Console => PanelRole::Console,
        SidebarTab::Agent => PanelRole::Agent,
        SidebarTab::Chat => return,
    };
    let mut options = SendKeysOptions::new(session.tmux_session_name, text.to_owned());
    options.pane = Some(PaneTarget::Role(role));
    options.press_enter = false;
    let _ = TmuxClient::new().send_keys(options);
}

fn build_tmux_attach_args(in_tmux: bool, session: &str) -> Vec<String> {
    if in_tmux {
        return vec![
            "switch-client".to_owned(),
            "-t".to_owned(),
            session.to_owned(),
        ];
    }

    vec![
        "attach-session".to_owned(),
        "-t".to_owned(),
        session.to_owned(),
    ]
}
