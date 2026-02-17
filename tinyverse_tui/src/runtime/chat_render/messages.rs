use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
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

    let msg_count = app.chat.messages.len();
    for (message_index, message) in app.chat.messages.iter().enumerate() {
        if message_index > 0 {
            lines.push(RenderedChatLine {
                line: Line::from(""),
                toggle_key: None,
            });
        }

        let (role_fg, role_label, header_bg) = role_palette(message.role, app);
        let w = inner.width as usize;

        // ── Role-aware card header ──────────────────────────────────
        // Full-width tinted background line with role pill + timestamp
        let pill_text = format!(" {role_label} ");
        let ts_text = format!("  {}", message.created_at);
        let used = 2 + pill_text.chars().count() + ts_text.chars().count();
        let pad = " ".repeat(w.saturating_sub(used).max(0));

        lines.push(RenderedChatLine {
            line: Line::from(vec![
                Span::styled("│ ", Style::default().fg(role_fg).bg(header_bg)),
                Span::styled(pill_text, Style::default().fg(role_fg).bg(header_bg)),
                Span::styled(
                    ts_text,
                    Style::default().fg(app.theme.text_muted).bg(header_bg),
                ),
                Span::styled(pad, Style::default().bg(header_bg)),
            ]),
            toggle_key: None,
        });

        if message.parts.is_empty() {
            for text_line in message.text.lines() {
                lines.push(RenderedChatLine {
                    line: Line::from(Span::styled(
                        format!("  {text_line}"),
                        Style::default().fg(app.theme.text_secondary),
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
                ));
            }
        }

        // Subtle role-aware separator only when role changes.
        let is_last = message_index + 1 == msg_count;
        if !is_last {
            let next_role = app.chat.messages.get(message_index + 1).map(|m| m.role);
            if next_role != Some(message.role) {
                let rule_len = w.saturating_sub(8).min(20).max(10);
                let sep_fg = separator_tint(role_fg, app.theme.chat_separator_fg);
                lines.push(RenderedChatLine {
                    line: Line::from(vec![
                        Span::styled("  ", Style::default().fg(sep_fg)),
                        Span::styled("┈".repeat(rule_len), Style::default().fg(sep_fg)),
                    ]),
                    toggle_key: None,
                });
            }
        }
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
        ChatMessageRole::System => (
            app.theme.pill_warn_fg,
            "SYSTEM",
            app.theme.chat_header_system_bg,
        ),
        ChatMessageRole::User => (
            app.theme.pill_accent_fg,
            "YOU",
            app.theme.chat_header_user_bg,
        ),
        ChatMessageRole::Assistant => (
            app.theme.pill_info_fg,
            "AGENT",
            app.theme.chat_header_agent_bg,
        ),
    }
}

/// Blends role accent towards the base separator color for a subtle tint.
fn separator_tint(role_fg: Color, base: Color) -> Color {
    let (rb, gb, bb) = match base {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (50, 50, 58),
    };
    let (rr, gr, br) = match role_fg {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (rb, gb, bb),
    };
    // 80% base, 20% role accent
    Color::Rgb(
        ((rb as u16 * 8 + rr as u16 * 2) / 10) as u8,
        ((gb as u16 * 8 + gr as u16 * 2) / 10) as u8,
        ((bb as u16 * 8 + br as u16 * 2) / 10) as u8,
    )
}
