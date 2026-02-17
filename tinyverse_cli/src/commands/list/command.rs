use super::args::ListArgs;
use crate::commands::output::{display_session_name, render_output};
use anyhow::{Context, Result};
use serde::Serialize;
use tinyverse_lib::tmux::{ListSessionsOptions, TmuxClient};
use tinyverse_lib::{SessionStore, StoredSession};
use tinyverse_ui::{
    ActionLine, GuidanceLine, Panel, StripeMode, StyledTable, SummaryFooter, Tone,
    default_stdout_context,
};

const EMPTY_MESSAGE: &str = "No TinyVerse sessions found in database.";
const SPAWN_GUIDANCE: &str = "Run `tinyverse spawn <name>` to create one.";
const UNMANAGED_GUIDANCE: &str = "Use --all to include unmanaged tmux sessions.";

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
        return EMPTY_MESSAGE.to_owned();
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
    let context = default_stdout_context();
    let mut table_panel_lines = Vec::new();
    let mut outer_footer_lines = vec![
        SummaryFooter::new(format!("{} session(s)", report.returned_sessions)).render(&context),
    ];

    if report.sessions.is_empty() {
        table_panel_lines.push(EMPTY_MESSAGE.to_owned());
        outer_footer_lines.push(GuidanceLine::new(SPAWN_GUIDANCE).render(&context));
        append_unmanaged_guidance(&mut outer_footer_lines, report.showing_all, &context);
    } else {
        let mut table = StyledTable::new(vec![
            "KEY", "NAME", "STATUS", "ATTACHED", "WINDOWS", "SOURCE",
        ])
        .with_numeric_columns(&[3, 4])
        .with_stripe_mode(StripeMode::DimEvenRows);

        for session in &report.sessions {
            table = table.with_row(vec![
                session.session_key.as_deref().unwrap_or("-").to_owned(),
                display_session_name(&session.name),
                session.status.as_deref().unwrap_or("-").to_owned(),
                session.attached_clients.to_string(),
                session.windows.to_string(),
                session.source.to_owned(),
            ]);
        }

        table_panel_lines.push(table.render(&context));

        append_unmanaged_guidance(&mut outer_footer_lines, report.showing_all, &context);
    }

    let table_panel = Panel::new(table_panel_lines.join("\n"))
        .with_title("Session Table")
        .with_tone(Tone::Info)
        .render(&context);

    let mut outer_lines = vec![
        ActionLine::new(
            "INFO",
            format!("Found {} session(s)", report.returned_sessions),
            Tone::Info,
        )
        .render(&context),
        String::new(),
        table_panel,
        String::new(),
    ];
    outer_lines.extend(outer_footer_lines);

    Panel::new(outer_lines.join("\n"))
        .with_title("TinyVerse: List")
        .with_tone(Tone::Info)
        .render(&context)
}

fn append_unmanaged_guidance(
    lines: &mut Vec<String>,
    showing_all: bool,
    context: &tinyverse_ui::RenderContext<'_>,
) {
    if !showing_all {
        lines.push(GuidanceLine::new(UNMANAGED_GUIDANCE).render(context));
    }
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
        assert!(rendered.contains("No TinyVerse sessions found in database"));
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
    fn styled_table_render_includes_rows() {
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
        assert!(rendered.contains("TinyVerse: List"));
        assert!(rendered.contains("Session Table"));
        assert!(rendered.contains("INFO"));
        assert!(rendered.contains("KEY"));
        assert!(rendered.contains("1 do TinyVerse // 1"));
        assert!(rendered.contains("Summary:"));
        assert!(!rendered.contains("Use --all to include unmanaged tmux sessions."));
    }

    #[test]
    fn table_empty_state_has_guidance() {
        let report = ListReport {
            showing_all: false,
            source_of_truth: "tinyverse_db",
            returned_sessions: 0,
            sessions: Vec::new(),
        };

        let rendered = format_table_report(&report);
        assert!(rendered.contains("TinyVerse: List"));
        assert!(rendered.contains("Session Table"));
        assert!(rendered.contains("INFO"));
        assert!(rendered.contains("No TinyVerse sessions found in database."));
        assert!(rendered.contains("Run `tinyverse spawn <name>` to create one."));
        assert!(rendered.contains("Use --all to include unmanaged tmux sessions."));
    }
}
