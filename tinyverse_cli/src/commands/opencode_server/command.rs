use anyhow::Result;
use log::{debug, info, warn};
use tinyverse_lib::SessionStore;
use tinyverse_ui::{default_stdout_context, ActionLine, DetailSection, LabeledField, Panel, Tone};

use super::args::{OpencodeServerArgs, OpencodeServerCommands};
use crate::commands::config::store;
use crate::opencode_service::{
    ensure_managed_opencode_service, lookup_managed_opencode_service,
    restart_managed_opencode_service, service_is_reachable, tmux_session_is_live,
};

pub fn execute(args: OpencodeServerArgs) -> Result<()> {
    match args.command.unwrap_or(OpencodeServerCommands::Status) {
        OpencodeServerCommands::Status => status(),
        OpencodeServerCommands::Ensure => ensure(),
        OpencodeServerCommands::Restart => restart(),
    }
}

fn status() -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let saved = lookup_managed_opencode_service(&mut store)?;
    let context = default_stdout_context();

    if let Some(saved) = saved {
        let reachable = service_is_reachable(&saved);
        let tmux_live = tmux_session_is_live(&saved.tmux_session_name);
        debug!(
            "managed opencode server status checked (reachable={}, tmux_live={}, base_url={})",
            reachable, tmux_live, saved.base_url
        );

        let details = DetailSection::new("Managed Service")
            .with_field(LabeledField::new("Provider", saved.provider_key))
            .with_field(LabeledField::new("Base URL", saved.base_url))
            .with_field(LabeledField::new("Hostname", saved.hostname))
            .with_field(LabeledField::new("Port", saved.port.to_string()))
            .with_field(LabeledField::new("tmux Session", saved.tmux_session_name))
            .with_field(LabeledField::new(
                "tmux Pane",
                saved.tmux_pane_id.unwrap_or_else(|| "<unset>".to_owned()),
            ))
            .with_field(LabeledField::new("Reachable", reachable.to_string()))
            .with_field(LabeledField::new("tmux Live", tmux_live.to_string()))
            .render(&context);

        let tone = if reachable && tmux_live {
            Tone::Success
        } else {
            Tone::Warning
        };
        let header = ActionLine::new("INFO", "Managed OpenCode server", tone).render(&context);
        let panel = Panel::new(format!("{header}\n\n{details}"))
            .with_title("TinyVerse: OpenCode Server")
            .with_tone(tone)
            .render(&context);
        println!("{panel}");
    } else {
        warn!("managed opencode server record not found in database");
        let header = ActionLine::new(
            "WARN",
            "No managed OpenCode server record found",
            Tone::Warning,
        )
        .render(&context);
        let panel = Panel::new(header)
            .with_title("TinyVerse: OpenCode Server")
            .with_tone(Tone::Warning)
            .render(&context);
        println!("{panel}");
    }

    Ok(())
}

fn ensure() -> Result<()> {
    let config = store::load()?;
    let mut session_store = SessionStore::open_default()?;
    let service = ensure_managed_opencode_service(&mut session_store, &config)?;
    info!("ensure command completed for managed opencode service");
    render_ensure_result(service.as_ref(), "Ensured managed OpenCode server")
}

fn restart() -> Result<()> {
    let config = store::load()?;
    let mut session_store = SessionStore::open_default()?;
    let service = restart_managed_opencode_service(&mut session_store, &config)?;
    info!("restart command completed for managed opencode service");
    render_ensure_result(service.as_ref(), "Restarted managed OpenCode server")
}

fn render_ensure_result(
    service: Option<&crate::opencode_service::ManagedOpencodeService>,
    title: &str,
) -> Result<()> {
    let context = default_stdout_context();
    let Some(service) = service else {
        let panel = Panel::new(
            ActionLine::new(
                "INFO",
                "Managed OpenCode server disabled in config",
                Tone::Info,
            )
            .render(&context),
        )
        .with_title("TinyVerse: OpenCode Server")
        .with_tone(Tone::Info)
        .render(&context);
        println!("{panel}");
        return Ok(());
    };

    let details = DetailSection::new("Managed Service")
        .with_field(LabeledField::new("Base URL", service.base_url.clone()))
        .with_field(LabeledField::new("Hostname", service.hostname.clone()))
        .with_field(LabeledField::new("Port", service.port.to_string()))
        .with_field(LabeledField::new(
            "tmux Session",
            service.tmux_session_name.clone(),
        ))
        .render(&context);
    let panel = Panel::new(
        [
            ActionLine::new("OK", title, Tone::Success).render(&context),
            String::new(),
            details,
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: OpenCode Server")
    .with_tone(Tone::Success)
    .render(&context);
    println!("{panel}");
    Ok(())
}
