use anyhow::{Result, bail};
use clap::Args;
use tinyverse_ui::{ActionLine, LabeledField, Panel, Tone, default_stdout_context};

use super::store;

#[derive(Debug, Args)]
pub struct ConfigSetArgs {
    /// Config key
    pub key: String,
    /// Config value
    pub value: String,
}

pub fn execute(args: ConfigSetArgs) -> Result<()> {
    let loaded = store::load_with_context()?;
    let source_label = loaded.source_label();
    let mut config = loaded.config;
    let key = args.key.trim();
    let value = args.value.trim();

    match key {
        "shell.clean" => {
            config.shell.clean = parse_bool(value)?;
        }
        "workspace.default_dir" => {
            config.workspace.default_dir = parse_optional_string(value);
        }
        "git.branch_prefix" => {
            config.git.branch_prefix = parse_required_string(value, key)?;
        }
        "spawn.default_agent" => {
            config.spawn.default_agent = parse_required_string(value, key)?;
        }
        "spawn.default_model" => {
            config.spawn.default_model = parse_optional_string(value);
        }
        _ => bail!(
            "unknown config key `{key}` (supported: shell.clean, workspace.default_dir, git.branch_prefix, spawn.default_agent, spawn.default_model)"
        ),
    }

    let path = store::save_to_path(&config, loaded.active_path.clone())?;
    let context = default_stdout_context();
    let output = Panel::new(
        [
            ActionLine::new("OK", "Updated TinyVerse config", Tone::Success).render(&context),
            String::new(),
            LabeledField::new("Path", path.display().to_string()).render(&context),
            LabeledField::new("Home source", source_label).render(&context),
            LabeledField::new("Key", key).render(&context),
            LabeledField::new("Value", value).render(&context),
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: Config")
    .with_tone(Tone::Success)
    .render(&context);

    println!("{output}");
    Ok(())
}

fn parse_bool(value: &str) -> Result<bool> {
    match value {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => bail!("invalid boolean `{value}`; use true/false"),
    }
}

fn parse_optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || matches!(trimmed, "none" | "null" | "-") {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn parse_required_string(value: &str, key: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        bail!("invalid value for `{key}`: cannot be empty")
    }
    Ok(trimmed.to_owned())
}
