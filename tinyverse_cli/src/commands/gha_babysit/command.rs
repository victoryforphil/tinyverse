use anyhow::Result;
use serde::Serialize;

use super::args::GhaBabysitArgs;
use crate::commands::output::render_output;

#[derive(Debug, Serialize)]
struct GhaBabysitReport {
    objective: &'static str,
    branch: String,
    max_attempts: u8,
    workflow: Vec<WorkflowStep>,
    pty_watch_tip: String,
    escalation_rule: &'static str,
}

#[derive(Debug, Serialize)]
struct WorkflowStep {
    id: u8,
    title: &'static str,
    command: String,
    purpose: &'static str,
}

pub fn execute(args: GhaBabysitArgs) -> Result<()> {
    let output_format = args.format;
    let report = build_report(args);
    let rendered = render_output(
        &report,
        output_format,
        format_table_report,
        format_text_report,
    )?;
    println!("{rendered}");
    Ok(())
}

fn build_report(args: GhaBabysitArgs) -> GhaBabysitReport {
    let run_ref = args
        .run_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| "<run-id>".to_owned());

    let workflow = vec![
        WorkflowStep {
            id: 1,
            title: "Discover latest failed run",
            command: format!("gh run list --branch {} --limit 10", args.branch),
            purpose: "Identify the newest failed/in-progress CI run and capture run id.",
        },
        WorkflowStep {
            id: 2,
            title: "Extract failing logs",
            command: format!("gh run view {run_ref} --log-failed"),
            purpose: "Pull exact failing command and stack/error lines.",
        },
        WorkflowStep {
            id: 3,
            title: "Fix and verify locally",
            command: "cargo build -p tinyverse_cli && cargo test -p tinyverse_tui".to_owned(),
            purpose: "Validate the targeted fix before pushing.",
        },
        WorkflowStep {
            id: 4,
            title: "Commit and push",
            command: "git add <files> && git commit -m \"fix: <ci issue>\" && git push origin main"
                .to_owned(),
            purpose: "Create a minimal fix commit and trigger a new CI run.",
        },
        WorkflowStep {
            id: 5,
            title: "Watch new run via PTY",
            command: "gh run watch <new-run-id>".to_owned(),
            purpose: "Stream live workflow status without command timeout interruptions.",
        },
        WorkflowStep {
            id: 6,
            title: "Repeat until green",
            command: format!("Repeat steps 2-5 up to {} attempts", args.max_attempts),
            purpose: "Iterate quickly until all jobs pass or escalation is needed.",
        },
    ];

    GhaBabysitReport {
        objective: "Stabilize GitHub Actions by iterating log -> fix -> push -> watch",
        branch: args.branch,
        max_attempts: args.max_attempts,
        workflow,
        pty_watch_tip: "Use an interactive PTY session for `gh run watch` to avoid CLI timeout and keep live output."
            .to_owned(),
        escalation_rule:
            "Escalate after repeated failures in the same area (dependency, infra, or secrets) instead of blind retries.",
    }
}

fn format_text_report(report: &GhaBabysitReport) -> String {
    let mut lines = vec![
        format!("objective: {}", report.objective),
        format!("branch: {}", report.branch),
        format!("max_attempts: {}", report.max_attempts),
        "workflow:".to_owned(),
    ];

    for step in &report.workflow {
        lines.push(format!(
            "- [{}] {} :: {} :: {}",
            step.id, step.title, step.command, step.purpose
        ));
    }

    lines.push(format!("pty_watch_tip: {}", report.pty_watch_tip));
    lines.push(format!("escalation_rule: {}", report.escalation_rule));

    lines.join("\n")
}

fn format_table_report(report: &GhaBabysitReport) -> String {
    let mut lines = vec![
        "TinyVerse: GHA Babysit".to_owned(),
        format!("Objective: {}", report.objective),
        format!("Branch: {}", report.branch),
        format!("Max attempts: {}", report.max_attempts),
        String::new(),
    ];

    for step in &report.workflow {
        lines.push(format!("{}. {}", step.id, step.title));
        lines.push(format!("   cmd: {}", step.command));
        lines.push(format!("   why: {}", step.purpose));
    }

    lines.push(String::new());
    lines.push(format!("PTY tip: {}", report.pty_watch_tip));
    lines.push(format!("Escalate: {}", report.escalation_rule));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{GhaBabysitArgs, build_report, format_table_report, format_text_report};

    #[test]
    fn includes_targeted_run_when_provided() {
        let report = build_report(GhaBabysitArgs {
            branch: "main".to_owned(),
            run_id: Some(1234),
            max_attempts: 3,
            format: crate::commands::output::OutputFormat::Text,
        });

        let rendered = format_text_report(&report);
        assert!(rendered.contains("gh run view 1234 --log-failed"));
    }

    #[test]
    fn table_render_mentions_pty_tip() {
        let report = build_report(GhaBabysitArgs {
            branch: "main".to_owned(),
            run_id: None,
            max_attempts: 5,
            format: crate::commands::output::OutputFormat::Table,
        });

        let rendered = format_table_report(&report);
        assert!(rendered.contains("TinyVerse: GHA Babysit"));
        assert!(rendered.contains("PTY tip:"));
    }
}
