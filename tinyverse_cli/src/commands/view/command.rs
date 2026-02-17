use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{CapturePaneOptions, PaneTarget, TmuxClient};
use tinyverse_lib::{
    SessionStore, pane_target_from_selector, resolve_session_target_with_store,
    strip_ansi_and_controls,
};
use tinyverse_ui::{ActionLine, LabeledField, Panel, Tone, default_stdout_context};

use super::args::{ViewArgs, ViewOutput};
use crate::commands::output::display_session_name;

pub fn execute(args: ViewArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let session = resolve_session_target_with_store(args.session.as_deref(), &mut store)?;
    let pane = pane_target_from_selector(args.panel.as_deref());

    let mut options = CapturePaneOptions::new(session.clone());
    options.pane = pane.clone();
    options.start_line = Some(-500);
    options.preserve_ansi = args.output == ViewOutput::Raw;
    options.include_alternate_screen = true;

    let client = TmuxClient::new();
    let captured = client
        .capture_pane(options)
        .with_context(|| format!("failed to capture pane for session `{}`", session.as_str()))?;

    let output = render_output(
        args.output,
        &session.to_string(),
        &pane,
        &captured.text,
        &captured.pane_id,
    );

    if let Some(export_path) = args.export.as_deref() {
        let file_path = export_output(export_path, &output)?;
        info!("Exported view output to {}", file_path.display());
    }

    println!("{output}");
    Ok(())
}

fn render_output(
    mode: ViewOutput,
    session_name: &str,
    pane: &Option<PaneTarget>,
    captured_text: &str,
    captured_pane_id: &str,
) -> String {
    if mode == ViewOutput::Raw {
        return captured_text.to_owned();
    }

    let cleaned_text = strip_ansi_and_controls(captured_text);

    let context = default_stdout_context();
    let display_name = display_session_name(session_name);

    let target_label = match pane {
        Some(PaneTarget::Role(role)) => format!("{display_name} ({role})"),
        Some(PaneTarget::PaneId(id)) => format!("{display_name} ({id})"),
        None => format!("{display_name} ({captured_pane_id})"),
    };

    let header = ActionLine::new(
        "INFO",
        format!("Captured pane from {display_name}"),
        Tone::Info,
    )
    .render(&context);
    let field = LabeledField::new("Pane", target_label).render(&context);

    Panel::new([header, field, String::new(), cleaned_text].join("\n"))
        .with_title("TinyVerse: View")
        .with_tone(Tone::Info)
        .render(&context)
}

fn export_output(path: &Path, output: &str) -> Result<PathBuf> {
    let export_path = ensure_markdown_extension(path);
    if let Some(parent) = export_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create export directory `{}`", parent.display()))?;
    }

    std::fs::write(&export_path, output)
        .with_context(|| format!("failed to write export file `{}`", export_path.display()))?;

    Ok(export_path)
}

fn ensure_markdown_extension(path: &Path) -> PathBuf {
    if path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.with_extension("md")
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ensure_markdown_extension;

    #[test]
    fn adds_md_extension_when_missing() {
        let output = ensure_markdown_extension(Path::new("capture"));
        assert_eq!(output, Path::new("capture.md"));
    }

    #[test]
    fn keeps_existing_extension() {
        let output = ensure_markdown_extension(Path::new("capture.txt"));
        assert_eq!(output, Path::new("capture.txt"));
    }
}
