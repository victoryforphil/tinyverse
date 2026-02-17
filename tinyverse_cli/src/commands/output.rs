use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Text,
    Json,
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
    };

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::{render_output, OutputFormat};

    #[derive(Serialize)]
    struct Example {
        label: &'static str,
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
}
