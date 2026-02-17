use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn render_output<T, F>(value: &T, format: OutputFormat, render_text: F) -> Result<String>
where
    T: Serialize,
    F: FnOnce(&T) -> String,
{
    let rendered = match format {
        OutputFormat::Text => render_text(value),
        OutputFormat::Json => serde_json::to_string_pretty(value)?,
    };

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::{OutputFormat, render_output};

    #[derive(Serialize)]
    struct Example {
        label: &'static str,
    }

    #[test]
    fn renders_text_output() {
        let value = Example { label: "ok" };
        let output = render_output(&value, OutputFormat::Text, |_| "text".to_owned())
            .expect("text rendering should succeed");

        assert_eq!(output, "text");
    }

    #[test]
    fn renders_json_output() {
        let value = Example { label: "ok" };
        let output = render_output(&value, OutputFormat::Json, |_| "ignored".to_owned())
            .expect("json rendering should succeed");

        assert!(output.contains("\"label\": \"ok\""));
    }
}
