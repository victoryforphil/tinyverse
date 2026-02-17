use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

use crate::runtime::helpers::{inset_rect, key_hint, truncate_to};

pub(super) fn render_chat_composer(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Composer ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.chat.composing {
            app.theme.pane_focused_border
        } else {
            app.theme.pane_unfocused_border
        }));
    let inner = inset_rect(block.inner(area), 1, 0);
    frame.render_widget(block, area);
    if inner.width < 4 || inner.height < 2 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    let model_text = format!("model:{}", truncate_to(&app.chat.active_model, 28));
    let agent_text = format!("agent:{}", truncate_to(&app.chat.active_agent, 22));
    let bridge = app.chat_bridge.status();
    let source_text = format!("source:{}", bridge.mode.label());
    let model_width = model_text.chars().count() as u16 + 2;
    let agent_width = agent_text.chars().count() as u16 + 2;
    let model_rect = Rect {
        x: rows[0].x,
        y: rows[0].y,
        width: model_width.min(rows[0].width),
        height: 1,
    };
    let agent_x = model_rect
        .x
        .saturating_add(model_rect.width)
        .saturating_add(2);
    let agent_rect = Rect {
        x: agent_x,
        y: rows[0].y,
        width: agent_width.min(rows[0].right().saturating_sub(agent_x)),
        height: 1,
    };
    app.layout.chat.model_chip_rect = Some(model_rect);
    app.layout.chat.agent_chip_rect = Some(agent_rect);
    app.layout.chat.composer_input_rect = Some(rows[1]);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!(" {model_text} "),
                Style::default()
                    .fg(app.theme.pill_accent_fg)
                    .bg(app.theme.pill_accent_bg),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" {agent_text} "),
                Style::default()
                    .fg(app.theme.pill_info_fg)
                    .bg(app.theme.pill_info_bg),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" {source_text} "),
                Style::default()
                    .fg(app.theme.pill_warn_fg)
                    .bg(app.theme.pill_warn_bg),
            ),
        ])),
        rows[0],
    );

    if app.chat.composing {
        frame.render_widget(
            Paragraph::new(app.chat.draft_with_cursor())
                .style(Style::default().fg(app.theme.text_primary)),
            rows[1],
        );
    } else {
        let mut hint_spans = Vec::new();
        hint_spans.extend(key_hint("c", "compose", &app.theme));
        hint_spans.push(Span::raw("  "));
        hint_spans.extend(key_hint("enter", "send", &app.theme));
        hint_spans.push(Span::raw("  "));
        hint_spans.extend(key_hint("z", "toggle details", &app.theme));
        frame.render_widget(
            Paragraph::new(Line::from(hint_spans)).style(Style::default().fg(app.theme.text_muted)),
            rows[1],
        );
    }
}
