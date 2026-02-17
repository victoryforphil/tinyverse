use std::process::Command;

use anyhow::{bail, Context, Result};
use tinyverse_lib::tmux::{PaneTarget, SessionTarget};

const LIST_PANES_FORMAT: &str = "#{pane_id}\t#{pane_index}\t#{pane_title}\t#{?pane_active,1,0}";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneSnapshot {
    pub pane_id: String,
    pub pane_index: u32,
    pub pane_title: String,
    pub is_active: bool,
}

pub fn pane_target_from_selector(selector: Option<&str>) -> Option<PaneTarget> {
    selector
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PaneTarget::from_selector)
}

pub fn current_session_target() -> Result<Option<SessionTarget>> {
    if std::env::var_os("TMUX").is_none() {
        return Ok(None);
    }

    let output = Command::new("tmux")
        .args(["display-message", "-p", "#S"])
        .output()
        .context("failed to resolve current tmux session")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!("failed to resolve current tmux session (stderr={stderr:?})");
    }

    let session_name = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if session_name.is_empty() {
        bail!("resolved empty tmux session name");
    }

    Ok(Some(SessionTarget::new(session_name)))
}

pub fn resolve_session_target(explicit_session: Option<&str>) -> Result<SessionTarget> {
    if let Some(session) = explicit_session {
        let trimmed = session.trim();
        if !trimmed.is_empty() {
            return Ok(SessionTarget::new(trimmed));
        }
    }

    current_session_target()?.ok_or_else(|| {
        anyhow::anyhow!("session is required outside tmux; pass --session <id-or-name>")
    })
}

pub fn current_pane_id() -> Option<String> {
    std::env::var("TMUX_PANE")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn list_pane_snapshots(session: &SessionTarget) -> Result<Vec<PaneSnapshot>> {
    let output = Command::new("tmux")
        .args([
            "list-panes",
            "-t",
            session.as_str(),
            "-F",
            LIST_PANES_FORMAT,
        ])
        .output()
        .with_context(|| format!("failed to list panes for session `{}`", session.as_str()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!(
            "failed to list panes for session `{}` (stderr={stderr:?})",
            session.as_str()
        );
    }

    parse_pane_snapshots(&String::from_utf8_lossy(&output.stdout))
}

fn parse_pane_snapshots(output: &str) -> Result<Vec<PaneSnapshot>> {
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut fields = line.splitn(4, '\t');
            let pane_id = fields
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing pane id in line {line:?}"))?;
            let pane_index = fields
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing pane index in line {line:?}"))?;
            let pane_title = fields
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing pane title in line {line:?}"))?;
            let pane_active = fields
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing pane active flag in line {line:?}"))?;

            let pane_index = pane_index
                .parse::<u32>()
                .with_context(|| format!("invalid pane index `{pane_index}` in line {line:?}"))?;

            let is_active = match pane_active {
                "1" => true,
                "0" => false,
                _ => {
                    bail!("invalid pane active flag `{pane_active}` in line {line:?}");
                }
            };

            Ok(PaneSnapshot {
                pane_id: pane_id.to_owned(),
                pane_index,
                pane_title: pane_title.to_owned(),
                is_active,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_pane_snapshots;

    #[test]
    fn parses_pane_snapshot_rows() {
        let panes = parse_pane_snapshots("%1\t0\tconsole\t1\n%2\t1\tagent\t0")
            .expect("pane rows should parse");
        assert_eq!(panes.len(), 2);
        assert!(panes[0].is_active);
        assert_eq!(panes[1].pane_title, "agent");
    }
}
