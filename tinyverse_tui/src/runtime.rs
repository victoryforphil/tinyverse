mod chat_render;
mod events;
mod helpers;
mod render;

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
use crate::app::App;
use crate::prefs;
use crate::theme::load_theme;

const POLL_TIMEOUT: Duration = Duration::from_millis(120);

pub fn run(options: TuiRunOptions) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    store.reconcile_now()?;

    let mut app = App::new(options);
    app.theme = load_theme();
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

    while !app.should_quit {
        terminal.draw(|frame| render::render_frame(frame, app))?;

        if event::poll(POLL_TIMEOUT)? {
            let ev = event::read()?;
            events::handle_event(ev, terminal, app, store)?;
        }

        events::refresh_chat_bridge(app, false);

        if Instant::now() >= next_refresh_at {
            app.refresh(store)?;
            events::refresh_selected_preview(app);
            next_refresh_at = Instant::now() + app.options.refresh_interval;
        }
    }

    Ok(())
}
