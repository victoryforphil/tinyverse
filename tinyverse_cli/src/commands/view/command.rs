use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::SessionStore;
use tinyverse_lib::tmux::{CapturePaneOptions, TmuxClient};

use super::args::ViewArgs;
use crate::commands::tmux_helpers::{pane_target_from_selector, resolve_session_target_with_store};

pub fn execute(args: ViewArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let session = resolve_session_target_with_store(args.session.as_deref(), &mut store)?;
    let pane = pane_target_from_selector(args.panel.as_deref());

    let mut options = CapturePaneOptions::new(session.clone());
    options.pane = pane;
    options.start_line = Some(-500);

    let client = TmuxClient::new();
    let captured = client
        .capture_pane(options)
        .with_context(|| format!("failed to capture pane for session `{}`", session.as_str()))?;

    info!(
        "Captured pane {} from session {}",
        captured.pane_id, captured.session
    );

    info!("{}", captured.text);
    Ok(())
}
