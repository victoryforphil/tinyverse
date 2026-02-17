use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::SessionStore;
use tinyverse_lib::tmux::{SessionTarget, TmuxClient};

use super::args::KillArgs;

pub fn execute(args: KillArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let stored = store
        .find_session(&args.session)?
        .with_context(|| format!("unknown session `{}`", args.session))?;

    let target = SessionTarget::new(stored.tmux_session_name.clone());
    let client = TmuxClient::new();

    client
        .kill_session(target.clone())
        .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

    let deleted = store.delete_session_by_key(&stored.session_key)?;
    info!(
        "Killed session {} (key={}, deleted_db_row={})",
        stored.session_name, stored.session_key, deleted
    );
    Ok(())
}
