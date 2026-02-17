mod chat_render;
mod events;
mod render;
mod session_tree_render;

use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tinyverse_lib::SessionStore;

use crate::TuiRunOptions;
use crate::app::{App, AppMode};
use crate::logger::{init_logger, log_line, log_path};
use crate::prefs;
use crate::theme::load_theme;

const POLL_TIMEOUT: Duration = Duration::from_millis(120);
const PANE_FOCUS_REFRESH_INTERVAL: Duration = Duration::from_millis(300);
const UI_TICK_INTERVAL: Duration = Duration::from_secs(1);

pub fn run(options: TuiRunOptions) -> Result<()> {
    init_logger();
    if let Some(path) = log_path() {
        log_line(&format!("log file: {}", path.display()));
    }

    let mut store = SessionStore::open_default()?;
    store.reconcile_now()?;

    let theme_selector = options.theme.clone();
    let mut app = App::new(options);
    app.theme = load_theme(theme_selector.as_deref());
    let (repo_name, git_branch) = detect_git_metadata();
    app.repo_name = repo_name;
    app.git_branch = git_branch;
    if let Ok(saved_prefs) = prefs::load() {
        saved_prefs.apply_to_spawn_form(&mut app.spawn_form);
        saved_prefs.apply_to_app(&mut app);
    }
    app.refresh(&mut store)?;
    events::refresh_selected_preview(&mut app);
    events::refresh_chat_bridge(&mut app, true);

    let mut terminal = setup_terminal()?;
    let run_result = run_loop(&mut terminal, &mut store, &mut app);
    let restore_result = restore_terminal(&mut terminal);

    match (run_result, restore_result) {
        (Err(run_error), Ok(())) => Err(run_error),
        (Ok(()), Err(restore_error)) => Err(restore_error),
        (Err(run_error), Err(restore_error)) => {
            Err(run_error.context(format!("also failed to restore terminal: {restore_error}")))
        }
        (Ok(()), Ok(())) => Ok(()),
    }
}

fn detect_git_metadata() -> (Option<String>, Option<String>) {
    let repo_name = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .and_then(|output| {
            if !output.status.success() {
                return None;
            }
            let top = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if top.is_empty() {
                return None;
            }
            PathBuf::from(top)
                .file_name()
                .map(|value| value.to_string_lossy().to_string())
        });

    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if !output.status.success() {
                return None;
            }
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if branch.is_empty() || branch == "HEAD" {
                None
            } else {
                Some(branch)
            }
        });

    (repo_name, git_branch)
}

pub(super) fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub(super) fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    store: &mut SessionStore,
    app: &mut App,
) -> Result<()> {
    let mut next_refresh_at = Instant::now() + app.options.refresh_interval;
    let mut next_pane_focus_refresh_at = Instant::now() + PANE_FOCUS_REFRESH_INTERVAL;
    let mut next_ui_tick_at = Instant::now() + UI_TICK_INTERVAL;
    let mut needs_draw = true;

    while !app.should_quit {
        let now = Instant::now();
        if needs_draw || now >= next_ui_tick_at {
            terminal.draw(|frame| render::render_frame(frame, app))?;
            needs_draw = false;
            next_ui_tick_at = Instant::now() + UI_TICK_INTERVAL;
        }

        let mut poll_timeout = POLL_TIMEOUT;
        let until_ui_tick = next_ui_tick_at.saturating_duration_since(Instant::now());
        if until_ui_tick < poll_timeout {
            poll_timeout = until_ui_tick;
        }

        if event::poll(poll_timeout)? {
            let ev = event::read()?;
            events::handle_event(ev, terminal, app, store)?;
            needs_draw = true;
        }

        if app.mode == AppMode::PaneFocus && Instant::now() >= next_pane_focus_refresh_at {
            events::refresh_selected_preview(app);
            next_pane_focus_refresh_at = Instant::now() + PANE_FOCUS_REFRESH_INTERVAL;
            needs_draw = true;
        }

        if events::refresh_chat_bridge(app, false) {
            needs_draw = true;
        }

        if Instant::now() >= next_refresh_at {
            app.refresh(store)?;
            events::refresh_selected_preview(app);
            next_refresh_at = Instant::now() + app.options.refresh_interval;
            needs_draw = true;
        }
    }

    Ok(())
}
