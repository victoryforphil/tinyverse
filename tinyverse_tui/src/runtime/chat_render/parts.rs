use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use serde_json::Value;

use crate::app::App;
use crate::chat::ChatMessagePart;
use tinyverse_tui_components::compact_text as truncate_to;

use super::types::RenderedChatLine;

#[derive(Debug, Clone)]
struct TodoRenderItem {
    content: String,
    status: String,
    priority: String,
}

// ── apply_patch support ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PatchOp {
    Add,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub(super) struct PatchFileEntry {
    pub(super) op: PatchOp,
    pub(super) path: String,
}

#[derive(Debug, Clone)]
pub(super) struct PatchSummary {
    pub(super) files: Vec<PatchFileEntry>,
}

impl PatchSummary {
    pub(super) fn count(&self, op: PatchOp) -> usize {
        self.files.iter().filter(|f| f.op == op).count()
    }

    fn compact_label(&self) -> String {
        let adds = self.count(PatchOp::Add);
        let updates = self.count(PatchOp::Update);
        let deletes = self.count(PatchOp::Delete);
        let total = self.files.len();

        let mut parts: Vec<String> = Vec::new();
        if adds > 0 {
            parts.push(format!("+{adds}"));
        }
        if updates > 0 {
            parts.push(format!("~{updates}"));
        }
        if deletes > 0 {
            parts.push(format!("-{deletes}"));
        }

        let ops = if parts.is_empty() {
            String::from("0 files")
        } else {
            parts.join(" ")
        };

        // Show first file basename as hint when ≤3 files
        let hint = if total == 1 {
            let basename = self.files[0]
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&self.files[0].path);
            format!(" {basename}")
        } else if total <= 3 {
            let names: Vec<&str> = self
                .files
                .iter()
                .map(|f| f.path.rsplit('/').next().unwrap_or(&f.path))
                .collect();
            format!(" {}", names.join(", "))
        } else {
            format!(" ({total} files)")
        };

        format!("{ops}{hint}")
    }
}

/// Parse patch text from `apply_patch` tool input.
///
/// Looks for `*** <Op> File: <path>` header lines in the patch body.
/// Accepts the raw input string which may be JSON containing `"patchText"` or
/// direct patch text.
pub(super) fn parse_patch_summary(raw: &str) -> Option<PatchSummary> {
    // Try to extract patchText from JSON wrapper first.
    let owned_patch;
    let patch_body = if let Some(extracted) = extract_patch_text_from_json(raw) {
        owned_patch = extracted;
        owned_patch.as_str()
    } else {
        raw
    };

    let mut files = Vec::new();
    for line in patch_body.lines() {
        let trimmed = line.trim();
        if let Some(path) = trimmed.strip_prefix("*** Add File: ") {
            files.push(PatchFileEntry {
                op: PatchOp::Add,
                path: path.trim().to_owned(),
            });
        } else if let Some(path) = trimmed.strip_prefix("*** Update File: ") {
            files.push(PatchFileEntry {
                op: PatchOp::Update,
                path: path.trim().to_owned(),
            });
        } else if let Some(path) = trimmed.strip_prefix("*** Delete File: ") {
            files.push(PatchFileEntry {
                op: PatchOp::Delete,
                path: path.trim().to_owned(),
            });
        }
    }

    if files.is_empty() {
        return None;
    }

    Some(PatchSummary { files })
}

pub(super) fn extract_patch_text(raw: &str) -> Option<String> {
    if let Some(extracted) = extract_patch_text_from_json(raw) {
        return Some(extracted);
    }

    if raw.contains("*** Begin Patch") || raw.contains("@@") {
        return Some(raw.to_owned());
    }

    None
}

fn extract_patch_text_from_json(raw: &str) -> Option<String> {
    // Fast check: skip full parse if no patchText key present.
    if !raw.contains("\"patchText\"") {
        return None;
    }
    let parsed = serde_json::from_str::<Value>(raw).ok()?;
    parsed
        .get("patchText")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn collapsible_header(
    label: &str,
    kind_tag: &str,
    fg: Color,
    width: u16,
    is_focused: bool,
    part_key: Option<String>,
    app: &App,
) -> RenderedChatLine {
    let icon = "◆";
    let hint = if is_focused {
        "  enter/click popup"
    } else {
        ""
    };
    let left = format!("  {icon} {label}");
    let tag = format!(" {kind_tag} ");
    let used = left.chars().count() + hint.chars().count() + tag.chars().count();
    let spacer = " ".repeat((width as usize).saturating_sub(used).max(1));

    let bg = if is_focused {
        app.theme.chat_collapsible_focused_bg
    } else {
        app.theme.chat_collapsible_bg
    };

    let mut spans = vec![Span::styled(
        left,
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
    )];

    if !hint.is_empty() {
        spans.push(Span::styled(
            hint.to_owned(),
            Style::default()
                .fg(app.theme.text_muted)
                .bg(bg)
                .add_modifier(Modifier::ITALIC),
        ));
    }

    spans.push(Span::styled(spacer, Style::default().bg(bg)));
    spans.push(Span::styled(
        tag,
        Style::default()
            .fg(fg)
            .bg(app.theme.chat_collapsible_tag_bg)
            .add_modifier(Modifier::BOLD),
    ));

    RenderedChatLine {
        line: Line::from(spans),
        toggle_key: part_key,
    }
}

pub(super) fn render_chat_part_lines(
    part: &ChatMessagePart,
    app: &App,
    width: u16,
    part_key: Option<String>,
) -> Vec<RenderedChatLine> {
    let max = width.saturating_sub(8) as usize;
    let is_focused = is_focused_part(app, part_key.as_deref());
    match part {
        ChatMessagePart::Text(value) => {
            if is_hidden_noise_line(value) {
                return Vec::new();
            }

            let style = Style::default().fg(app.theme.text_secondary);
            value
                .lines()
                .map(|line| {
                    let mut prefixed = String::with_capacity(line.len() + 2);
                    prefixed.push_str("  ");
                    prefixed.push_str(line);
                    RenderedChatLine {
                        line: line_with_path_pills(&prefixed, style, app),
                        toggle_key: part_key
                            .clone()
                            .filter(|_| line_contains_highlightable_path(line)),
                    }
                })
                .collect()
        }
        ChatMessagePart::Markdown(value) => {
            let first_key = if part_key.is_some() && part.is_collapsible() {
                part_key.clone()
            } else {
                None
            };
            let base_style = Style::default().fg(app.theme.text_secondary);
            let heading_style = Style::default()
                .fg(app.theme.text_primary)
                .add_modifier(Modifier::BOLD);
            value
                .lines()
                .enumerate()
                .map(|(index, line)| {
                    let trimmed = line.trim_start();
                    let style = if trimmed.starts_with('#') {
                        heading_style
                    } else {
                        base_style
                    };

                    let mut prefixed = String::with_capacity(line.len() + 2);
                    prefixed.push_str("  ");
                    prefixed.push_str(line);
                    RenderedChatLine {
                        line: line_with_path_pills(&prefixed, style, app),
                        toggle_key: part_key
                            .clone()
                            .filter(|_| line_contains_highlightable_path(line))
                            .or_else(|| if index == 0 { first_key.clone() } else { None }),
                    }
                })
                .collect()
        }
        ChatMessagePart::Thinking(value) => {
            let mut out = Vec::with_capacity(2);
            out.push(collapsible_header(
                "Reasoning",
                "thinking",
                app.theme.pill_muted_fg,
                width,
                is_focused,
                part_key.clone(),
                app,
            ));

            let preview = value.lines().next().unwrap_or("(no detail)");
            let mut preview_text = String::with_capacity(preview.len() + 4);
            preview_text.push_str("    ");
            preview_text.push_str(&truncate_to(preview, max));
            out.push(RenderedChatLine {
                line: line_with_path_pills(
                    &preview_text,
                    Style::default().fg(app.theme.text_muted),
                    app,
                ),
                toggle_key: part_key
                    .clone()
                    .filter(|_| line_contains_highlightable_path(preview)),
            });
            out
        }
        ChatMessagePart::Code { language, code } => {
            let label = language.as_deref().unwrap_or("text");
            let mut out = Vec::with_capacity(2);
            out.push(collapsible_header(
                &format!("code ({label})"),
                "code",
                app.theme.pill_accent_fg,
                width,
                is_focused,
                part_key.clone(),
                app,
            ));

            let preview = code.lines().next().unwrap_or("(empty)");
            let mut preview_text = String::with_capacity(preview.len() + 4);
            preview_text.push_str("    ");
            preview_text.push_str(&truncate_to(preview, max));
            out.push(RenderedChatLine {
                line: Line::from(Span::styled(
                    preview_text,
                    Style::default().fg(app.theme.text_muted),
                )),
                toggle_key: None,
            });
            out
        }
        ChatMessagePart::ToolCall {
            name,
            input,
            output,
        } => {
            let is_todo_tool = name.eq_ignore_ascii_case("todowrite");
            let is_patch_tool = name.eq_ignore_ascii_case("apply_patch");
            let header_label = if is_todo_tool {
                String::from("todo list update")
            } else if is_patch_tool {
                String::from("patch")
            } else {
                format!("tool {name}")
            };
            let header_tag = if is_todo_tool {
                "todo"
            } else if is_patch_tool {
                "patch"
            } else {
                "tool"
            };

            let mut out = Vec::with_capacity(2);
            out.push(collapsible_header(
                &header_label,
                header_tag,
                app.theme.pill_info_fg,
                width,
                is_focused,
                part_key.clone(),
                app,
            ));

            let preview = if is_todo_tool {
                output
                    .as_deref()
                    .and_then(parse_todo_items)
                    .or_else(|| input.as_deref().and_then(parse_todo_items))
                    .map(|todos| todo_preview_label(&todos))
                    .or_else(|| {
                        input
                            .as_deref()
                            .and_then(first_meaningful_line)
                            .map(str::to_owned)
                    })
                    .or_else(|| {
                        output
                            .as_deref()
                            .and_then(first_meaningful_line)
                            .map(str::to_owned)
                    })
                    .unwrap_or_else(|| String::from("todo state update"))
            } else if is_patch_tool {
                input
                    .as_deref()
                    .and_then(parse_patch_summary)
                    .map(|s| s.compact_label())
                    .unwrap_or_else(|| String::from("apply patch"))
            } else {
                input
                    .as_deref()
                    .and_then(first_meaningful_line)
                    .or_else(|| output.as_deref().and_then(first_meaningful_line))
                    .unwrap_or("(details)")
                    .to_owned()
            };
            let mut preview_text = String::with_capacity(preview.len() + 4);
            preview_text.push_str("    ");
            preview_text.push_str(&truncate_to(&preview, max));
            out.push(RenderedChatLine {
                line: line_with_path_pills(
                    &preview_text,
                    Style::default().fg(app.theme.text_muted),
                    app,
                ),
                toggle_key: part_key
                    .clone()
                    .filter(|_| line_contains_highlightable_path(&preview)),
            });
            out
        }
        ChatMessagePart::ShellCommand(value) => {
            let command = truncate_to(value, max);
            let spans = command_spans(&command, app);

            vec![RenderedChatLine {
                line: Line::from(spans),
                toggle_key: part_key
                    .clone()
                    .filter(|_| line_contains_highlightable_path(value)),
            }]
        }
        ChatMessagePart::ShellOutput { output, exit_code } => {
            let mut out = Vec::with_capacity(2);
            out.push(collapsible_header(
                "shell output",
                "shell",
                app.theme.text_muted,
                width,
                is_focused,
                part_key.clone(),
                app,
            ));

            let preview = output.lines().next().unwrap_or("(empty)");
            let suffix = exit_code
                .map(|code| format!("  exit {code}"))
                .unwrap_or_default();
            let mut preview_text = String::with_capacity(preview.len() + suffix.len() + 4);
            preview_text.push_str("    ");
            preview_text.push_str(&truncate_to(preview, max));
            preview_text.push_str(&suffix);
            out.push(RenderedChatLine {
                line: line_with_path_pills(
                    &preview_text,
                    Style::default().fg(app.theme.text_muted),
                    app,
                ),
                toggle_key: None,
            });
            out
        }
        ChatMessagePart::Error(value) => vec![RenderedChatLine {
            line: Line::from(vec![
                Span::styled(
                    " ERR ",
                    Style::default()
                        .fg(app.theme.pill_err_fg)
                        .bg(app.theme.pill_err_bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    truncate_to(value, max),
                    Style::default().fg(app.theme.pill_err_fg),
                ),
            ]),
            toggle_key: None,
        }],
    }
}

fn is_focused_part(app: &App, part_key: Option<&str>) -> bool {
    part_key.is_some_and(|key| app.chat.focused_part_key() == Some(key))
}

fn line_with_path_pills(raw: &str, base_style: Style, app: &App) -> Line<'static> {
    Line::from(spans_with_path_pills(raw, base_style, app))
}

fn spans_with_path_pills(raw: &str, base_style: Style, app: &App) -> Vec<Span<'static>> {
    // Fast path: if the line has no '/' or common path indicators, skip path detection entirely.
    if !raw.contains('/') && !raw.contains("\"filePath\"") && !raw.contains("\"path\"") {
        return vec![Span::styled(raw.to_owned(), base_style)];
    }

    if let Some((prefix, path, suffix)) = extract_json_filepath(raw) {
        let display = display_path(&path, app);
        let path_style = Style::default()
            .fg(app.theme.path_pill_fg)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
        let mut spans = Vec::with_capacity(3);
        if !prefix.is_empty() {
            spans.push(Span::styled(prefix, base_style));
        }
        spans.push(Span::styled(display, path_style));
        if !suffix.is_empty() {
            spans.push(Span::styled(suffix, base_style));
        }
        return spans;
    }

    let path_style = Style::default()
        .fg(app.theme.path_pill_fg)
        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);

    let mut spans: Vec<Span<'static>> = Vec::new();
    for chunk in raw.split_inclusive(' ') {
        let token = chunk.trim_end_matches(' ');
        let trailing_spaces = &chunk[token.len()..];

        if token.is_empty() {
            if !trailing_spaces.is_empty() {
                spans.push(Span::styled(trailing_spaces.to_owned(), base_style));
            }
            continue;
        }

        let (leading, core, trailing) = split_token_edges(token);
        if is_path_like(core) {
            if !leading.is_empty() {
                spans.push(Span::styled(leading.to_owned(), base_style));
            }
            spans.push(Span::styled(display_path(core, app), path_style));
            if !trailing.is_empty() {
                spans.push(Span::styled(trailing.to_owned(), base_style));
            }
        } else {
            spans.push(Span::styled(token.to_owned(), base_style));
        }

        if !trailing_spaces.is_empty() {
            spans.push(Span::styled(trailing_spaces.to_owned(), base_style));
        }
    }

    if spans.is_empty() {
        spans.push(Span::styled(raw.to_owned(), base_style));
    }
    spans
}

fn line_contains_highlightable_path(raw: &str) -> bool {
    extract_json_filepath(raw).is_some()
        || raw.split_whitespace().any(|token| {
            let (_, core, _) = split_token_edges(token);
            is_path_like(core)
        })
}

fn command_spans(command: &str, app: &App) -> Vec<Span<'static>> {
    let bg = app.theme.chat_code_bg;
    let base = Style::default().fg(app.theme.text_primary).bg(bg);
    let command_style = Style::default()
        .fg(app.theme.pill_info_fg)
        .bg(bg)
        .add_modifier(Modifier::BOLD);
    let flag_style = Style::default().fg(app.theme.text_secondary).bg(bg);
    let path_style = Style::default()
        .fg(app.theme.path_pill_fg)
        .bg(bg)
        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);

    let mut spans = Vec::new();
    let mut first_token = true;
    for chunk in command.split_inclusive(' ') {
        let token = chunk.trim_end_matches(' ');
        let trailing_spaces = &chunk[token.len()..];

        if !token.is_empty() {
            let (leading, core, trailing) = split_token_edges(token);
            if !leading.is_empty() {
                spans.push(Span::styled(leading.to_owned(), base));
            }

            let style = if first_token {
                command_style
            } else if is_path_like(core) {
                path_style
            } else if core.starts_with('-') {
                flag_style
            } else {
                base
            };
            spans.push(Span::styled(core.to_owned(), style));

            if !trailing.is_empty() {
                spans.push(Span::styled(trailing.to_owned(), base));
            }
            first_token = false;
        }

        if !trailing_spaces.is_empty() {
            spans.push(Span::styled(trailing_spaces.to_owned(), base));
        }
    }

    if spans.is_empty() {
        spans.push(Span::styled(command.to_owned(), base));
    }
    spans
}

fn extract_json_filepath(raw: &str) -> Option<(String, String, String)> {
    let key_index = raw.find("\"filePath\"").or_else(|| raw.find("\"path\""))?;
    let colon_index = raw[key_index..].find(':')? + key_index;
    let first_quote = raw[colon_index + 1..].find('"')? + colon_index + 1;
    let second_quote = raw[first_quote + 1..].find('"')? + first_quote + 1;

    let prefix = raw[..first_quote].to_owned();
    let value = raw[first_quote + 1..second_quote].to_owned();
    let suffix = raw[second_quote + 1..].to_owned();
    Some((prefix, value, suffix))
}

fn split_token_edges(token: &str) -> (&str, &str, &str) {
    let mut start = 0usize;
    for (idx, ch) in token.char_indices() {
        if !is_wrapper_punct(ch) {
            start = idx;
            break;
        }
        start = idx + ch.len_utf8();
    }

    if start >= token.len() {
        return (token, "", "");
    }

    let mut end = token.len();
    for (idx, ch) in token.char_indices().rev() {
        if !is_wrapper_punct(ch) {
            end = idx + ch.len_utf8();
            break;
        }
        end = idx;
    }

    (&token[..start], &token[start..end], &token[end..])
}

fn is_wrapper_punct(ch: char) -> bool {
    matches!(
        ch,
        '"' | '\'' | '`' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ',' | ';' | ':'
    )
}

fn is_path_like(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.contains("://") {
        return false;
    }

    if trimmed.starts_with('/') || trimmed.starts_with("./") || trimmed.starts_with("../") {
        return true;
    }

    if trimmed.ends_with('/') {
        return false;
    }

    const EXTENSIONS: [&str; 13] = [
        ".rs", ".ts", ".tsx", ".js", ".json", ".toml", ".yml", ".yaml", ".md", ".sh", ".py", ".go",
        ".txt",
    ];

    if trimmed.contains('/') && EXTENSIONS.iter().any(|ext| trimmed.ends_with(ext)) {
        return true;
    }

    if trimmed.contains('/') {
        const PATH_PREFIXES: [&str; 8] = [
            "src/",
            "docs/",
            "scripts/",
            "tinyverse_",
            ".github/",
            "crates/",
            "README.",
            "Cargo.",
        ];
        return PATH_PREFIXES
            .iter()
            .any(|prefix| trimmed.starts_with(prefix));
    }

    false
}

fn display_path(path: &str, app: &App) -> String {
    let mut value = path.replace('\\', "/");
    let root = app
        .chat
        .workspace_root()
        .to_string_lossy()
        .replace('\\', "/");
    if !root.is_empty()
        && value.starts_with(&root)
        && let Some(stripped) = value.strip_prefix(&root)
    {
        value = stripped.trim_start_matches('/').to_owned();
    }

    truncate_to(&value, 64)
}

fn first_meaningful_line(value: &str) -> Option<&str> {
    value
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('{') && !line.starts_with('['))
}

fn parse_todo_items(raw: &str) -> Option<Vec<TodoRenderItem>> {
    let payload = serde_json::from_str::<Value>(raw).ok()?;
    let list = if let Some(value) = payload.get("todos") {
        value.as_array()?
    } else {
        payload.as_array()?
    };

    let mut items = Vec::new();
    for entry in list {
        let Some(object) = entry.as_object() else {
            continue;
        };
        let content = object
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim();
        if content.is_empty() {
            continue;
        }

        let status = object
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("pending")
            .trim()
            .to_ascii_lowercase();
        let priority = object
            .get("priority")
            .and_then(Value::as_str)
            .unwrap_or("medium")
            .trim()
            .to_ascii_lowercase();

        items.push(TodoRenderItem {
            content: content.to_owned(),
            status,
            priority,
        });
    }

    if items.is_empty() { None } else { Some(items) }
}

fn todo_preview_label(items: &[TodoRenderItem]) -> String {
    let total = items.len();
    if total == 1 {
        let label = truncate_to(items[0].content.trim(), 32);
        let status = items[0].status.as_str();
        return format!("1 todo ({status}: {label})");
    }

    let completed = items
        .iter()
        .filter(|item| item.status.eq_ignore_ascii_case("completed"))
        .count();
    let in_progress = items
        .iter()
        .filter(|item| item.status.eq_ignore_ascii_case("in_progress"))
        .count();
    let high_priority = items
        .iter()
        .filter(|item| item.priority.eq_ignore_ascii_case("high"))
        .count();
    let priority_suffix = if high_priority > 0 {
        format!(", {high_priority} high")
    } else {
        String::new()
    };

    if completed == total {
        return format!("{total} todos completed{priority_suffix}");
    }
    if in_progress > 0 {
        return format!("{total} todos ({completed} done, {in_progress} active{priority_suffix})");
    }
    format!("{total} todos ({completed} done{priority_suffix})")
}

fn is_hidden_noise_line(value: &str) -> bool {
    let trimmed = value.trim_start();
    trimmed.starts_with("step finished |")
        || trimmed.starts_with("shell metadata:")
        || trimmed.starts_with("shell call:")
        || trimmed.starts_with("shell status:")
}

#[cfg(test)]
mod tests {
    use super::{PatchOp, parse_patch_summary, parse_todo_items, todo_preview_label};

    #[test]
    fn parses_todo_envelope_payload() {
        let payload =
            r#"{"todos":[{"content":"Write docs","status":"in_progress","priority":"high"}]}"#;
        let items = parse_todo_items(payload).expect("expected todo list");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].content, "Write docs");
        assert_eq!(items[0].status, "in_progress");
        assert_eq!(items[0].priority, "high");
    }

    #[test]
    fn parses_todo_array_payload() {
        let payload = r#"[{"content":"Ship feature","status":"completed","priority":"medium"}]"#;
        let items = parse_todo_items(payload).expect("expected todo list");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].content, "Ship feature");
        assert_eq!(items[0].status, "completed");
        assert_eq!(items[0].priority, "medium");
    }

    #[test]
    fn formats_todo_preview_summary() {
        let payload = r#"[{"content":"A","status":"completed","priority":"high"},{"content":"B","status":"completed","priority":"low"}]"#;
        let items = parse_todo_items(payload).expect("expected todo list");
        assert_eq!(todo_preview_label(&items), "2 todos completed, 1 high");
    }

    #[test]
    fn parses_patch_from_raw_text() {
        let raw = "*** Begin Patch\n*** Update File: src/main.rs\n@@ -1,3 +1,3 @@\n some code\n*** Add File: src/new.rs\n+new file content\n*** End Patch";
        let summary = parse_patch_summary(raw).expect("expected patch summary");
        assert_eq!(summary.files.len(), 2);
        assert_eq!(summary.files[0].op, PatchOp::Update);
        assert_eq!(summary.files[0].path, "src/main.rs");
        assert_eq!(summary.files[1].op, PatchOp::Add);
        assert_eq!(summary.files[1].path, "src/new.rs");
    }

    #[test]
    fn parses_patch_from_json_wrapper() {
        let raw = r#"{"patchText": "*** Begin Patch\n*** Delete File: old.rs\n*** Update File: lib.rs\n@@ -1 +1 @@\n-old\n+new\n*** End Patch"}"#;
        let summary = parse_patch_summary(raw).expect("expected patch summary");
        assert_eq!(summary.files.len(), 2);
        assert_eq!(summary.files[0].op, PatchOp::Delete);
        assert_eq!(summary.files[0].path, "old.rs");
        assert_eq!(summary.files[1].op, PatchOp::Update);
        assert_eq!(summary.files[1].path, "lib.rs");
    }

    #[test]
    fn patch_compact_label_single_file() {
        let raw = "*** Begin Patch\n*** Update File: src/app.rs\n@@ -1 +1 @@\n-old\n+new";
        let summary = parse_patch_summary(raw).expect("expected patch summary");
        assert_eq!(summary.compact_label(), "~1 app.rs");
    }

    #[test]
    fn patch_compact_label_multiple_ops() {
        let raw = "*** Add File: a.rs\n*** Update File: b.rs\n*** Delete File: c.rs\n*** Update File: d.rs";
        let summary = parse_patch_summary(raw).expect("expected patch summary");
        assert_eq!(summary.compact_label(), "+1 ~2 -1 (4 files)");
    }

    #[test]
    fn patch_returns_none_for_non_patch() {
        assert!(parse_patch_summary("just some random text").is_none());
        assert!(parse_patch_summary(r#"{"key": "value"}"#).is_none());
    }
}
