use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::{App, ChatPartToggleHitbox};
use crate::chat::ChatMessageRole;

use super::parts::render_chat_part_lines;
use super::types::RenderedChatLine;
use crate::runtime::helpers::inset_rect;

pub(super) fn render_chat_messages(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Messages ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_unfocused_border));
    let inner = inset_rect(block.inner(area), 1, 0);
    frame.render_widget(block, area);

    let max_lines = inner.height as usize;
    let mut lines: Vec<RenderedChatLine> = Vec::new();
    app.layout.chat.part_toggle_hitboxes.clear();

    for (message_index, message) in app.chat.messages.iter().enumerate() {
        let (role_fg, role_label, bubble_bg) = role_palette(message.role, app);
        lines.push(RenderedChatLine {
            line: Line::from(vec![
                Span::styled(
                    "▎ ",
                    Style::default().fg(role_fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    role_label,
                    Style::default().fg(role_fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {}", message.created_at),
                    Style::default().fg(app.theme.text_muted),
                ),
            ]),
            toggle_key: None,
        });

        if message.parts.is_empty() {
            for text_line in message.text.lines() {
                lines.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("  {text_line}"),
                        Style::default().fg(app.theme.text_secondary).bg(bubble_bg),
                    )),
                    toggle_key: None,
                });
            }
        } else {
            for (part_index, part) in message.parts.iter().enumerate() {
                let part_key = app.chat.part_key(message, message_index, part_index);
                let expanded = app.chat.is_part_expanded(&part_key);
                lines.extend(render_chat_part_lines(
                    part,
                    app,
                    inner.width,
                    Some(part_key),
                    expanded,
                    bubble_bg,
                ));
            }
        }

        lines.push(RenderedChatLine {
            line: Line::from(""),
            toggle_key: None,
        });
    }

    if lines.is_empty() {
        lines.push(RenderedChatLine {
            line: Line::from(Span::styled(
                "No messages yet. Press c to compose.",
                Style::default().fg(app.theme.text_muted),
            )),
            toggle_key: None,
        });
    }

    let overflow = lines.len().saturating_sub(max_lines);
    let scroll = app.chat.scroll_lines as usize;
    let start = overflow.saturating_sub(scroll);
    let visible = lines
        .into_iter()
        .skip(start)
        .take(max_lines)
        .collect::<Vec<_>>();

    for (row, rendered) in visible.iter().enumerate() {
        if let Some(part_key) = rendered.toggle_key.as_ref() {
            app.layout
                .chat
                .part_toggle_hitboxes
                .push(ChatPartToggleHitbox {
                    rect: Rect {
                        x: inner.x,
                        y: inner.y.saturating_add(row as u16),
                        width: inner.width,
                        height: 1,
                    },
                    part_key: part_key.clone(),
                });
        }
    }

    if app.chat.focused_part_key().is_none()
        && let Some(first) = app.layout.chat.part_toggle_hitboxes.first()
    {
        app.chat.set_focused_part_key(Some(first.part_key.clone()));
    }

    frame.render_widget(
        Paragraph::new(
            visible
                .into_iter()
                .map(|entry| entry.line)
                .collect::<Vec<_>>(),
        )
        .wrap(Wrap { trim: false }),
        inner,
    );
}

fn role_palette(role: ChatMessageRole, app: &App) -> (Color, &'static str, Color) {
    match role {
        ChatMessageRole::System => (app.theme.pill_warn_fg, "SYSTEM", app.theme.pill_muted_bg),
        ChatMessageRole::User => (app.theme.pill_info_fg, "YOU", app.theme.pill_info_bg),
        ChatMessageRole::Assistant => (
            app.theme.pill_accent_fg,
            "AGENT",
            app.theme.selected_card_bg,
        ),
    }
}
