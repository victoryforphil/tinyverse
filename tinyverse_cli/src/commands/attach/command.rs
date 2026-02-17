use std::io::IsTerminal;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use log::{error, info};
use tinyverse_lib::{RequiredSessionSelectConfig, SessionStore, resolve_required_session_key};
use tinyverse_ui::{
    ActionLine, ErrorBlock, GuidanceLine, LabeledField, Panel, Tone, default_stdout_context,
};

use super::args::AttachArgs;
use crate::commands::output::display_session_name;

pub fn execute(args: AttachArgs) -> Result<()> {
    let session_query = match args.session {
        Some(session) => session,
        None => resolve_session_interactively()?,
    };

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        error!(
            "Attach needs an interactive terminal (session: {})",
            session_query
        );
        print_tty_error();
        bail!(
            "attach requires an interactive terminal (TTY); run this command directly in your shell"
        );
    }

    let mut store = SessionStore::open_default()?;
    let stored = store
        .find_session(&session_query)?
        .with_context(|| format!("unknown session `{}`", session_query))?;

    let display_name = display_session_name(&stored.session_name);
    let in_tmux = std::env::var_os("TMUX").is_some();
    let tmux_args = build_tmux_attach_args(in_tmux, &stored.tmux_session_name);
    let action = if in_tmux { "switch" } else { "attach" };

    let status = Command::new("tmux")
        .args(&tmux_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to run tmux for session `{}`", stored.session_name))?;

    if !status.success() {
        error!(
            "Attach failed for {} (exit code: {:?})",
            display_name,
            status.code()
        );
        bail!(
            "failed to attach/switch to session `{}`",
            stored.session_name
        );
    }

    info!("Attached to {} (key={})", display_name, stored.session_key);
    print_attach_summary(&display_name, &stored.session_key, action);

    Ok(())
}

/// When session is omitted, try to resolve via interactive picker.
fn resolve_session_interactively() -> Result<String> {
    const EMPTY_SESSIONS_MSG: &str =
        "no sessions available; spawn one first with `tinyverse spawn`";
    let mut store = SessionStore::open_default()?;
    let result = resolve_required_session_key(
        None,
        &mut store,
        RequiredSessionSelectConfig::new(
            "Select a session to attach",
            "tinyverse attach <session>",
        )
        .with_cancelled_message("session selection cancelled")
        .with_empty_message(EMPTY_SESSIONS_MSG),
    );

    if let Err(error) = &result {
        if error.to_string() == EMPTY_SESSIONS_MSG {
            print_no_sessions();
        }
    }

    result
}

fn print_no_sessions() {
    let context = default_stdout_context();

    let guidance =
        GuidanceLine::new("Run `tinyverse spawn <name>` to create a session.").render(&context);

    let panel = Panel::new(format!("No TinyVerse sessions found.\n\n{guidance}"))
        .with_title("tinyverse attach")
        .with_tone(Tone::Info)
        .render(&context);

    let header = ActionLine::new("INFO", "No sessions available", Tone::Info).render(&context);

    eprintln!("{header}\n\n{panel}");
}

fn print_attach_summary(display_name: &str, session_key: &str, action: &str) {
    let context = default_stdout_context();

    let details_body = [
        LabeledField::new("Session", display_name).render(&context),
        LabeledField::new("Key", session_key).render(&context),
        LabeledField::new("Action", action).render(&context),
    ]
    .join("\n");

    let panel = Panel::new(details_body)
        .with_title("tinyverse attach")
        .with_tone(Tone::Success)
        .render(&context);

    let header = ActionLine::new("OK", format!("Attached to {display_name}"), Tone::Success)
        .render(&context);

    println!("{header}\n\n{panel}");
}

fn print_tty_error() {
    let context = default_stdout_context();

    let block = ErrorBlock::new("Interactive terminal required")
        .with_detail("The attach command must run inside a terminal (TTY).")
        .with_guidance("Run this command directly in your shell, not through a pipe or script.")
        .render(&context);

    eprintln!("\n{block}\n");
}

fn build_tmux_attach_args(in_tmux: bool, session: &str) -> Vec<String> {
    if in_tmux {
        return vec![
            "switch-client".to_owned(),
            "-t".to_owned(),
            session.to_owned(),
        ];
    }

    vec![
        "attach-session".to_owned(),
        "-t".to_owned(),
        session.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::build_tmux_attach_args;

    #[test]
    fn outside_tmux_uses_attach_session() {
        let args = build_tmux_attach_args(false, "tinyverse_1");
        assert_eq!(args, vec!["attach-session", "-t", "tinyverse_1"]);
    }

    #[test]
    fn inside_tmux_uses_switch_client() {
        let args = build_tmux_attach_args(true, "tinyverse_1");
        assert_eq!(args, vec!["switch-client", "-t", "tinyverse_1"]);
    }
}
