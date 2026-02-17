use std::io::IsTerminal;

use anyhow::{bail, Context, Result};
use log::info;
use tinyverse_lib::tmux::{ListSessionsOptions, SessionTarget, TmuxClient};
use tinyverse_lib::{
    resolve_required_session_key, select_required_arg, ArgSelectOption, RequiredArgSelectConfig,
    RequiredSessionSelectConfig, SessionStore,
};
use tinyverse_ui::{
    default_stdout_context, ActionLine, ErrorBlock, GuidanceLine, LabeledField, Panel,
    SummaryFooter, Tone,
};

use super::args::KillArgs;
use crate::commands::output::display_session_name;

pub fn execute(args: KillArgs) -> Result<()> {
    if args.tmux {
        return execute_tmux_mode(args);
    }

    let mut store = SessionStore::open_default()?;
    if args.all {
        return kill_all_sessions(&mut store);
    }

    let session_query = match args.session.as_deref() {
        Some(session) => session.to_owned(),
        None => resolve_session_interactively(&mut store)?,
    };

    let stored = store
        .find_session(&session_query)?
        .with_context(|| format!("unknown session `{session_query}`"))?;

    let target = SessionTarget::new(stored.tmux_session_name.clone());
    let client = TmuxClient::new();

    client
        .kill_session(target.clone())
        .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

    let deleted = store.delete_session_by_key(&stored.session_key)?;
    let display_name = display_session_name(&stored.session_name);

    info!(
        "Killed {} (key={}, deleted_db_row={})",
        display_name, stored.session_key, deleted
    );
    print_kill_summary(&display_name, &stored.session_key);

    Ok(())
}

fn execute_tmux_mode(args: KillArgs) -> Result<()> {
    let client = TmuxClient::new();

    if args.all {
        return kill_all_tmux_sessions(&client);
    }

    let session_name = match args.session.as_deref() {
        Some(session) => session.to_owned(),
        None => resolve_tmux_session_interactively(&client)?,
    };

    let target = SessionTarget::new(session_name.clone());
    client
        .kill_session(target.clone())
        .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

    info!("Killed tmux session {}", target.as_str());
    print_kill_tmux_summary(target.as_str());

    Ok(())
}

/// When session is omitted (and not --all), try interactive picker.
fn resolve_session_interactively(store: &mut SessionStore) -> Result<String> {
    const EMPTY_SESSIONS_MSG: &str = "no sessions available to kill";

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        let context = default_stdout_context();
        let block = ErrorBlock::new("Session argument required")
            .with_detail("In non-interactive mode, pass a session key/name or --all.")
            .with_guidance("Example: tinyverse kill <session>  or  tinyverse kill --all")
            .render(&context);
        eprintln!("\n{block}\n");
        bail!("session argument is required in non-interactive mode");
    }

    let result = resolve_required_session_key(
        None,
        store,
        RequiredSessionSelectConfig::new("Select a session to kill", "tinyverse kill <session>")
            .with_cancelled_message("session selection cancelled")
            .with_empty_message(EMPTY_SESSIONS_MSG),
    );

    if let Err(error) = &result {
        if error.to_string() == EMPTY_SESSIONS_MSG {
            print_kill_empty();
        }
    }

    result
}

fn kill_all_sessions(store: &mut SessionStore) -> Result<()> {
    let sessions = store.list_sessions()?;
    if sessions.is_empty() {
        print_kill_empty();
        return Ok(());
    }

    let client = TmuxClient::new();
    let total = sessions.len();
    let mut killed_names: Vec<String> = Vec::with_capacity(total);

    for session in sessions {
        let target = SessionTarget::new(session.tmux_session_name.clone());

        client
            .kill_session(target.clone())
            .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;

        store.delete_session_by_key(&session.session_key)?;
        killed_names.push(display_session_name(&session.session_name));
    }

    info!("Killed {total} TinyVerse session(s)");
    print_kill_all_summary(&killed_names);

    Ok(())
}

fn kill_all_tmux_sessions(client: &TmuxClient) -> Result<()> {
    let sessions = client
        .list_sessions(ListSessionsOptions)
        .context("failed to list tmux sessions")?;
    if sessions.is_empty() {
        print_kill_tmux_empty();
        return Ok(());
    }

    let total = sessions.len();
    let mut killed_names: Vec<String> = Vec::with_capacity(total);

    for session in sessions {
        let target = SessionTarget::new(session.session_name.clone());
        client
            .kill_session(target.clone())
            .with_context(|| format!("failed to kill session `{}`", target.as_str()))?;
        killed_names.push(target.as_str().to_owned());
    }

    info!("Killed {total} tmux session(s)");
    print_kill_tmux_all_summary(&killed_names);

    Ok(())
}

fn resolve_tmux_session_interactively(client: &TmuxClient) -> Result<String> {
    let sessions = client
        .list_sessions(ListSessionsOptions)
        .context("failed to list tmux sessions")?;

    if sessions.is_empty() {
        print_kill_tmux_empty();
        bail!("no tmux sessions available to kill");
    }

    let options: Vec<ArgSelectOption> = sessions
        .into_iter()
        .map(|session| {
            ArgSelectOption::new(
                format!(
                    "{} (attached={}, windows={})",
                    session.session_name, session.attached_clients, session.windows
                ),
                session.session_name,
            )
        })
        .collect();

    select_required_arg(
        RequiredArgSelectConfig::new(
            "session",
            "Select a tmux session to kill",
            "tinyverse kill --tmux <session>",
        )
        .with_cancelled_message("session selection cancelled"),
        options,
    )
}

fn print_kill_summary(display_name: &str, session_key: &str) {
    let context = default_stdout_context();

    let details_body = [
        LabeledField::new("Session", display_name).render(&context),
        LabeledField::new("Key", session_key).render(&context),
    ]
    .join("\n");

    let panel = Panel::new(details_body)
        .with_title("tinyverse kill")
        .with_tone(Tone::Warning)
        .render(&context);

    let header = ActionLine::new(
        "KILLED",
        format!("Terminated {display_name}"),
        Tone::Warning,
    )
    .render(&context);

    println!("{header}\n\n{panel}");
}

fn print_kill_all_summary(killed_names: &[String]) {
    let context = default_stdout_context();
    let count = killed_names.len();

    let mut body_lines: Vec<String> = killed_names
        .iter()
        .map(|name| LabeledField::new("Killed", name.as_str()).render(&context))
        .collect();

    body_lines.push(String::new());
    body_lines.push(SummaryFooter::new(format!("{count} session(s) terminated")).render(&context));

    let panel = Panel::new(body_lines.join("\n"))
        .with_title("tinyverse kill --all")
        .with_tone(Tone::Warning)
        .render(&context);

    let header = ActionLine::new(
        "KILLED",
        format!("Terminated {count} session(s)"),
        Tone::Warning,
    )
    .render(&context);

    println!("{header}\n\n{panel}");
}

fn print_kill_tmux_summary(session_name: &str) {
    let context = default_stdout_context();

    let details_body = [
        LabeledField::new("Session", session_name).render(&context),
        LabeledField::new("Source", "tmux").render(&context),
    ]
    .join("\n");

    let panel = Panel::new(details_body)
        .with_title("tinyverse kill --tmux")
        .with_tone(Tone::Warning)
        .render(&context);

    let header = ActionLine::new(
        "KILLED",
        format!("Terminated {session_name}"),
        Tone::Warning,
    )
    .render(&context);

    println!("{header}\n\n{panel}");
}

fn print_kill_tmux_all_summary(killed_names: &[String]) {
    let context = default_stdout_context();
    let count = killed_names.len();

    let mut body_lines: Vec<String> = killed_names
        .iter()
        .map(|name| LabeledField::new("Killed", name.as_str()).render(&context))
        .collect();

    body_lines.push(String::new());
    body_lines.push(SummaryFooter::new(format!("{count} session(s) terminated")).render(&context));

    let panel = Panel::new(body_lines.join("\n"))
        .with_title("tinyverse kill --tmux --all")
        .with_tone(Tone::Warning)
        .render(&context);

    let header = ActionLine::new(
        "KILLED",
        format!("Terminated {count} session(s)"),
        Tone::Warning,
    )
    .render(&context);

    println!("{header}\n\n{panel}");
}

fn print_kill_empty() {
    let context = default_stdout_context();

    let guidance =
        GuidanceLine::new("Run `tinyverse spawn` to create a session first.").render(&context);

    let panel = Panel::new(format!(
        "No TinyVerse sessions found to kill.\n\n{guidance}"
    ))
    .with_title("tinyverse kill")
    .with_tone(Tone::Info)
    .render(&context);

    let header = ActionLine::new("INFO", "No sessions to kill", Tone::Info).render(&context);

    println!("{header}\n\n{panel}");
}

fn print_kill_tmux_empty() {
    let context = default_stdout_context();

    let guidance =
        GuidanceLine::new("Start tmux sessions first, then re-run `tinyverse kill --tmux`.")
            .render(&context);

    let panel = Panel::new(format!("No tmux sessions found to kill.\n\n{guidance}"))
        .with_title("tinyverse kill --tmux")
        .with_tone(Tone::Info)
        .render(&context);

    let header = ActionLine::new("INFO", "No tmux sessions to kill", Tone::Info).render(&context);

    println!("{header}\n\n{panel}");
}
