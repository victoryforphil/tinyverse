use anyhow::Result;
use clap::Args;
use tinyverse_ui::{ActionLine, DetailSection, LabeledField, Panel, Tone, default_stdout_context};

use super::store;

#[derive(Debug, Args)]
pub struct ConfigPrintArgs {}

pub fn execute(_args: ConfigPrintArgs) -> Result<()> {
    let context = default_stdout_context();
    let loaded = store::load_with_context()?;
    let source_label = loaded.source_label();
    let config = loaded.config;

    let details = DetailSection::new("Shell")
        .with_field(LabeledField::new(
            "Clean shell",
            config.shell.clean.to_string(),
        ))
        .render(&context);

    let workspace = DetailSection::new("Workspace")
        .with_field(LabeledField::new(
            "Default dir",
            config.workspace.default_dir.as_deref().unwrap_or("<unset>"),
        ))
        .render(&context);

    let git = DetailSection::new("Git")
        .with_field(LabeledField::new(
            "Branch prefix",
            config.git.branch_prefix.as_str(),
        ))
        .render(&context);

    let spawn = DetailSection::new("Spawn")
        .with_field(LabeledField::new(
            "Default agent",
            config.spawn.default_agent.as_str(),
        ))
        .with_field(LabeledField::new(
            "Default model",
            config.spawn.default_model.as_deref().unwrap_or("<unset>"),
        ))
        .render(&context);

    let loaded_from = if loaded.loaded_paths.is_empty() {
        "<none>".to_owned()
    } else {
        loaded
            .loaded_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    };

    let output = Panel::new(
        [
            ActionLine::new("INFO", "TinyVerse config", Tone::Info).render(&context),
            String::new(),
            LabeledField::new("Home source", source_label).render(&context),
            LabeledField::new("Selected home", loaded.selected_home.display().to_string())
                .render(&context),
            LabeledField::new("Active path", loaded.active_path.display().to_string())
                .render(&context),
            LabeledField::new("Legacy path", loaded.legacy_path.display().to_string())
                .render(&context),
            LabeledField::new("Loaded from", loaded_from).render(&context),
            String::new(),
            details,
            String::new(),
            workspace,
            String::new(),
            git,
            String::new(),
            spawn,
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: Config")
    .with_tone(Tone::Info)
    .render(&context);

    println!("{output}");
    Ok(())
}
