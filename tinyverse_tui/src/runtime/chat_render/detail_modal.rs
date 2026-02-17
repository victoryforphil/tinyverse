use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

use crate::app::App;
use crate::chat::ChatMessagePart;
use tinyverse_tui_components::{
    CodeViewMode, DiffLine, DiffLineNumberMode, DiffViewComponent, DiffViewProps, anchored_rect,
    inset_rect,
};

use super::parts::{PatchOp, extract_patch_text, parse_patch_summary};

/// Maximum height the anchored detail card may occupy (fraction of parent).
const CARD_MAX_HEIGHT_RATIO: f32 = 0.5;
/// Minimum card width.
const CARD_MIN_WIDTH: u16 = 34;
/// Maximum card width.
const CARD_MAX_WIDTH: u16 = 68;
/// Vertical padding between the anchor row and the card.
const ANCHOR_GAP: u16 = 1;

pub(super) fn render_chat_detail_modal(frame: &mut Frame, parent: Rect, app: &mut App) {
    if !app.chat.is_detail_modal_open() {
        app.layout.chat.detail_modal_rect = None;
        app.layout.chat.detail_modal_body_rect = None;
        return;
    }

    let Some(part_key) = app.chat.detail_part_key().map(str::to_owned) else {
        app.layout.chat.detail_modal_rect = None;
        app.layout.chat.detail_modal_body_rect = None;
        return;
    };

    let Some((header, kind_label, lines, diff_lines)) = detail_lines_for_part_key(app, &part_key)
    else {
        app.chat.close_detail_modal();
        app.layout.chat.detail_modal_rect = None;
        app.layout.chat.detail_modal_body_rect = None;
        return;
    };

    // ── Determine anchor position from part hitbox ──────────────────
    let (anchor_x, anchor_y) = app
        .layout
        .chat
        .part_toggle_hitboxes
        .iter()
        .find(|h| h.part_key == part_key)
        .map(|h| {
            (
                h.rect.x,
                h.rect
                    .y
                    .saturating_add(h.rect.height)
                    .saturating_add(ANCHOR_GAP),
            )
        })
        .unwrap_or_else(|| {
            // Fallback: center-ish within the parent
            (
                parent.x.saturating_add(parent.width / 4),
                parent.y.saturating_add(parent.height / 3),
            )
        });

    // ── Compute card dimensions ────────────────────────────────────
    let max_h = ((parent.height as f32) * CARD_MAX_HEIGHT_RATIO) as u16;
    let header_rows = 2u16; // header line + part key line
    let divider_rows = 1u16;
    let detail_line_count = diff_lines.as_ref().map_or(lines.len(), Vec::len);
    let body_rows =
        (detail_line_count as u16).clamp(3, max_h.saturating_sub(header_rows + divider_rows + 2));
    let card_height = (header_rows + divider_rows + body_rows + 2)
        .min(max_h)
        .max(8);
    let body_width = diff_lines
        .as_ref()
        .map(|lines| {
            lines
                .iter()
                .map(rendered_diff_line_width)
                .max()
                .unwrap_or(24)
        })
        .unwrap_or_else(|| lines.iter().map(rendered_line_width).max().unwrap_or(24))
        as u16;
    let header_width = header.chars().count() as u16 + kind_label.chars().count() as u16 + 14;
    let desired_width = body_width.max(header_width).saturating_add(6);
    let max_allowed = parent.width.saturating_sub(4).max(CARD_MIN_WIDTH);
    let card_width = desired_width.clamp(CARD_MIN_WIDTH, CARD_MAX_WIDTH.min(max_allowed));

    let card_area = anchored_rect(card_width, card_height, anchor_x, anchor_y, parent);
    frame.render_widget(Clear, card_area);

    // ── Card chrome ────────────────────────────────────────────────
    let block = Block::default()
        .title(format!(" {} Detail ", kind_label))
        .title_style(
            Style::default()
                .fg(app.theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.pane_focused_border));
    let inner = inset_rect(block.inner(card_area), 1, 0);
    frame.render_widget(block, card_area);

    if inner.width == 0 || inner.height == 0 {
        app.layout.chat.detail_modal_rect = Some(card_area);
        app.layout.chat.detail_modal_body_rect = Some(inner);
        return;
    }

    // ── Internal layout: header / divider / body / hint ────────────
    let real_header_h = header_rows.min(inner.height);
    let real_body_h = inner
        .height
        .saturating_sub(real_header_h)
        .saturating_sub(divider_rows)
        .max(1);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(real_header_h),
            Constraint::Length(divider_rows),
            Constraint::Length(real_body_h),
        ])
        .split(inner);

    let header_area = sections[0];
    let divider_area = sections[1];
    let body_area = sections[2];

    // ── Header ─────────────────────────────────────────────────────
    let header_lines = vec![
        Line::from(Span::styled(
            header,
            Style::default()
                .fg(app.theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            part_key,
            Style::default().fg(app.theme.text_muted),
        )),
    ];
    frame.render_widget(Paragraph::new(header_lines), header_area);

    // ── Divider ────────────────────────────────────────────────────
    let rule_len = divider_area.width as usize;
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(rule_len),
            Style::default().fg(app.theme.chat_separator_fg),
        ))),
        divider_area,
    );

    // ── Scrollable body ────────────────────────────────────────────
    let max_lines = body_area.height as usize;
    let scroll = app.chat.detail_scroll_lines() as usize;

    if let Some(diff_lines) = diff_lines.as_ref() {
        let total = diff_lines.len();
        let overflow = total.saturating_sub(max_lines);
        let start = overflow.saturating_sub(scroll);
        let mode = if body_area.width < 56 {
            CodeViewMode::Compact
        } else {
            CodeViewMode::Normal
        };

        DiffViewComponent::render(
            frame,
            body_area,
            &app.theme,
            DiffViewProps {
                lines: diff_lines,
                scroll: start,
                horizontal_offset: 0,
                title: None,
                mode,
                line_number_mode: DiffLineNumberMode::Both,
                empty_message: "(no diff lines)",
            },
        );
    } else {
        let total = lines.len();
        let overflow = total.saturating_sub(max_lines);
        let start = overflow.saturating_sub(scroll);
        let visible: Vec<Line<'static>> = if start == 0 && total <= max_lines {
            lines
        } else {
            lines.into_iter().skip(start).take(max_lines).collect()
        };
        frame.render_widget(
            Paragraph::new(visible).wrap(Wrap { trim: false }),
            body_area,
        );
    }

    app.layout.chat.detail_modal_rect = Some(card_area);
    app.layout.chat.detail_modal_body_rect = Some(body_area);
}

fn detail_lines_for_part_key(
    app: &App,
    target_key: &str,
) -> Option<(
    String,
    &'static str,
    Vec<Line<'static>>,
    Option<Vec<DiffLine>>,
)> {
    for (message_index, message) in app.chat.messages.iter().enumerate() {
        for (part_index, part) in message.parts.iter().enumerate() {
            let part_key = app.chat.part_key(message, message_index, part_index);
            if part_key != target_key {
                continue;
            }

            let header = format!("{}  {}", message.role.label(), message.created_at);
            let kind_label = part_kind_label(part);
            let lines = render_detail_body(part, app, 0);
            let diff_lines = render_detail_diff_lines(part);
            return Some((header, kind_label, lines, diff_lines));
        }
    }

    None
}

fn render_detail_diff_lines(part: &ChatMessagePart) -> Option<Vec<DiffLine>> {
    match part {
        ChatMessagePart::Code { language, code } if is_diff_language(language.as_deref()) => {
            Some(DiffViewComponent::parse_unified(code))
        }
        ChatMessagePart::ToolCall {
            name,
            input: Some(input),
            ..
        } if name.eq_ignore_ascii_case("apply_patch") => {
            extract_patch_text(input).map(|patch| DiffViewComponent::parse_unified(&patch))
        }
        _ => None,
    }
}

fn is_diff_language(language: Option<&str>) -> bool {
    matches!(
        language.map(str::trim).map(str::to_ascii_lowercase),
        Some(lang) if matches!(lang.as_str(), "diff" | "patch" | "udiff")
    )
}

fn part_kind_label(part: &ChatMessagePart) -> &'static str {
    match part {
        ChatMessagePart::Text(_) => "Text",
        ChatMessagePart::Markdown(_) => "Markdown",
        ChatMessagePart::Thinking(_) => "Thinking",
        ChatMessagePart::Code { .. } => "Code",
        ChatMessagePart::ToolCall { name, .. } => {
            if name.eq_ignore_ascii_case("apply_patch") {
                "Patch"
            } else {
                "Tool"
            }
        }
        ChatMessagePart::ShellCommand(_) => "Shell",
        ChatMessagePart::ShellOutput { .. } => "Output",
        ChatMessagePart::Error(_) => "Error",
    }
}

fn render_detail_body(part: &ChatMessagePart, app: &App, indent: usize) -> Vec<Line<'static>> {
    let prefix: &str = if indent == 0 { "" } else { &" ".repeat(indent) };
    let mut lines = Vec::new();
    match part {
        ChatMessagePart::Text(value) | ChatMessagePart::Markdown(value) => {
            let style = Style::default().fg(app.theme.text_secondary);
            lines.reserve(value.lines().count());
            for line in value.lines() {
                let mut s = String::with_capacity(prefix.len() + line.len());
                s.push_str(prefix);
                s.push_str(line);
                lines.push(Line::from(Span::styled(s, style)));
            }
        }
        ChatMessagePart::Thinking(value) => {
            lines.push(Line::from(vec![Span::styled(
                format!("{prefix} THINKING "),
                Style::default()
                    .fg(app.theme.pill_muted_fg)
                    .bg(app.theme.pill_muted_bg)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            let style = Style::default().fg(app.theme.text_muted);
            for line in value.lines() {
                let mut s = String::with_capacity(prefix.len() + line.len());
                s.push_str(prefix);
                s.push_str(line);
                lines.push(Line::from(Span::styled(s, style)));
            }
        }
        ChatMessagePart::Code { language, code } => {
            let label = language.as_deref().unwrap_or("text");
            lines.push(Line::from(vec![Span::styled(
                format!("{prefix} {label} "),
                Style::default()
                    .fg(app.theme.pill_accent_fg)
                    .bg(app.theme.pill_accent_bg)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            let code_style = Style::default()
                .fg(app.theme.text_primary)
                .bg(app.theme.chat_code_bg);
            for line in code.lines() {
                let mut s = String::with_capacity(prefix.len() + line.len());
                s.push_str(prefix);
                s.push_str(line);
                lines.push(Line::from(Span::styled(s, code_style)));
            }
        }
        ChatMessagePart::ToolCall {
            name,
            input,
            output,
        } => {
            let is_patch_tool = name.eq_ignore_ascii_case("apply_patch");
            let patch_summary = if is_patch_tool {
                input.as_deref().and_then(parse_patch_summary)
            } else {
                None
            };

            if let Some(ref summary) = patch_summary {
                // ── Rich apply_patch rendering ─────────────────────
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{prefix} PATCH "),
                        Style::default()
                            .fg(app.theme.pill_info_fg)
                            .bg(app.theme.pill_info_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" apply_patch"),
                        Style::default()
                            .fg(app.theme.pill_info_fg)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(""));

                // Operation counts summary line
                let adds = summary.count(PatchOp::Add);
                let updates = summary.count(PatchOp::Update);
                let deletes = summary.count(PatchOp::Delete);

                let mut count_spans: Vec<Span<'static>> = Vec::new();
                count_spans.push(Span::styled(
                    format!("{prefix}  "),
                    Style::default().fg(app.theme.text_muted),
                ));
                if adds > 0 {
                    count_spans.push(Span::styled(
                        format!(" +{adds} add "),
                        Style::default()
                            .fg(app.theme.pill_ok_fg)
                            .add_modifier(Modifier::BOLD),
                    ));
                    count_spans.push(Span::styled("  ", Style::default()));
                }
                if updates > 0 {
                    count_spans.push(Span::styled(
                        format!(" ~{updates} update "),
                        Style::default()
                            .fg(app.theme.pill_info_fg)
                            .add_modifier(Modifier::BOLD),
                    ));
                    count_spans.push(Span::styled("  ", Style::default()));
                }
                if deletes > 0 {
                    count_spans.push(Span::styled(
                        format!(" -{deletes} delete "),
                        Style::default()
                            .fg(app.theme.pill_err_fg)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
                lines.push(Line::from(count_spans));
                lines.push(Line::from(""));

                // File list
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{prefix}Files"),
                        Style::default()
                            .fg(app.theme.pill_muted_fg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" ─", Style::default().fg(app.theme.chat_separator_fg)),
                ]));

                let max_files_shown = 12;
                let total_files = summary.files.len();
                for (i, entry) in summary.files.iter().enumerate() {
                    if i >= max_files_shown {
                        lines.push(Line::from(Span::styled(
                            format!("{prefix}  … and {} more", total_files - max_files_shown),
                            Style::default().fg(app.theme.text_muted),
                        )));
                        break;
                    }
                    let op_marker = match entry.op {
                        PatchOp::Add => "+",
                        PatchOp::Update => "~",
                        PatchOp::Delete => "-",
                    };
                    let op_color = match entry.op {
                        PatchOp::Add => app.theme.pill_ok_fg,
                        PatchOp::Update => app.theme.pill_info_fg,
                        PatchOp::Delete => app.theme.pill_err_fg,
                    };
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{prefix}  {op_marker} "),
                            Style::default().fg(op_color).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            entry.path.clone(),
                            Style::default()
                                .fg(app.theme.path_pill_fg)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                    ]));
                }

                // If there's output (tool result), show it below
                if let Some(output) = output {
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{prefix}Result"),
                            Style::default()
                                .fg(app.theme.pill_accent_fg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" ─", Style::default().fg(app.theme.chat_separator_fg)),
                    ]));
                    let output_style = Style::default().fg(app.theme.text_secondary);
                    for line in output.lines().take(20) {
                        let mut s = String::with_capacity(prefix.len() + 2 + line.len());
                        s.push_str(prefix);
                        s.push_str("  ");
                        s.push_str(line);
                        lines.push(Line::from(Span::styled(s, output_style)));
                    }
                }
            } else {
                // ── Generic tool rendering (unchanged) ─────────────
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{prefix} TOOL "),
                        Style::default()
                            .fg(app.theme.pill_info_fg)
                            .bg(app.theme.pill_info_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" {name}"),
                        Style::default()
                            .fg(app.theme.pill_info_fg)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(""));
                if let Some(input) = input {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{prefix}Input"),
                            Style::default()
                                .fg(app.theme.pill_muted_fg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" ─", Style::default().fg(app.theme.chat_separator_fg)),
                    ]));
                    let input_style = Style::default().fg(app.theme.text_muted);
                    for line in input.lines() {
                        let mut s = String::with_capacity(prefix.len() + 2 + line.len());
                        s.push_str(prefix);
                        s.push_str("  ");
                        s.push_str(line);
                        lines.push(Line::from(Span::styled(s, input_style)));
                    }
                    lines.push(Line::from(""));
                }
                if let Some(output) = output {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{prefix}Output"),
                            Style::default()
                                .fg(app.theme.pill_accent_fg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" ─", Style::default().fg(app.theme.chat_separator_fg)),
                    ]));
                    let output_style = Style::default().fg(app.theme.text_secondary);
                    for line in output.lines() {
                        let mut s = String::with_capacity(prefix.len() + 2 + line.len());
                        s.push_str(prefix);
                        s.push_str("  ");
                        s.push_str(line);
                        lines.push(Line::from(Span::styled(s, output_style)));
                    }
                }
            }
        }
        ChatMessagePart::ShellCommand(value) => {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{prefix}$ "),
                    Style::default()
                        .fg(app.theme.pill_info_fg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    value.to_owned(),
                    Style::default().fg(app.theme.text_primary),
                ),
            ]));
        }
        ChatMessagePart::ShellOutput { output, exit_code } => {
            lines.push(Line::from(vec![Span::styled(
                format!("{prefix} shell output "),
                Style::default()
                    .fg(app.theme.pill_muted_fg)
                    .bg(app.theme.pill_muted_bg)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            let out_style = Style::default().fg(app.theme.text_muted);
            for line in output.lines() {
                let mut s = String::with_capacity(prefix.len() + line.len());
                s.push_str(prefix);
                s.push_str(line);
                lines.push(Line::from(Span::styled(s, out_style)));
            }
            if let Some(code) = exit_code {
                lines.push(Line::from(""));
                let exit_style = if *code == 0 {
                    Style::default()
                        .fg(app.theme.pill_ok_fg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(app.theme.pill_err_fg)
                        .add_modifier(Modifier::BOLD)
                };
                lines.push(Line::from(Span::styled(
                    format!("{prefix}exit {code}"),
                    exit_style,
                )));
            }
        }
        ChatMessagePart::Error(value) => {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{prefix} ERROR "),
                    Style::default()
                        .fg(app.theme.pill_err_fg)
                        .bg(app.theme.pill_err_bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {value}"),
                    Style::default().fg(app.theme.pill_err_fg),
                ),
            ]));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("{prefix}(empty)"),
            Style::default().fg(app.theme.text_muted),
        )));
    }

    lines
}

fn rendered_line_width(line: &Line<'static>) -> usize {
    line.spans
        .iter()
        .map(|span| span.content.chars().count())
        .sum()
}

fn rendered_diff_line_width(line: &DiffLine) -> usize {
    line.text.chars().count().saturating_add(16)
}
