use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::SessionStore;
use tinyverse_lib::tmux::{SessionTarget, TmuxClient};

use super::args::KillArgs;

pub fn execute(args: KillArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    if args.all {
        return kill_all_sessions(&mut store);
    }

    let session_query = args
        .session
        .as_deref()
        .context("missing session; pass a session id/name or --all")?;
    let stored = store
        .find_session(session_query)?
        .with_context(|| format!("unknown session `{session_query}`"))?;

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

fn kill_all_sessions(store: &mut SessionStore) -> Result<()> {
    let sessions = store.list_sessions()?;
    if sessions.is_empty() {
        info!("No tinyverse sessions found to kill");
        return Ok(());
    }

    let client = TmuxClient::new();
    let total = sessions.len();

    for session in sessions {
        let target = SessionTarget::new(session.tmux_session_name.clone());

        client
            .kill_session(target.clone())
            .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

        let deleted = store.delete_session_by_key(&session.session_key)?;
        info!(
            "Killed session {} (key={}, deleted_db_row={})",
            session.session_name, session.session_key, deleted
        );
    }

    info!("Killed {total} tinyverse session(s)");
    Ok(())
}
