use anyhow::{Context, Result};
use log::info;
use prettytable::{Cell, Row, Table};
use serde::Serialize;
use tinyverse_lib::tmux::{ListSessionsOptions, TmuxClient};

use super::args::ListArgs;
use crate::commands::output::render_output;

const TINYVERSE_SESSION_PREFIX: &str = "tinyverse_";

#[derive(Debug, Serialize)]
struct ListReport {
    showing_all: bool,
    total_sessions: usize,
    returned_sessions: usize,
    sessions: Vec<ListItem>,
}

#[derive(Debug, Serialize)]
struct ListItem {
    id: String,
    name: String,
    attached_clients: u32,
    windows: u32,
}

pub fn execute(args: ListArgs) -> Result<()> {
    let client = TmuxClient::new();
    let mut sessions = client
        .list_sessions(ListSessionsOptions)
        .context("failed to list tmux sessions")?;

    let total_sessions = sessions.len();
    if !args.all {
        sessions.retain(|session| session.session_name.starts_with(TINYVERSE_SESSION_PREFIX));
    }

    let report = ListReport {
        showing_all: args.all,
        total_sessions,
        returned_sessions: sessions.len(),
        sessions: sessions
            .into_iter()
            .map(|session| ListItem {
                id: session.session_id,
                name: session.session_name,
                attached_clients: session.attached_clients,
                windows: session.windows,
            })
            .collect(),
    };

    info!(
        "CLI // Sessions // Listed sessions (meta={{\"showing_all\":{},\"count\":{},\"total\":{}}})",
        report.showing_all, report.returned_sessions, report.total_sessions
    );

    let output = render_output(
        &report,
        args.format,
        format_table_report,
        format_text_report,
    )?;
    println!("{output}");

    Ok(())
}

fn format_text_report(report: &ListReport) -> String {
    if report.sessions.is_empty() {
        return if report.showing_all {
            "No sessions found.".to_owned()
        } else {
            format!(
                "No tinyverse sessions found (prefix={TINYVERSE_SESSION_PREFIX}). Use --all to include every tmux session."
            )
        };
    }

    let mut lines = Vec::new();
    lines.push("ID\tNAME\tATTACHED\tWINDOWS".to_owned());
    for session in &report.sessions {
        lines.push(format!(
            "{}\t{}\t{}\t{}",
            session.id, session.name, session.attached_clients, session.windows
        ));
    }

    if !report.showing_all {
        lines.push(format!(
            "Filtered to tinyverse sessions only (prefix={TINYVERSE_SESSION_PREFIX}, shown={}, total={})",
            report.returned_sessions, report.total_sessions
        ));
    }

    lines.join("\n")
}

fn format_table_report(report: &ListReport) -> String {
    if report.sessions.is_empty() {
        return format_text_report(report);
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("NAME"),
        Cell::new("ATTACHED"),
        Cell::new("WINDOWS"),
    ]));

    for session in &report.sessions {
        table.add_row(Row::new(vec![
            Cell::new(&session.id),
            Cell::new(&session.name),
            Cell::new(&session.attached_clients.to_string()),
            Cell::new(&session.windows.to_string()),
        ]));
    }

    let mut rendered = table.to_string();
    if !report.showing_all {
        rendered.push_str(&format!(
            "\nFiltered to tinyverse sessions only (prefix={TINYVERSE_SESSION_PREFIX}, shown={}, total={})",
            report.returned_sessions, report.total_sessions
        ));
    }

    rendered
}

#[cfg(test)]
mod tests {
    use super::{
        format_table_report, format_text_report, ListItem, ListReport, TINYVERSE_SESSION_PREFIX,
    };

    #[test]
    fn empty_filtered_text_mentions_all_flag() {
        let report = ListReport {
            showing_all: false,
            total_sessions: 2,
            returned_sessions: 0,
            sessions: Vec::new(),
        };

        let rendered = format_text_report(&report);
        assert!(rendered.contains("--all"));
        assert!(rendered.contains(TINYVERSE_SESSION_PREFIX));
    }

    #[test]
    fn table_render_includes_rows() {
        let report = ListReport {
            showing_all: true,
            total_sessions: 1,
            returned_sessions: 1,
            sessions: vec![ListItem {
                id: "$1".to_owned(),
                name: "tinyverse_1".to_owned(),
                attached_clients: 0,
                windows: 1,
            }],
        };

        let rendered = format_text_report(&report);
        assert!(rendered.contains("ID\tNAME\tATTACHED\tWINDOWS"));
        assert!(rendered.contains("tinyverse_1"));
    }

    #[test]
    fn pretty_table_render_includes_rows() {
        let report = ListReport {
            showing_all: true,
            total_sessions: 1,
            returned_sessions: 1,
            sessions: vec![ListItem {
                id: "$1".to_owned(),
                name: "tinyverse_1".to_owned(),
                attached_clients: 0,
                windows: 1,
            }],
        };

        let rendered = format_table_report(&report);
        assert!(rendered.contains("ID"));
        assert!(rendered.contains("tinyverse_1"));
    }
}
