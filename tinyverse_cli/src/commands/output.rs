use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;

/// Re-export the canonical display-name formatter from `tinyverse_ui`.
///
/// Converts machine names like `tinyverse_redding` to polished
/// user-facing names like `Redding do TinyVerse // Redding`.
pub use tinyverse_ui::format_display_name as display_session_name;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Text,
    Json,
    Toml,
    Yaml,
}

pub fn render_output<T, FT, FTab>(
    value: &T,
    format: OutputFormat,
    render_table: FTab,
    render_text: FT,
) -> Result<String>
where
    T: Serialize,
    FT: FnOnce(&T) -> String,
    FTab: FnOnce(&T) -> String,
{
    let rendered = match format {
        OutputFormat::Table => render_table(value),
        OutputFormat::Text => render_text(value),
        OutputFormat::Json => serde_json::to_string_pretty(value)?,
        OutputFormat::Toml => toml::to_string_pretty(value)?,
        OutputFormat::Yaml => serde_yaml::to_string(value)?,
    };

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::{display_session_name, render_output, OutputFormat};

    #[derive(Serialize)]
    struct Example {
        label: &'static str,
    }

    #[test]
    fn display_name_reexport_works() {
        assert_eq!(
            display_session_name("tinyverse_redding"),
            "Redding do TinyVerse // Redding"
        );
    }

    #[test]
    fn renders_text_output() {
        let value = Example { label: "ok" };
        let output = render_output(
            &value,
            OutputFormat::Text,
            |_| "table".to_owned(),
            |_| "text".to_owned(),
        )
        .expect("text rendering should succeed");

        assert_eq!(output, "text");
    }

    #[test]
    fn renders_json_output() {
        let value = Example { label: "ok" };
        let output = render_output(
            &value,
            OutputFormat::Json,
            |_| "ignored".to_owned(),
            |_| "ignored".to_owned(),
        )
        .expect("json rendering should succeed");

        assert!(output.contains("\"label\": \"ok\""));
    }

    #[test]
    fn renders_table_output() {
        let value = Example { label: "ok" };
        let output = render_output(
            &value,
            OutputFormat::Table,
            |_| "table".to_owned(),
            |_| "text".to_owned(),
        )
        .expect("table rendering should succeed");

        assert_eq!(output, "table");
    }

    #[test]
    fn renders_toml_output() {
        let value = Example { label: "ok" };
        let output = render_output(
            &value,
            OutputFormat::Toml,
            |_| "ignored".to_owned(),
            |_| "ignored".to_owned(),
        )
        .expect("toml rendering should succeed");

        assert!(output.contains("label = \"ok\""));
    }

    #[test]
    fn renders_yaml_output() {
        let value = Example { label: "ok" };
        let output = render_output(
            &value,
            OutputFormat::Yaml,
            |_| "ignored".to_owned(),
            |_| "ignored".to_owned(),
        )
        .expect("yaml rendering should succeed");

        assert!(output.contains("label: ok"));
    }
}
