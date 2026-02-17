mod events;
mod helpers;
mod render;

use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tinyverse_lib::SessionStore;

use crate::app::App;
use crate::TuiRunOptions;

const POLL_TIMEOUT: Duration = Duration::from_millis(120);

pub fn run(options: TuiRunOptions) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    store.reconcile_now()?;

    let mut app = App::new(options);
    app.refresh(&mut store)?;

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

        if Instant::now() >= next_refresh_at {
            app.refresh(store)?;
            next_refresh_at = Instant::now() + app.options.refresh_interval;
        }
    }

    Ok(())
}
