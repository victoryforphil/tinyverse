use anyhow::Result;
use log::info;
use serde::Serialize;
use tinyverse_lib::tmux::SessionTarget;
use tinyverse_lib::{
    PaneSnapshot, SessionStore, TinyverseHomeSource, current_pane_id, current_session_target,
    list_pane_snapshots, reset_db_with_backup, resolve_tinyverse_paths,
};
use tinyverse_ui::{
    ActionLine, DetailSection, GuidanceLine, LabeledField, Panel, StatusBadge, StyledTable,
    SummaryFooter, Tone, default_stdout_context,
};

use super::args::{DebugCommands, DebugSelfArgs};
use crate::commands::output::render_output;

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

    let rendered = render_output(
        &report,
        args.format,
        format_table_report,
        format_text_report,
    )?;
    println!("{rendered}");

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

fn home_source_label(source: &str) -> &str {
    match source {
        "arg" => "CLI argument",
        "env" => "Environment variable",
        "repo_local" => "Repository-local",
        "cwd_local" => "CWD-local",
        "home" => "User home",
        other => other,
    }
}

fn format_text_report(report: &DebugSelfReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("in tmux: {}", report.in_tmux));
    lines.push(format!("home: {}", report.tinyverse_home));
    lines.push(format!("database: {}", report.tinyverse_db));
    lines.push(format!(
        "home source: {}",
        home_source_label(report.tinyverse_home_source)
    ));

    if let Some(session) = report.session.as_deref() {
        lines.push(format!(
            "session: {}",
            tinyverse_ui::format_display_name(session)
        ));
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

fn format_table_report(report: &DebugSelfReport) -> String {
    let context = default_stdout_context();

    // -- Environment section --
    let tmux_tone = if report.in_tmux {
        Tone::Success
    } else {
        Tone::Neutral
    };
    let tmux_label = if report.in_tmux { "yes" } else { "no" };

    let mut env_section = DetailSection::new("Environment").with_field(LabeledField::new(
        "tmux",
        StatusBadge::new(tmux_label, tmux_tone).render(&context),
    ));

    if let Some(session) = report.session.as_deref() {
        let display = tinyverse_ui::format_display_name(session);
        env_section = env_section.with_field(LabeledField::new("Session", display));
    }

    if let Some(pane) = report.current_pane.as_deref() {
        env_section = env_section.with_field(LabeledField::new("Current pane", pane));
    }

    // -- Paths section --
    let paths_section = DetailSection::new("Paths")
        .with_field(LabeledField::new("Home", report.tinyverse_home.as_str()))
        .with_field(LabeledField::new("Database", report.tinyverse_db.as_str()))
        .with_field(LabeledField::new(
            "Source",
            home_source_label(report.tinyverse_home_source),
        ));

    // -- Panes section --
    let panes_content = if report.panes.is_empty() {
        GuidanceLine::new("No panes detected (not inside a tmux session).").render(&context)
    } else {
        let mut table =
            StyledTable::new(vec!["ID", "INDEX", "TITLE", "ACTIVE"]).with_numeric_columns(&[1]);

        for pane in &report.panes {
            let active_badge = if pane.is_active {
                StatusBadge::new("yes", Tone::Success).render(&context)
            } else {
                StatusBadge::new("no", Tone::Neutral).render(&context)
            };

            table = table.with_row(vec![
                pane.pane_id.clone(),
                pane.pane_index.to_string(),
                pane.pane_title.clone(),
                active_badge,
            ]);
        }

        table.render(&context)
    };

    let panes_panel = Panel::new(panes_content)
        .with_title("Panes")
        .with_tone(Tone::Info)
        .render(&context);

    // -- Assemble outer panel --
    Panel::new(
        [
            ActionLine::new(
                "DEBUG",
                format!("Runtime snapshot ({} pane(s))", report.panes.len()),
                Tone::Info,
            )
            .render(&context),
            String::new(),
            env_section.render(&context),
            String::new(),
            paths_section.render(&context),
            String::new(),
            panes_panel,
            String::new(),
            SummaryFooter::new(format!("{} pane(s) detected", report.panes.len())).render(&context),
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: Debug Self")
    .with_tone(Tone::Info)
    .render(&context)
}

#[cfg(test)]
mod tests {
    use super::{DebugSelfReport, format_table_report, format_text_report};
    use crate::commands::output::{OutputFormat, render_output};

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
        assert!(rendered.contains("in tmux: false"));
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
            format_table_report,
            format_text_report,
        )
        .expect("json rendering should succeed");
        assert!(rendered.contains("\"in_tmux\": false"));
    }

    #[test]
    fn table_render_includes_structured_sections() {
        let report = DebugSelfReport {
            in_tmux: true,
            session: Some("tinyverse_redding".to_owned()),
            current_pane: Some("%1".to_owned()),
            tinyverse_home: "/tmp/.tinyverse".to_owned(),
            tinyverse_db: "/tmp/.tinyverse/tinyverse_sessions.sqlite3".to_owned(),
            tinyverse_home_source: "cwd_local",
            panes: Vec::new(),
        };

        let rendered = format_table_report(&report);
        assert!(rendered.contains("TinyVerse: Debug Self"));
        assert!(rendered.contains("DEBUG"));
        assert!(rendered.contains("Environment"));
        assert!(rendered.contains("Paths"));
        assert!(rendered.contains("Redding do TinyVerse // Redding"));
    }

    #[test]
    fn table_render_shows_friendly_home_source() {
        let report = DebugSelfReport {
            in_tmux: false,
            session: None,
            current_pane: None,
            tinyverse_home: "/tmp/.tinyverse".to_owned(),
            tinyverse_db: "/tmp/.tinyverse/tinyverse_sessions.sqlite3".to_owned(),
            tinyverse_home_source: "home",
            panes: Vec::new(),
        };

        let rendered = format_table_report(&report);
        assert!(rendered.contains("User home"));
    }
}
