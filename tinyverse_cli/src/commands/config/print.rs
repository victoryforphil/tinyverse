use anyhow::Result;
use clap::{Args, ValueEnum};
use serde::Serialize;
use tinyverse_ui::{
    ActionLine, LabeledField, Panel, StripeMode, StyledTable, SummaryFooter, Tone,
    default_stdout_context,
};

use crate::commands::output::OutputFormat;

use super::store;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ConfigPrintOutput {
    #[value(alias = "current")]
    Full,
    Raw,
}

#[derive(Debug, Args)]
pub struct ConfigPrintArgs {
    /// Output mode: full (panel + metadata) or raw (config only)
    #[arg(long, value_enum, default_value_t = ConfigPrintOutput::Full)]
    pub output: ConfigPrintOutput,
    /// Render format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigPrintReport {
    selected_source: String,
    selected_home: String,
    active_path: String,
    legacy_path: String,
    loaded_from: Vec<String>,
    config: store::TinyverseConfig,
}

pub fn execute(args: ConfigPrintArgs) -> Result<()> {
    let loaded = store::load_with_context()?;
    let report = ConfigPrintReport {
        selected_source: loaded.source_label().to_owned(),
        selected_home: loaded.selected_home.display().to_string(),
        active_path: loaded.active_path.display().to_string(),
        legacy_path: loaded.legacy_path.display().to_string(),
        loaded_from: loaded
            .loaded_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        config: loaded.config,
    };

    let output = if args.output == ConfigPrintOutput::Raw {
        render_raw_output(&report.config, args.format)?
    } else {
        render_full_output(&report, args.format)?
    };

    println!("{output}");
    Ok(())
}

fn render_full_output(report: &ConfigPrintReport, format: OutputFormat) -> Result<String> {
    if format == OutputFormat::Table {
        return Ok(render_full_table_output(report));
    }

    let context = default_stdout_context();
    let loaded_from = if report.loaded_from.is_empty() {
        "<none>".to_owned()
    } else {
        report.loaded_from.join(" | ")
    };

    let config_payload = render_raw_output(&report.config, format)?;
    let preview_panel = Panel::new(prefix_preview_lines(&config_payload))
        .with_title(format!("Config preview ({})", format_label(format)))
        .with_tone(Tone::Neutral)
        .render(&context);

    Ok(Panel::new(
        [
            ActionLine::new("INFO", "TinyVerse config", Tone::Info).render(&context),
            String::new(),
            LabeledField::new("Render format", format_label(format)).render(&context),
            LabeledField::new("Home source", report.selected_source.as_str()).render(&context),
            LabeledField::new("Selected home", report.selected_home.as_str()).render(&context),
            LabeledField::new("Active path", report.active_path.as_str()).render(&context),
            LabeledField::new("Legacy path", report.legacy_path.as_str()).render(&context),
            LabeledField::new("Loaded from", loaded_from).render(&context),
            String::new(),
            ActionLine::new("INFO", "Config payload", Tone::Info).render(&context),
            preview_panel,
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: Config")
    .with_tone(Tone::Info)
    .render(&context))
}

fn prefix_preview_lines(content: &str) -> String {
    content
        .lines()
        .map(|line| format!("| {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_full_table_output(report: &ConfigPrintReport) -> String {
    let context = default_stdout_context();

    let mut metadata = StyledTable::new(vec!["FIELD", "VALUE"])
        .with_stripe_mode(StripeMode::DimEvenRows)
        .with_row(vec!["Render format".to_owned(), "table".to_owned()])
        .with_row(vec![
            "Home source".to_owned(),
            report.selected_source.clone(),
        ])
        .with_row(vec![
            "Selected home".to_owned(),
            report.selected_home.clone(),
        ])
        .with_row(vec!["Active path".to_owned(), report.active_path.clone()])
        .with_row(vec!["Legacy path".to_owned(), report.legacy_path.clone()]);

    if report.loaded_from.is_empty() {
        metadata = metadata.with_row(vec!["Loaded from".to_owned(), "<none>".to_owned()]);
    } else {
        metadata = metadata.with_row(vec![
            "Loaded from".to_owned(),
            report.loaded_from.join(" | "),
        ]);
    }

    let settings = StyledTable::new(vec!["KEY", "VALUE"])
        .with_stripe_mode(StripeMode::DimEvenRows)
        .with_row(vec![
            "shell.clean".to_owned(),
            report.config.shell.clean.to_string(),
        ])
        .with_row(vec![
            "workspace.default_dir".to_owned(),
            report
                .config
                .workspace
                .default_dir
                .as_deref()
                .unwrap_or("<unset>")
                .to_owned(),
        ])
        .with_row(vec![
            "git.branch_prefix".to_owned(),
            report.config.git.branch_prefix.clone(),
        ])
        .with_row(vec![
            "spawn.default_agent".to_owned(),
            report.config.spawn.default_agent.clone(),
        ])
        .with_row(vec![
            "spawn.default_model".to_owned(),
            report
                .config
                .spawn
                .default_model
                .as_deref()
                .unwrap_or("<unset>")
                .to_owned(),
        ]);

    Panel::new(
        [
            ActionLine::new("INFO", "TinyVerse config", Tone::Info).render(&context),
            String::new(),
            Panel::new(metadata.render(&context))
                .with_title("Metadata")
                .with_tone(Tone::Info)
                .render(&context),
            String::new(),
            Panel::new(settings.render(&context))
                .with_title("Config")
                .with_tone(Tone::Info)
                .render(&context),
            String::new(),
            SummaryFooter::new("5 config key(s)").render(&context),
        ]
        .join("\n"),
    )
    .with_title("TinyVerse: Config")
    .with_tone(Tone::Info)
    .render(&context)
}

fn format_label(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Table => "table",
        OutputFormat::Text => "text",
        OutputFormat::Json => "json",
        OutputFormat::Toml => "toml",
        OutputFormat::Yaml => "yaml",
    }
}

fn render_raw_output(config: &store::TinyverseConfig, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Table | OutputFormat::Text | OutputFormat::Toml => {
            Ok(toml::to_string_pretty(config)?)
        }
        OutputFormat::Json => Ok(serde_json::to_string_pretty(config)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(config)?),
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigPrintReport, render_full_output, render_raw_output};
    use crate::commands::config::store::{
        GitConfig, ShellConfig, SpawnConfig, TinyverseConfig, WorkspaceConfig,
    };
    use crate::commands::output::OutputFormat;

    fn sample_report() -> ConfigPrintReport {
        ConfigPrintReport {
            selected_source: "repo_local".to_owned(),
            selected_home: "/tmp/.tinyverse".to_owned(),
            active_path: "/tmp/.tinyverse/config.toml".to_owned(),
            legacy_path: "/tmp/.tinyverse/tinyverse.toml".to_owned(),
            loaded_from: vec!["/tmp/.tinyverse/config.toml".to_owned()],
            config: TinyverseConfig {
                shell: ShellConfig { clean: false },
                workspace: WorkspaceConfig {
                    default_dir: Some(".".to_owned()),
                },
                git: GitConfig {
                    branch_prefix: "tv/".to_owned(),
                },
                spawn: SpawnConfig {
                    default_agent: "opencode".to_owned(),
                    default_model: None,
                },
            },
        }
    }

    #[test]
    fn full_table_renders_panel_with_payload() {
        let rendered =
            render_full_output(&sample_report(), OutputFormat::Table).expect("full table works");
        assert!(rendered.contains("TinyVerse: Config"));
        assert!(rendered.contains("Metadata"));
        assert!(rendered.contains("Config"));
        assert!(rendered.contains("shell.clean"));
    }

    #[test]
    fn raw_yaml_renders_config_only() {
        let config = sample_report().config;
        let rendered =
            render_raw_output(&config, OutputFormat::Yaml).expect("yaml render should work");
        assert!(rendered.contains("shell:"));
        assert!(rendered.contains("spawn:"));
    }

    #[test]
    fn full_json_renders_inside_panel() {
        let rendered =
            render_full_output(&sample_report(), OutputFormat::Json).expect("full json works");
        assert!(rendered.contains("TinyVerse: Config"));
        assert!(rendered.contains("Render format: json"));
        assert!(rendered.contains("\"shell\""));
    }
}
