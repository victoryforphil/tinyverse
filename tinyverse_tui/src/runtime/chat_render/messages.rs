use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use tinyverse_tui_components::{compact_text, inset_rect};

use crate::app::{App, CachedChatLine, ChatPartToggleHitbox, ChatRenderCache, ChatRenderSignature};
use crate::chat::ChatMessageRole;

use super::parts::render_chat_part_lines;
use super::types::RenderedChatLine;

pub(super) fn render_chat_messages(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = inset_rect(area, 1, 0);
    app.layout.chat.part_toggle_hitboxes.clear();

    if inner.width == 0 || inner.height == 0 {
        frame.render_widget(Paragraph::new(Vec::<Line<'static>>::new()), inner);
        return;
    }

    let max_lines = inner.height as usize;
    let signature = render_signature(app);
    let should_rebuild = app
        .chat_render_cache
        .as_ref()
        .map(|cache| cache.width != inner.width || cache.signature != signature)
        .unwrap_or(true);

    if should_rebuild {
        let lines = build_chat_lines(app, inner);
        app.chat_render_cache = Some(ChatRenderCache {
            width: inner.width,
            signature: signature.clone(),
            lines,
        });
    }

    let Some(cache) = app.chat_render_cache.as_ref() else {
        return;
    };

    let total_lines = cache.lines.len();
    let overflow = total_lines.saturating_sub(max_lines);
    let scroll = app.chat.scroll_lines as usize;
    let start = overflow.saturating_sub(scroll);

    let mut visible_window = Vec::with_capacity(max_lines.min(total_lines.saturating_sub(start)));
    for (line, toggle_key) in cache.lines.iter().skip(start).take(max_lines) {
        visible_window.push((line.clone(), toggle_key.clone()));
    }

    let mut visible_lines: Vec<Line<'static>> = Vec::with_capacity(visible_window.len());
    for (row, (line, toggle_key)) in visible_window.into_iter().enumerate() {
        if let Some(part_key) = toggle_key {
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
                    part_key,
                });
        }
        visible_lines.push(line);
    }

    if app.chat.focused_part_key().is_none()
        && let Some(first) = app.layout.chat.part_toggle_hitboxes.first()
    {
        app.chat.set_focused_part_key(Some(first.part_key.clone()));
    }

    frame.render_widget(
        Paragraph::new(visible_lines).wrap(Wrap { trim: false }),
        inner,
    );
}

fn render_signature(app: &App) -> ChatRenderSignature {
    let last_message = app.chat.messages.last();
    ChatRenderSignature {
        message_count: app.chat.messages.len(),
        last_message_id: last_message.and_then(|message| message.id.clone()),
        last_message_created_at: last_message.map(|message| message.created_at.clone()),
        last_part_count: last_message.map(|message| message.parts.len()).unwrap_or(0),
        collapse_verbose_parts: app.chat.collapse_verbose_parts,
        focused_part_key: app.chat.focused_part_key().map(ToOwned::to_owned),
    }
}

fn build_chat_lines(app: &App, inner: Rect) -> Vec<CachedChatLine> {
    let max_lines = inner.height as usize;
    let estimated_capacity = app
        .chat
        .messages
        .len()
        .saturating_mul(4)
        .min(max_lines.saturating_mul(2))
        .max(16);
    let mut lines: Vec<RenderedChatLine> = Vec::with_capacity(estimated_capacity);
    let message_count = app.chat.messages.len();
    let compact_mode = inner.width < 20;
    let w = inner.width as usize;
    let content_width = w.saturating_sub(4).max(1);
    let border_fg = app.theme.chat_card_border_fg;
    let border_style = Style::default().fg(border_fg);
    let text_secondary_style = Style::default().fg(app.theme.text_secondary);
    let text_muted_style = Style::default().fg(app.theme.text_muted);
    let separator_style = Style::default().fg(app.theme.chat_separator_fg);
    let separator_str: String = "╌".repeat(content_width);

    for (message_index, message) in app.chat.messages.iter().enumerate() {
        let (role_fg, role_label, header_bg) = role_palette(message.role, app);
        let top_rule = if message_index == 0 { '╭' } else { '├' };
        let top_right = if message_index == 0 { '╮' } else { '┤' };
        let is_last_message = message_index + 1 == message_count;

        if !compact_mode {
            lines.push(RenderedChatLine {
                line: boxed_rule(top_rule, top_right, w, border_fg),
                toggle_key: None,
            });
        }

        let pill_text = format!(" {role_label} ");
        let ts_text = if message.created_at.trim().is_empty() {
            String::new()
        } else {
            format!(" {}", message.created_at)
        };
        let used = pill_text.chars().count() + ts_text.chars().count();
        let pad = " ".repeat(content_width.saturating_sub(used));

        if compact_mode {
            lines.push(RenderedChatLine {
                line: Line::from(vec![
                    Span::styled(
                        format!("[{role_label}]"),
                        Style::default().fg(role_fg).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(ts_text, text_muted_style),
                ]),
                toggle_key: None,
            });
        } else {
            lines.push(RenderedChatLine {
                line: Line::from(vec![
                    Span::styled("│ ", border_style),
                    Span::styled(
                        pill_text,
                        Style::default()
                            .fg(role_fg)
                            .bg(header_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(ts_text, text_muted_style.bg(header_bg)),
                    Span::styled(pad, Style::default().bg(header_bg)),
                    Span::styled(" │", border_style),
                ]),
                toggle_key: None,
            });
        }

        if message.parts.is_empty() {
            let mut had_text = false;
            for text_line in message.text.lines() {
                had_text = true;
                let content_line = Line::from(Span::styled(
                    compact_text(text_line, content_width),
                    text_secondary_style,
                ));
                lines.push(RenderedChatLine {
                    line: if compact_mode {
                        content_line
                    } else {
                        boxed_content_line(content_line, content_width, border_fg)
                    },
                    toggle_key: None,
                });
            }

            if !had_text {
                let content_line = Line::from(Span::styled("(no content)", text_muted_style));
                lines.push(RenderedChatLine {
                    line: if compact_mode {
                        content_line
                    } else {
                        boxed_content_line(content_line, content_width, border_fg)
                    },
                    toggle_key: None,
                });
            }
        } else {
            let part_count = message.parts.len();
            for (part_index, part) in message.parts.iter().enumerate() {
                if !compact_mode && part_index > 0 && part.is_collapsible() && part_count > 3 {
                    lines.push(RenderedChatLine {
                        line: boxed_content_line(
                            Line::from(Span::styled(separator_str.clone(), separator_style)),
                            content_width,
                            border_fg,
                        ),
                        toggle_key: None,
                    });
                }
                let part_key = app.chat.part_key(message, message_index, part_index);
                let rendered_parts =
                    render_chat_part_lines(part, app, content_width as u16, Some(part_key));
                for rendered in rendered_parts {
                    lines.push(RenderedChatLine {
                        line: if compact_mode {
                            rendered.line
                        } else {
                            boxed_content_line(rendered.line, content_width, border_fg)
                        },
                        toggle_key: rendered.toggle_key,
                    });
                }
            }
        }

        if !compact_mode && is_last_message {
            lines.push(RenderedChatLine {
                line: boxed_rule('╰', '╯', w, border_fg),
                toggle_key: None,
            });
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

    lines
        .into_iter()
        .map(|rendered| (rendered.line, rendered.toggle_key))
        .collect()
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

fn boxed_rule(left: char, right: char, width: usize, color: Color) -> Line<'static> {
    let style = Style::default().fg(color);
    let middle = "─".repeat(width.saturating_sub(2));
    Line::from(vec![
        Span::styled(left.to_string(), style),
        Span::styled(middle, style),
        Span::styled(right.to_string(), style),
    ])
}

fn boxed_content_line(
    mut content: Line<'static>,
    content_width: usize,
    border_fg: Color,
) -> Line<'static> {
    let border_style = Style::default().fg(border_fg);
    let span_count = content.spans.len();
    let mut spans = Vec::with_capacity(span_count + 3);
    spans.push(Span::styled("│ ", border_style));

    let mut used = 0usize;
    for span in content.spans.drain(..) {
        // For ASCII-only content, len() == char count and is faster.
        used += if span.content.is_ascii() {
            span.content.len()
        } else {
            span.content.chars().count()
        };
        spans.push(span);
    }

    if used < content_width {
        spans.push(Span::raw(" ".repeat(content_width - used)));
    }

    spans.push(Span::styled(" │", border_style));
    Line::from(spans)
}
