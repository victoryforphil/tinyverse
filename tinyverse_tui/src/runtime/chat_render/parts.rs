use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::app::App;
use crate::chat::ChatMessagePart;

use super::types::RenderedChatLine;
use crate::runtime::helpers::truncate_to;

fn collapsible_header(
    label: &str,
    kind_tag: &str,
    expanded: bool,
    style: Style,
    width: u16,
    is_focused: bool,
    part_key: Option<String>,
) -> RenderedChatLine {
    let chevron = if expanded { "▼" } else { "▶" };
    let left = format!("  {chevron} {label}");
    let tag = format!(" {kind_tag} ");
    let used = left.chars().count() + tag.chars().count();
    let spacer = " ".repeat((width as usize).saturating_sub(used).max(1));
    let header_style = if is_focused {
        style
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    } else {
        style.add_modifier(Modifier::BOLD)
    };
    RenderedChatLine {
        line: Line::from(vec![
            Span::styled(left, header_style),
            Span::raw(spacer),
            Span::styled(
                tag,
                Style::default()
                    .fg(style.fg.unwrap_or(Color::Gray))
                    .bg(Color::Rgb(24, 24, 28))
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
    bubble_bg: Color,
) -> Vec<RenderedChatLine> {
    let max = width.saturating_sub(6) as usize;
    match part {
        ChatMessagePart::Text(value) => {
            if is_hidden_noise_line(value) {
                return Vec::new();
            }
            value
                .lines()
                .map(|line| RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("  {line}"),
                        Style::default().fg(app.theme.text_secondary).bg(bubble_bg),
                    )),
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
                    Style::default().fg(app.theme.pill_info_fg)
                } else {
                    Style::default().fg(app.theme.text_secondary)
                };
                RenderedChatLine {
                    line: Line::from(Span::styled(format!("  {line}"), style.bg(bubble_bg))),
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
                Style::default().fg(app.theme.pill_muted_fg),
                width,
                part_key
                    .as_deref()
                    .is_some_and(|key| app.chat.focused_part_key() == Some(key)),
                part_key.clone(),
            )];
            if !expanded {
                let preview = value.lines().next().unwrap_or("(no detail)");
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                return out;
            }

            for line in value.lines() {
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {line}"),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
            }
            out
        }
        ChatMessagePart::Code { language, code } => {
            let mut out = Vec::new();
            let label = language.as_deref().unwrap_or("text");
            out.push(RenderedChatLine {
                line: Line::from(Span::styled(
                    format!("  {} code ({label})", if expanded { "▼" } else { "▶" }),
                    Style::default().fg(app.theme.pill_accent_fg).bg(bubble_bg),
                )),
                toggle_key: part_key.clone(),
            });

            if !expanded {
                let preview = code.lines().next().unwrap_or("(empty)");
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                return out;
            }

            for line in code.lines() {
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {line}"),
                        Style::default().fg(app.theme.text_primary).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
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
                Style::default().fg(app.theme.pill_info_fg),
                width,
                part_key
                    .as_deref()
                    .is_some_and(|key| app.chat.focused_part_key() == Some(key)),
                part_key.clone(),
            )];

            if !expanded {
                let preview = input
                    .as_deref()
                    .and_then(first_meaningful_line)
                    .or_else(|| output.as_deref().and_then(first_meaningful_line))
                    .unwrap_or("(details)");
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}", truncate_to(preview, max)),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                return out;
            }

            if let Some(input) = input {
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        "    in:",
                        Style::default()
                            .fg(app.theme.pill_muted_fg)
                            .add_modifier(Modifier::BOLD)
                            .bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                for line in input.lines() {
                    out.push(RenderedChatLine {
                        line: Line::from(Span::styled(
                            format!("      {}", truncate_to(line, max)),
                            Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                        )),
                        toggle_key: None,
                    });
                }
            }
            if let Some(output) = output {
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        "    out:",
                        Style::default()
                            .fg(app.theme.pill_accent_fg)
                            .add_modifier(Modifier::BOLD)
                            .bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                for line in output.lines() {
                    out.push(RenderedChatLine {
                        line: Line::from(Span::styled(
                            format!("      {}", truncate_to(line, max)),
                            Style::default().fg(app.theme.text_secondary).bg(bubble_bg),
                        )),
                        toggle_key: None,
                    });
                }
            }
            out
        }
        ChatMessagePart::ShellCommand(value) => vec![RenderedChatLine {
            line: Line::from(Span::styled(
                format!("  $ {}", truncate_to(value, max)),
                Style::default()
                    .fg(app.theme.pill_info_fg)
                    .add_modifier(Modifier::BOLD)
                    .bg(bubble_bg),
            )),
            toggle_key: None,
        }],
        ChatMessagePart::ShellOutput { output, exit_code } => {
            let mut out = vec![collapsible_header(
                "shell output",
                "shell",
                expanded,
                Style::default().fg(app.theme.text_muted),
                width,
                part_key
                    .as_deref()
                    .is_some_and(|key| app.chat.focused_part_key() == Some(key)),
                part_key,
            )];

            if !expanded {
                let preview = output.lines().next().unwrap_or("(empty)");
                let suffix = exit_code
                    .map(|code| format!("  exit {code}"))
                    .unwrap_or_default();
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}{}", truncate_to(preview, max), suffix),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
                return out;
            }

            for line in output.lines() {
                out.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("    {}", truncate_to(line, max)),
                        Style::default().fg(app.theme.text_muted).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
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
                    line: Line::from(Span::styled(
                        format!("    exit {code}"),
                        style.bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
            }
            out
        }
        ChatMessagePart::Error(value) => vec![RenderedChatLine {
            line: Line::from(Span::styled(
                format!("  error: {}", truncate_to(value, max)),
                Style::default()
                    .fg(app.theme.pill_err_fg)
                    .add_modifier(Modifier::BOLD)
                    .bg(bubble_bg),
            )),
            toggle_key: None,
        }],
    }
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
