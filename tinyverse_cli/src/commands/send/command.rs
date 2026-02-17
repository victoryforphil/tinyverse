use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{SendKeysOptions, TmuxClient};

use super::args::SendArgs;
use crate::commands::tmux_helpers::{pane_target_from_selector, resolve_session_target};

pub fn execute(args: SendArgs) -> Result<()> {
    let session = resolve_session_target(args.session.as_deref())?;
    let pane = pane_target_from_selector(args.panel.as_deref());

    let mut options = SendKeysOptions::new(session.clone(), args.command.clone());
    options.pane = pane;

    let client = TmuxClient::new();
    client
        .send_keys(options)
        .with_context(|| format!("failed to send command to session `{}`", session.as_str()))?;

    info!("Sent command to session {}", session);
    Ok(())
}
