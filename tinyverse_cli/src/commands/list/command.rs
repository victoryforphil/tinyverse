use anyhow::{Context, Result};
use log::info;
use prettytable::{Cell, Row, Table};
use serde::Serialize;
use tinyverse_lib::tmux::{ListSessionsOptions, TmuxClient};
use tinyverse_lib::{SessionStore, StoredSession};

use super::args::ListArgs;
use crate::commands::output::render_output;

#[derive(Debug, Serialize)]
struct ListReport {
    showing_all: bool,
    source_of_truth: &'static str,
    returned_sessions: usize,
    sessions: Vec<ListItem>,
}

#[derive(Debug, Serialize)]
struct ListItem {
    session_key: Option<String>,
    name: String,
    status: Option<String>,
    attached_clients: u32,
    windows: u32,
    source: &'static str,
}

pub fn execute(args: ListArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let db_sessions = store.list_sessions()?;

    let client = TmuxClient::new();
    let tmux_sessions = client
        .list_sessions(ListSessionsOptions)
        .context("failed to list tmux sessions")?;

    let mut report_rows: Vec<ListItem> = db_sessions
        .iter()
        .map(|session| {
            let tmux_match = tmux_sessions
                .iter()
                .find(|tmux| tmux.session_name == session.tmux_session_name);
            ListItem {
                session_key: Some(session.session_key.clone()),
                name: session.session_name.clone(),
                status: Some(session.status_string.clone()),
                attached_clients: tmux_match.map(|value| value.attached_clients).unwrap_or(0),
                windows: tmux_match.map(|value| value.windows).unwrap_or(0),
                source: "db",
            }
        })
        .collect();

    if args.all {
        append_unmanaged_tmux_sessions(&db_sessions, &tmux_sessions, &mut report_rows);
    }

    let report = ListReport {
        showing_all: args.all,
        source_of_truth: "tinyverse_db",
        returned_sessions: report_rows.len(),
        sessions: report_rows,
    };

    info!("Found {} session(s)", report.returned_sessions);

    let output = render_output(
        &report,
        args.format,
        format_table_report,
        format_text_report,
    )?;
    info!("{output}");

    Ok(())
}

fn format_text_report(report: &ListReport) -> String {
    if report.sessions.is_empty() {
        return "No tinyverse sessions found in database.".to_owned();
    }

    let mut lines = Vec::new();
    lines.push("KEY\tNAME\tSTATUS\tATTACHED\tWINDOWS\tSOURCE".to_owned());
    for session in &report.sessions {
        lines.push(format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            session.session_key.as_deref().unwrap_or("-"),
            session.name,
            session.status.as_deref().unwrap_or("-"),
            session.attached_clients,
            session.windows,
            session.source,
        ));
    }

    lines.push(format!(
        "source_of_truth={} shown={} include_unmanaged_tmux={}",
        report.source_of_truth, report.returned_sessions, report.showing_all
    ));

    lines.join("\n")
}

fn format_table_report(report: &ListReport) -> String {
    if report.sessions.is_empty() {
        return format_text_report(report);
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("KEY"),
        Cell::new("NAME"),
        Cell::new("STATUS"),
        Cell::new("ATTACHED"),
        Cell::new("WINDOWS"),
        Cell::new("SOURCE"),
    ]));

    for session in &report.sessions {
        table.add_row(Row::new(vec![
            Cell::new(session.session_key.as_deref().unwrap_or("-")),
            Cell::new(&session.name),
            Cell::new(session.status.as_deref().unwrap_or("-")),
            Cell::new(&session.attached_clients.to_string()),
            Cell::new(&session.windows.to_string()),
            Cell::new(session.source),
        ]));
    }

    let mut rendered = table.to_string();
    rendered.push_str(&format!(
        "\nsource_of_truth={} shown={} include_unmanaged_tmux={}",
        report.source_of_truth, report.returned_sessions, report.showing_all
    ));

    rendered
}

fn append_unmanaged_tmux_sessions(
    db_sessions: &[StoredSession],
    tmux_sessions: &[tinyverse_lib::SessionSummary],
    report_rows: &mut Vec<ListItem>,
) {
    for tmux_session in tmux_sessions {
        let is_managed = db_sessions
            .iter()
            .any(|session| session.tmux_session_name == tmux_session.session_name);
        if is_managed {
            continue;
        }

        report_rows.push(ListItem {
            session_key: None,
            name: tmux_session.session_name.clone(),
            status: None,
            attached_clients: tmux_session.attached_clients,
            windows: tmux_session.windows,
            source: "tmux_unmanaged",
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{ListItem, ListReport, format_table_report, format_text_report};

    #[test]
    fn empty_report_has_clear_message() {
        let report = ListReport {
            showing_all: false,
            source_of_truth: "tinyverse_db",
            returned_sessions: 0,
            sessions: Vec::new(),
        };

        let rendered = format_text_report(&report);
        assert!(rendered.contains("No tinyverse sessions found in database"));
    }

    #[test]
    fn table_render_includes_rows() {
        let report = ListReport {
            showing_all: true,
            source_of_truth: "tinyverse_db",
            returned_sessions: 1,
            sessions: vec![ListItem {
                session_key: Some("tinyverse-1".to_owned()),
                name: "tinyverse_1".to_owned(),
                status: Some("active".to_owned()),
                attached_clients: 0,
                windows: 1,
                source: "db",
            }],
        };

        let rendered = format_text_report(&report);
        assert!(rendered.contains("KEY\tNAME\tSTATUS\tATTACHED\tWINDOWS\tSOURCE"));
        assert!(rendered.contains("tinyverse_1"));
    }

    #[test]
    fn pretty_table_render_includes_rows() {
        let report = ListReport {
            showing_all: true,
            source_of_truth: "tinyverse_db",
            returned_sessions: 1,
            sessions: vec![ListItem {
                session_key: Some("tinyverse-1".to_owned()),
                name: "tinyverse_1".to_owned(),
                status: Some("active".to_owned()),
                attached_clients: 0,
                windows: 1,
                source: "db",
            }],
        };

        let rendered = format_table_report(&report);
        assert!(rendered.contains("KEY"));
        assert!(rendered.contains("tinyverse_1"));
    }
}
