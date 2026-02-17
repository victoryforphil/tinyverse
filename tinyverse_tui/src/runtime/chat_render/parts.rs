use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::app::App;
use crate::chat::ChatMessagePart;

use super::types::RenderedChatLine;
use crate::runtime::helpers::truncate_to;

const EXPANDED_LINE_CAP: usize = 120;
const TRUNCATED_HINT: &str = "    ... (press enter for full detail)";

fn collapsible_header(
    label: &str,
    kind_tag: &str,
    expanded: bool,
    fg: Color,
    width: u16,
    is_focused: bool,
    part_key: Option<String>,
    app: &App,
) -> RenderedChatLine {
    let chevron = if expanded { "▼" } else { "▶" };
    let left = format!("  {chevron} {label}");
    let tag = format!(" {kind_tag} ");
    let used = left.chars().count() + tag.chars().count();
    let spacer = " ".repeat((width as usize).saturating_sub(used).max(1));

    let bg = if is_focused {
        app.theme.chat_collapsible_focused_bg
    } else {
        app.theme.chat_collapsible_bg
    };

    RenderedChatLine {
        line: Line::from(vec![
            Span::styled(
                left,
                Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
            ),
            Span::styled(spacer, Style::default().bg(bg)),
            Span::styled(
                tag,
                Style::default()
                    .fg(fg)
                    .bg(app.theme.chat_collapsible_tag_bg)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        toggle_key: part_key,
    }
}

pub(super) fn render_chat_part_lines(
    part: &ChatMessagePart,
    app: &App,
    width: u16,
    part_key: Option<String>,
    expanded: bool,
) -> Vec<RenderedChatLine> {
    let max = width.saturating_sub(8) as usize;
    match part {
        ChatMessagePart::Text(value) => {
            if is_hidden_noise_line(value) {
                return Vec::new();
            }

            value
                .lines()
                .map(|line| RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("  {line}"),
                        Style::default().fg(app.theme.text_secondary),
                        app,
                    ),
                    toggle_key: None,
                })
                .collect()
        }
        ChatMessagePart::Markdown(value) => value
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let trimmed = line.trim_start();
                let style = if trimmed.starts_with('#') {
                    Style::default()
                        .fg(app.theme.text_primary)
                        .add_modifier(Modifier::BOLD)
                } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    Style::default().fg(app.theme.text_secondary)
                } else {
                    Style::default().fg(app.theme.text_secondary)
                };

                RenderedChatLine {
                    line: line_with_path_pills(&format!("  {line}"), style, app),
                    toggle_key: if index == 0 && part_key.is_some() && part.is_collapsible() {
                        part_key.clone()
                    } else {
                        None
                    },
                }
            })
            .collect(),
        ChatMessagePart::Thinking(value) => {
            let mut out = vec![collapsible_header(
                "Reasoning",
                "thinking",
                expanded,
                app.theme.pill_muted_fg,
                width,
                is_focused_part(app, part_key.as_deref()),
                part_key.clone(),
                app,
            )];

            if !expanded {
                let preview = value.lines().next().unwrap_or("(no detail)");
                out.push(RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted),
                        app,
                    ),
                    toggle_key: None,
                });
                return out;
            }

            let mut count = 0usize;
            for line in value.lines() {
                if count >= EXPANDED_LINE_CAP {
                    out.push(truncated_hint_line(app));
                    break;
                }
                out.push(RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("    {line}"),
                        Style::default().fg(app.theme.text_muted),
                        app,
                    ),
                    toggle_key: None,
                });
                count += 1;
            }
            out
        }
        ChatMessagePart::Code { language, code } => {
            let label = language.as_deref().unwrap_or("text");
            let mut out = vec![collapsible_header(
                &format!("code ({label})"),
                "code",
                expanded,
                app.theme.pill_accent_fg,
                width,
                is_focused_part(app, part_key.as_deref()),
                part_key.clone(),
                app,
            )];

            if !expanded {
                let preview = code.lines().next().unwrap_or("(empty)");
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted),
                    )),
                    toggle_key: None,
                });
                return out;
            }

            let mut count = 0usize;
            for line in code.lines() {
                if count >= EXPANDED_LINE_CAP {
                    out.push(truncated_hint_line(app));
                    break;
                }
                out.push(RenderedChatLine {
                    line: Line::from(vec![
                        Span::styled("    │ ", Style::default().fg(app.theme.chat_separator_fg)),
                        Span::styled(line.to_owned(), Style::default().fg(app.theme.text_primary)),
                    ]),
                    toggle_key: None,
                });
                count += 1;
            }
            out
        }
        ChatMessagePart::ToolCall {
            name,
            input,
            output,
        } => {
            let mut out = vec![collapsible_header(
                &format!("tool {name}"),
                "tool",
                expanded,
                app.theme.pill_info_fg,
                width,
                is_focused_part(app, part_key.as_deref()),
                part_key.clone(),
                app,
            )];

            if !expanded {
                let preview = input
                    .as_deref()
                    .and_then(first_meaningful_line)
                    .or_else(|| output.as_deref().and_then(first_meaningful_line))
                    .unwrap_or("(details)");
                out.push(RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted),
                        app,
                    ),
                    toggle_key: None,
                });
                return out;
            }

            if let Some(input) = input {
                out.push(RenderedChatLine {
                    line: Line::from(vec![
                        Span::styled(
                            " IN ",
                            Style::default()
                                .fg(app.theme.pill_muted_fg)
                                .bg(app.theme.pill_muted_bg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" ", Style::default()),
                    ]),
                    toggle_key: None,
                });

                let mut count = 0usize;
                for line in input.lines() {
                    if count >= EXPANDED_LINE_CAP / 2 {
                        out.push(truncated_hint_line(app));
                        break;
                    }
                    out.push(RenderedChatLine {
                        line: line_with_path_pills(
                            &format!("      {}", truncate_to(line, max)),
                            Style::default().fg(app.theme.text_muted),
                            app,
                        ),
                        toggle_key: None,
                    });
                    count += 1;
                }
            }

            if let Some(output) = output {
                out.push(RenderedChatLine {
                    line: Line::from(vec![
                        Span::styled(
                            " OUT ",
                            Style::default()
                                .fg(app.theme.pill_muted_fg)
                                .bg(app.theme.pill_muted_bg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" ", Style::default()),
                    ]),
                    toggle_key: None,
                });

                let mut count = 0usize;
                for line in output.lines() {
                    if count >= EXPANDED_LINE_CAP {
                        out.push(truncated_hint_line(app));
                        break;
                    }
                    out.push(RenderedChatLine {
                        line: line_with_path_pills(
                            &format!("      {}", truncate_to(line, max)),
                            Style::default().fg(app.theme.text_secondary),
                            app,
                        ),
                        toggle_key: None,
                    });
                    count += 1;
                }
            }
            out
        }
        ChatMessagePart::ShellCommand(value) => {
            let mut spans = vec![
                Span::styled(
                    " CMD ",
                    Style::default()
                        .fg(app.theme.pill_info_fg)
                        .bg(app.theme.pill_info_bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled("$ ", Style::default().fg(app.theme.pill_info_fg)),
            ];
            spans.extend(spans_with_path_pills(
                &truncate_to(value, max),
                Style::default().fg(app.theme.text_primary),
                app,
            ));

            vec![RenderedChatLine {
                line: Line::from(spans),
                toggle_key: None,
            }]
        }
        ChatMessagePart::ShellOutput { output, exit_code } => {
            let mut out = vec![collapsible_header(
                "shell output",
                "shell",
                expanded,
                app.theme.text_muted,
                width,
                is_focused_part(app, part_key.as_deref()),
                part_key,
                app,
            )];

            if !expanded {
                let preview = output.lines().next().unwrap_or("(empty)");
                let suffix = exit_code
                    .map(|code| format!("  exit {code}"))
                    .unwrap_or_default();
                out.push(RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("    {}{}", truncate_to(preview, max), suffix),
                        Style::default().fg(app.theme.text_muted),
                        app,
                    ),
                    toggle_key: None,
                });
                return out;
            }

            let mut count = 0usize;
            for line in output.lines() {
                if count >= EXPANDED_LINE_CAP {
                    out.push(truncated_hint_line(app));
                    break;
                }
                out.push(RenderedChatLine {
                    line: line_with_path_pills(
                        &format!("    │ {}", truncate_to(line, max)),
                        Style::default().fg(app.theme.text_muted),
                        app,
                    ),
                    toggle_key: None,
                });
                count += 1;
            }

            if let Some(code) = exit_code {
                let style = if *code == 0 {
                    Style::default().fg(app.theme.pill_ok_fg)
                } else {
                    Style::default()
                        .fg(app.theme.pill_err_fg)
                        .add_modifier(Modifier::BOLD)
                };
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(format!("    exit {code}"), style)),
                    toggle_key: None,
                });
            }
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

fn truncated_hint_line(app: &App) -> RenderedChatLine {
    RenderedChatLine {
        line: Line::from(Span::styled(
            TRUNCATED_HINT,
            Style::default()
                .fg(app.theme.pill_muted_fg)
                .add_modifier(Modifier::ITALIC),
        )),
        toggle_key: None,
    }
}

fn is_focused_part(app: &App, part_key: Option<&str>) -> bool {
    part_key.is_some_and(|key| app.chat.focused_part_key() == Some(key))
}

fn line_with_path_pills(raw: &str, base_style: Style, app: &App) -> Line<'static> {
    Line::from(spans_with_path_pills(raw, base_style, app))
}

fn spans_with_path_pills(raw: &str, base_style: Style, app: &App) -> Vec<Span<'static>> {
    if let Some((prefix, path, suffix)) = extract_json_filepath(raw) {
        let display = display_path(&path, app);
        let mut spans = Vec::new();
        if !prefix.is_empty() {
            spans.push(Span::styled(prefix, base_style));
        }
        spans.push(Span::styled(
            format!(" {display} "),
            Style::default()
                .fg(app.theme.path_pill_fg)
                .bg(app.theme.path_pill_bg)
                .add_modifier(Modifier::BOLD),
        ));
        if !suffix.is_empty() {
            spans.push(Span::styled(suffix, base_style));
        }
        return spans;
    }

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
            spans.push(Span::styled(
                format!(" {} ", display_path(core, app)),
                Style::default()
                    .fg(app.theme.path_pill_fg)
                    .bg(app.theme.path_pill_bg)
                    .add_modifier(Modifier::BOLD),
            ));
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

fn is_hidden_noise_line(value: &str) -> bool {
    let trimmed = value.trim_start();
    trimmed.starts_with("step finished |")
        || trimmed.starts_with("shell metadata:")
        || trimmed.starts_with("shell call:")
        || trimmed.starts_with("shell status:")
}
