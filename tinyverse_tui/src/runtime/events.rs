use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tinyverse_lib::{
    CapturePaneOptions, CreateSessionInput, PaneTarget, PanelRole, SendKeysOptions, SessionStore,
    SessionTarget, SpawnSessionOptions, TmuxClient, resolve_session_name,
};

use crate::app::{App, AppMode, FooterHotkeyAction, MENU_ACTIONS, MenuAction, PanePreview};
use crate::prefs::{self, TuiPrefs};

use super::helpers::rect_contains;
use super::render;
use super::{restore_terminal, setup_terminal};

pub(crate) fn handle_event(
    event: Event,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            handle_key_event(key.code, terminal, app, store)?
        }
        Event::Mouse(mouse) => handle_mouse_event(mouse, terminal, app, store)?,
        Event::Resize(_, _) => {
            app.layout.card_rects.clear();
        }
        _ => {}
    }

    Ok(())
}

fn handle_key_event(
    key: KeyCode,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    store: &mut SessionStore,
) -> Result<()> {
    match app.mode {
        AppMode::Normal => match key {
            KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Left | KeyCode::Char('h') => {
                app.select_prev();
                refresh_selected_preview(app);
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Right | KeyCode::Char('l') => {
                app.select_next();
                refresh_selected_preview(app);
            }
            KeyCode::Char('r') => refresh_sessions_and_preview(app, store)?,
            KeyCode::Char('i') | KeyCode::Tab => app.toggle_inspector(),
            KeyCode::Enter => app.open_action_menu(),
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
        AppMode::ActionMenu => match key {
            KeyCode::Esc | KeyCode::Char('q') => app.close_action_menu(),
            KeyCode::Up | KeyCode::Char('k') => app.action_menu_prev(),
            KeyCode::Down | KeyCode::Char('j') => app.action_menu_next(),
            KeyCode::Enter => {
                execute_menu_action(app.selected_menu_action(), terminal, app, store)?
            }
            KeyCode::Char(c) => {
                if let Some(index) = digit_to_index(c) {
                    if let Some(action) = MENU_ACTIONS.get(index).copied() {
                        execute_menu_action(action, terminal, app, store)?;
                    }
                }
            }
            _ => {}
        },
        AppMode::ConfirmKill => match key {
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
        AppMode::ConfirmKillAll => match key {
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
        AppMode::SendInput => match key {
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
        AppMode::SpawnInput => match key {
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
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(menu_rect) = app.layout.action_menu_rect {
                if rect_contains(menu_rect, x, y) {
                    if let Some(index) = action_menu_index_from_click(menu_rect, y) {
                        app.action_menu_index = index;
                        execute_menu_action(MENU_ACTIONS[index], terminal, app, store)?;
                    }
                } else {
                    app.close_action_menu();
                }
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

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if app.inspector_visible && is_near_divider(x, app) {
                app.dragging_divider = true;
                return Ok(());
            }
            if let Some(index) = card_index_from_position(x, y, app) {
                app.selected_index = index;
                refresh_selected_preview(app);
            }
        }
        MouseEventKind::Down(MouseButton::Right) => {
            if let Some(index) = card_index_from_position(x, y, app) {
                app.selected_index = index;
                refresh_selected_preview(app);
                app.mode = AppMode::ActionMenu;
                app.action_menu_index = 0;
                app.action_menu_anchor = Some((x, y));
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if app.dragging_divider {
                update_divider_ratio(x, app);
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            app.dragging_divider = false;
        }
        MouseEventKind::ScrollDown => app.select_next(),
        MouseEventKind::ScrollUp => app.select_prev(),
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
    let mut spawn_options = SpawnSessionOptions::new(&tmux_session_name);
    let resolved_prompt = resolve_prompt_input(&prompt);
    spawn_options.agent_command = Some(build_agent_command(&agent_type, &model, &resolved_prompt));
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
    let Some(session) = app.selected_session().cloned() else {
        return;
    };

    let session_target = SessionTarget::new(session.tmux_session_name);

    let console = {
        let mut options = CapturePaneOptions::new(session_target.clone());
        options.pane = Some(PaneTarget::Role(PanelRole::Console));
        options.start_line = Some(-40);
        match TmuxClient::new().capture_pane(options) {
            Ok(captured) => captured.text,
            Err(error) => format!("Preview unavailable: {error}"),
        }
    };

    let agent = {
        let mut options = CapturePaneOptions::new(session_target);
        options.pane = Some(PaneTarget::Role(PanelRole::Agent));
        options.start_line = Some(-40);
        match TmuxClient::new().capture_pane(options) {
            Ok(captured) => captured.text,
            Err(error) => format!("Preview unavailable: {error}"),
        }
    };

    app.pane_preview_cache
        .insert(session.session_key, PanePreview { console, agent });
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
    }

    Ok(())
}

fn build_agent_command(agent: &str, model: &str, prompt: &str) -> String {
    let mut parts = vec![agent.to_owned()];

    if !model.is_empty() {
        parts.push(String::from("--model"));
        parts.push(shell_escape(model));
    }
    if !prompt.is_empty() {
        parts.push(String::from("--prompt"));
        parts.push(shell_escape(prompt));
    }

    parts.join(" ")
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

fn save_spawn_prefs(app: &App) {
    let prefs = TuiPrefs {
        spawn_agent: Some(app.spawn_form.agent_type.trim().to_owned()),
        spawn_model: Some(app.spawn_form.model.trim().to_owned()),
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

fn digit_to_index(ch: char) -> Option<usize> {
    if !ch.is_ascii_digit() {
        return None;
    }

    let digit = ch.to_digit(10)?;
    if digit == 0 {
        return None;
    }

    Some((digit - 1) as usize)
}

fn action_menu_index_from_click(menu_rect: ratatui::layout::Rect, y: u16) -> Option<usize> {
    if y <= menu_rect.y || y >= menu_rect.bottom() {
        return None;
    }

    let row = y.saturating_sub(menu_rect.y + 1) as usize;
    if row < MENU_ACTIONS.len() {
        Some(row)
    } else {
        None
    }
}

fn card_index_from_position(x: u16, y: u16, app: &App) -> Option<usize> {
    app.layout
        .card_rects
        .iter()
        .find_map(|(index, rect)| rect_contains(*rect, x, y).then_some(*index))
}

fn is_near_divider(x: u16, app: &App) -> bool {
    let Some(divider_x) = app.layout.divider_x else {
        return false;
    };
    x.abs_diff(divider_x) <= 1
}

fn update_divider_ratio(x: u16, app: &mut App) {
    let Some(body_rect) = app.layout.body_rect else {
        return;
    };
    if body_rect.width == 0 {
        return;
    }

    let relative = x.saturating_sub(body_rect.x) as f32;
    let ratio = ((relative / body_rect.width as f32) * 100.0) as u16;
    app.inspector_ratio = ratio.clamp(45, 85);
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
