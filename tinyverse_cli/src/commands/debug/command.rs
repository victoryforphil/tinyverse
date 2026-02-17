use anyhow::Result;
use log::info;
use serde::Serialize;
use tinyverse_lib::tmux::SessionTarget;
use tinyverse_lib::{
    reset_db_with_backup, resolve_tinyverse_paths, SessionStore, TinyverseHomeSource,
};

use super::args::{DebugCommands, DebugSelfArgs};
use crate::commands::output::render_output;
use crate::commands::tmux_helpers::{
    current_pane_id, current_session_target, list_pane_snapshots, PaneSnapshot,
};

#[derive(Debug, Serialize)]
struct DebugSelfReport {
    in_tmux: bool,
    session: Option<String>,
    current_pane: Option<String>,
    tinyverse_home: String,
    tinyverse_db: String,
    tinyverse_home_source: &'static str,
    panes: Vec<DebugPane>,
}

#[derive(Debug, Serialize)]
struct DebugPane {
    pane_id: String,
    pane_index: u32,
    pane_title: String,
    is_active: bool,
}

pub fn execute(command: DebugCommands) -> Result<()> {
    match command {
        DebugCommands::SelfInfo(args) => execute_self(args),
        DebugCommands::ResetDb => execute_reset_db(),
    }
}

fn execute_reset_db() -> Result<()> {
    let store = SessionStore::open_default()?;
    let report = reset_db_with_backup(store.paths())?;

    if let Some(path) = report.backup_path.as_ref() {
        info!("Database backup created at {}", path.display());
    } else {
        info!("No existing database found; created a fresh database");
    }

    info!("Database reset at {}", report.database_path.display());
    Ok(())
}

fn execute_self(args: DebugSelfArgs) -> Result<()> {
    let session = current_session_target()?;
    let current_pane = current_pane_id();
    let panes = match session.as_ref() {
        Some(session) => list_pane_snapshots(session)?,
        None => Vec::new(),
    };
    let tinyverse_paths = resolve_tinyverse_paths(None)?;

    let report = build_report(session, current_pane, panes, tinyverse_paths);

    info!("Collected debug info ({} pane(s))", report.panes.len());

    let rendered = render_output(&report, args.format, format_text_report, format_text_report)?;
    info!("{rendered}");

    Ok(())
}

fn build_report(
    session: Option<SessionTarget>,
    current_pane: Option<String>,
    panes: Vec<PaneSnapshot>,
    tinyverse_paths: tinyverse_lib::TinyversePaths,
) -> DebugSelfReport {
    let pane_reports = panes
        .into_iter()
        .map(|pane| DebugPane {
            pane_id: pane.pane_id,
            pane_index: pane.pane_index,
            pane_title: pane.pane_title,
            is_active: pane.is_active,
        })
        .collect();

    DebugSelfReport {
        in_tmux: session.is_some(),
        session: session.map(|value| value.to_string()),
        current_pane,
        tinyverse_home: tinyverse_paths.home_dir.display().to_string(),
        tinyverse_db: tinyverse_paths.db_path.display().to_string(),
        tinyverse_home_source: match tinyverse_paths.source {
            TinyverseHomeSource::ArgOverride => "arg",
            TinyverseHomeSource::EnvOverride => "env",
            TinyverseHomeSource::RepoLocal => "repo_local",
            TinyverseHomeSource::CwdLocal => "cwd_local",
            TinyverseHomeSource::UserHome => "home",
        },
        panes: pane_reports,
    }
}

fn format_text_report(report: &DebugSelfReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("in_tmux: {}", report.in_tmux));
    lines.push(format!("tinyverse_home: {}", report.tinyverse_home));
    lines.push(format!("tinyverse_db: {}", report.tinyverse_db));
    lines.push(format!(
        "tinyverse_home_source: {}",
        report.tinyverse_home_source
    ));

    if let Some(session) = report.session.as_deref() {
        lines.push(format!("session: {session}"));
    }

    if let Some(current_pane) = report.current_pane.as_deref() {
        lines.push(format!("current_pane: {current_pane}"));
    }

    if report.panes.is_empty() {
        lines.push("panes: none".to_owned());
        return lines.join("\n");
    }

    lines.push("panes:".to_owned());
    for pane in &report.panes {
        lines.push(format!(
            "- id={} index={} title={} active={}",
            pane.pane_id, pane.pane_index, pane.pane_title, pane.is_active
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{format_text_report, DebugSelfReport};
    use crate::commands::output::{render_output, OutputFormat};

    #[test]
    fn text_render_contains_expected_fields() {
        let report = DebugSelfReport {
            in_tmux: false,
            session: None,
            current_pane: None,
            tinyverse_home: "/tmp/.tinyverse".to_owned(),
            tinyverse_db: "/tmp/.tinyverse/tinyverse_sessions.sqlite3".to_owned(),
            tinyverse_home_source: "cwd_local",
            panes: Vec::new(),
        };

        let rendered = format_text_report(&report);
        assert!(rendered.contains("in_tmux: false"));
        assert!(rendered.contains("panes: none"));
    }

    #[test]
    fn shared_output_renderer_works_for_json() {
        let report = DebugSelfReport {
            in_tmux: false,
            session: None,
            current_pane: None,
            tinyverse_home: "/tmp/.tinyverse".to_owned(),
            tinyverse_db: "/tmp/.tinyverse/tinyverse_sessions.sqlite3".to_owned(),
            tinyverse_home_source: "cwd_local",
            panes: Vec::new(),
        };

        let rendered = render_output(
            &report,
            OutputFormat::Json,
            format_text_report,
            format_text_report,
        )
        .expect("json rendering should succeed");
        assert!(rendered.contains("\"in_tmux\": false"));
    }
}
