use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{ListSessionsOptions, TmuxClient};

pub fn execute() -> Result<()> {
    let client = TmuxClient::new();
    let sessions = client
        .list_sessions(ListSessionsOptions)
        .context("failed to list tmux sessions")?;

    info!(
        "CLI // Sessions // Listed sessions (meta={{\"count\":{}}})",
        sessions.len()
    );

    if sessions.is_empty() {
        println!("No sessions found.");
        return Ok(());
    }

    println!("ID\tNAME\tATTACHED\tWINDOWS");
    for session in sessions {
        println!(
            "{}\t{}\t{}\t{}",
            session.session_id, session.session_name, session.attached_clients, session.windows
        );
    }

    Ok(())
}
