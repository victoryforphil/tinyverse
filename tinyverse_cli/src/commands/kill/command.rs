use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{SessionTarget, TmuxClient};

use super::args::KillArgs;

pub fn execute(args: KillArgs) -> Result<()> {
    let target = SessionTarget::new(args.session.clone());
    let client = TmuxClient::new();

    client
        .kill_session(target.clone())
        .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

    info!(
        "CLI // Sessions // Killed session (meta={{\"session\":\"{}\"}})",
        args.session
    );
    println!("Killed session {}", args.session);
    Ok(())
}
