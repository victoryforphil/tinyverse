use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::app::App;
use crate::chat::ChatMessagePart;

use crate::runtime::helpers::{centered_rect, inset_rect};

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

    let Some((header, lines)) = detail_lines_for_part_key(app, &part_key) else {
        app.chat.close_detail_modal();
        app.layout.chat.detail_modal_rect = None;
        app.layout.chat.detail_modal_body_rect = None;
        return;
    };

    let popup = centered_rect(
        parent.width.saturating_sub(6).max(40),
        parent.height.saturating_sub(6).max(10),
        parent,
    );
    app.layout.chat.detail_modal_rect = Some(popup);

    frame.render_widget(Clear, popup);
    let block = Block::default()
        .title(" Detail ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_focused_border));
    let inner = inset_rect(block.inner(popup), 1, 0);
    frame.render_widget(block, popup);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);
    let meta_area = sections[0];
    let body_area = sections[1];
    let hint_area = sections[2];
    app.layout.chat.detail_modal_body_rect = Some(body_area);

    frame.render_widget(
        Paragraph::new(vec![
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
        ]),
        meta_area,
    );

    let max_lines = body_area.height as usize;
    let overflow = lines.len().saturating_sub(max_lines);
    let scroll = app.chat.detail_scroll_lines() as usize;
    let start = overflow.saturating_sub(scroll);
    let visible = lines
        .into_iter()
        .skip(start)
        .take(max_lines)
        .collect::<Vec<_>>();

    frame.render_widget(
        Paragraph::new(visible).wrap(Wrap { trim: false }),
        body_area,
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "esc",
                Style::default()
                    .fg(app.theme.key_hint_key_fg)
                    .bg(app.theme.key_hint_key_bg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" close  ", Style::default().fg(app.theme.text_muted)),
            Span::styled(
                "j/k",
                Style::default()
                    .fg(app.theme.key_hint_key_fg)
                    .bg(app.theme.key_hint_key_bg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" scroll", Style::default().fg(app.theme.text_muted)),
        ])),
        hint_area,
    );
}

fn detail_lines_for_part_key(app: &App, target_key: &str) -> Option<(String, Vec<Line<'static>>)> {
    for (message_index, message) in app.chat.messages.iter().enumerate() {
        for (part_index, part) in message.parts.iter().enumerate() {
            let part_key = app.chat.part_key(message, message_index, part_index);
            if part_key != target_key {
                continue;
            }

            let header = format!("{}  {}", message.role.label(), message.created_at);
            let lines = render_detail_body(part, app, 0);
            return Some((header, lines));
        }
    }

    None
}

fn render_detail_body(part: &ChatMessagePart, app: &App, indent: usize) -> Vec<Line<'static>> {
    let prefix = " ".repeat(indent);
    let mut lines = Vec::new();
    match part {
        ChatMessagePart::Text(value) | ChatMessagePart::Markdown(value) => {
            for line in value.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}{line}"),
                    Style::default().fg(app.theme.text_secondary),
                )));
            }
        }
        ChatMessagePart::Thinking(value) => {
            lines.push(Line::from(Span::styled(
                format!("{prefix}THINKING"),
                Style::default()
                    .fg(app.theme.pill_muted_fg)
                    .add_modifier(Modifier::ITALIC),
            )));
            lines.push(Line::from(""));
            for line in value.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}{line}"),
                    Style::default().fg(app.theme.text_muted),
                )));
            }
        }
        ChatMessagePart::Code { language, code } => {
            let label = language.as_deref().unwrap_or("text");
            lines.push(Line::from(Span::styled(
                format!("{prefix}code ({label})"),
                Style::default().fg(app.theme.pill_accent_fg),
            )));
            lines.push(Line::from(""));
            for line in code.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}{line}"),
                    Style::default().fg(app.theme.text_primary),
                )));
            }
        }
        ChatMessagePart::ToolCall {
            name,
            input,
            output,
        } => {
            lines.push(Line::from(Span::styled(
                format!("{prefix}TOOL {name}"),
                Style::default().fg(app.theme.pill_info_fg),
            )));
            lines.push(Line::from(""));
            if let Some(input) = input {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}Input:"),
                    Style::default()
                        .fg(app.theme.pill_muted_fg)
                        .add_modifier(Modifier::BOLD),
                )));
                for line in input.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("{prefix}  {line}"),
                        Style::default().fg(app.theme.text_muted),
                    )));
                }
                lines.push(Line::from(""));
            }
            if let Some(output) = output {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}Output:"),
                    Style::default()
                        .fg(app.theme.pill_accent_fg)
                        .add_modifier(Modifier::BOLD),
                )));
                for line in output.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("{prefix}  {line}"),
                        Style::default().fg(app.theme.text_secondary),
                    )));
                }
            }
        }
        ChatMessagePart::ShellCommand(value) => {
            lines.push(Line::from(Span::styled(
                format!("{prefix}$ {value}"),
                Style::default().fg(app.theme.pill_info_fg),
            )));
        }
        ChatMessagePart::ShellOutput { output, exit_code } => {
            lines.push(Line::from(Span::styled(
                format!("{prefix}shell output"),
                Style::default().fg(app.theme.text_muted),
            )));
            lines.push(Line::from(""));
            for line in output.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{prefix}{line}"),
                    Style::default().fg(app.theme.text_muted),
                )));
            }
            if let Some(code) = exit_code {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("{prefix}exit {code}"),
                    if *code == 0 {
                        Style::default().fg(app.theme.pill_ok_fg)
                    } else {
                        Style::default().fg(app.theme.pill_err_fg)
                    },
                )));
            }
        }
        ChatMessagePart::Error(value) => {
            lines.push(Line::from(Span::styled(
                format!("{prefix}ERROR: {value}"),
                Style::default().fg(app.theme.pill_err_fg),
            )));
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
