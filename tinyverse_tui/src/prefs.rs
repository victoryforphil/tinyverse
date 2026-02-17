use std::path::PathBuf;

use anyhow::{Context, Result};
use tinyverse_lib::resolve_tinyverse_paths;

const PREFS_FILE_NAME: &str = "tui_prefs.toml";

#[derive(Debug, Clone, Default)]
pub struct TuiPrefs {
    pub spawn_agent: Option<String>,
    pub spawn_model: Option<String>,
}

impl TuiPrefs {
    pub fn apply_to_spawn_form(&self, form: &mut crate::app::SpawnForm) {
        if let Some(agent) = self.spawn_agent.as_deref().map(str::trim)
            && !agent.is_empty()
        {
            form.agent_type = agent.to_owned();
        }
        form.model = self
            .spawn_model
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_default();
    }
}

pub fn load() -> Result<TuiPrefs> {
    let path = prefs_path()?;
    if !path.exists() {
        return Ok(TuiPrefs::default());
    }

    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read prefs `{}`", path.display()))?;
    Ok(parse_prefs(&raw))
}

pub fn save(prefs: &TuiPrefs) -> Result<PathBuf> {
    let path = prefs_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create prefs dir `{}`", parent.display()))?;
    }

    std::fs::write(&path, render_prefs(prefs))
        .with_context(|| format!("failed to write prefs `{}`", path.display()))?;
    Ok(path)
}

fn prefs_path() -> Result<PathBuf> {
    let resolved = resolve_tinyverse_paths(None)?;
    Ok(resolved.home_dir.join(PREFS_FILE_NAME))
}

fn parse_prefs(raw: &str) -> TuiPrefs {
    let mut prefs = TuiPrefs::default();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        let parsed = unquote(value);
        match key {
            "spawn_agent" => prefs.spawn_agent = Some(parsed),
            "spawn_model" => prefs.spawn_model = Some(parsed),
            _ => {}
        }
    }
    prefs
}

fn render_prefs(prefs: &TuiPrefs) -> String {
    let mut lines = vec![String::from("# tinyverse tui preferences")];
    if let Some(agent) = prefs.spawn_agent.as_deref() {
        lines.push(format!("spawn_agent = \"{}\"", escape(agent)));
    }
    if let Some(model) = prefs.spawn_model.as_deref() {
        lines.push(format!("spawn_model = \"{}\"", escape(model)));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn unquote(value: &str) -> String {
    let unwrapped = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(value);
    unwrapped.replace("\\\"", "\"").replace("\\\\", "\\")
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
