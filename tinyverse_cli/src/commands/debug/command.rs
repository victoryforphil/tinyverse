use anyhow::Result;
use log::info;
use serde::Serialize;
use tinyverse_lib::tmux::SessionTarget;

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
    }
}

fn execute_self(args: DebugSelfArgs) -> Result<()> {
    let session = current_session_target()?;
    let current_pane = current_pane_id();
    let panes = match session.as_ref() {
        Some(session) => list_pane_snapshots(session)?,
        None => Vec::new(),
    };

    let report = build_report(session, current_pane, panes);

    info!(
        "CLI // Debug // Self inspection complete (meta={{\"in_tmux\":{},\"pane_count\":{}}})",
        report.in_tmux,
        report.panes.len()
    );

    let rendered = render_output(&report, args.format, format_text_report, format_text_report)?;
    println!("{rendered}");

    Ok(())
}

fn build_report(
    session: Option<SessionTarget>,
    current_pane: Option<String>,
    panes: Vec<PaneSnapshot>,
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
        panes: pane_reports,
    }
}

fn format_text_report(report: &DebugSelfReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("in_tmux: {}", report.in_tmux));

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
