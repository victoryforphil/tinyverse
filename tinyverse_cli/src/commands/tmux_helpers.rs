use std::process::Command;

use anyhow::{Context, Result, bail};
use tinyverse_lib::tmux::{PaneTarget, SessionTarget};

pub fn pane_target_from_selector(selector: Option<&str>) -> Option<PaneTarget> {
    selector
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PaneTarget::from_selector)
}

pub fn resolve_session_target(explicit_session: Option<&str>) -> Result<SessionTarget> {
    if let Some(session) = explicit_session {
        let trimmed = session.trim();
        if !trimmed.is_empty() {
            return Ok(SessionTarget::new(trimmed));
        }
    }

    if std::env::var_os("TMUX").is_none() {
        bail!("session is required outside tmux; pass --session <id-or-name>");
    }

    let output = Command::new("tmux")
        .args(["display-message", "-p", "#S"])
        .output()
        .context("failed to resolve current tmux session")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!(
            "failed to resolve current tmux session; pass --session <id-or-name> (stderr={stderr:?})"
        );
    }

    let session_name = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if session_name.is_empty() {
        bail!("resolved empty tmux session name; pass --session <id-or-name>");
    }

    Ok(SessionTarget::new(session_name))
}
