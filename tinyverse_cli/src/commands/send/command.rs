use anyhow::{Context, Result};
use tinyverse_lib::tmux::{PaneTarget, SendKeysOptions, TmuxClient};
use tinyverse_lib::{SessionStore, pane_target_from_selector, resolve_session_target_with_store};
use tinyverse_ui::{ActionLine, LabeledField, Panel, Tone, default_stdout_context};

use super::args::SendArgs;
use crate::commands::output::display_session_name;

pub fn execute(args: SendArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let session = resolve_session_target_with_store(args.session.as_deref(), &mut store)?;
    let pane = pane_target_from_selector(args.panel.as_deref());

    let mut options = SendKeysOptions::new(session.clone(), args.command.clone());
    options.pane = pane.clone();

    let client = TmuxClient::new();
    client
        .send_keys(options)
        .with_context(|| format!("failed to send command to session `{}`", session.as_str()))?;

    let context = default_stdout_context();
    let display_name = display_session_name(session.as_str());

    let header = ActionLine::new(
        "DONE",
        format!("Sent command to {display_name}"),
        Tone::Success,
    )
    .render(&context);

    let target_label = match &pane {
        Some(PaneTarget::Role(role)) => format!("{display_name} ({role})"),
        Some(PaneTarget::PaneId(id)) => format!("{display_name} ({id})"),
        None => display_name,
    };

    let details = vec![
        LabeledField::new("Target", target_label).render(&context),
        LabeledField::new("Command", &args.command).render(&context),
    ]
    .join("\n");

    let output = Panel::new([header, String::new(), details].join("\n"))
        .with_title("TinyVerse: Send")
        .with_tone(Tone::Success)
        .render(&context);

    println!("{output}");
    Ok(())
}
