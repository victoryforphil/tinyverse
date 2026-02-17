use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

use crate::theme::ComponentThemeLike;
use crate::{centered_rect, inset_rect};

/// Render props for a centered modal overlay.
#[derive(Debug, Clone)]
pub struct ModalOverlayProps {
    pub title: String,
    pub header_lines: Vec<Line<'static>>,
    pub body_lines: Vec<Line<'static>>,
    pub hint_line: Option<Line<'static>>,
    pub width: u16,
    pub height: u16,
    pub scroll_lines: usize,
}

/// Layout metadata returned from modal rendering.
#[derive(Debug, Clone, Copy)]
pub struct ModalOverlayLayout {
    pub area: Rect,
    pub body_area: Rect,
}

/// Shared modal renderer with header, divider, scrollable body, and footer hint.
pub struct ModalOverlay;

impl ModalOverlay {
    /// Renders a centered modal and returns popup/body rectangles.
    pub fn render(
        frame: &mut Frame,
        parent: Rect,
        props: &ModalOverlayProps,
        theme: &impl ComponentThemeLike,
    ) -> Option<ModalOverlayLayout> {
        if parent.width < 6 || parent.height < 6 {
            return None;
        }

        let popup = centered_rect(props.width, props.height, parent);
        frame.render_widget(Clear, popup);

        let block = Block::default()
            .title(format!(" {} ", props.title))
            .title_style(
                Style::default()
                    .fg(theme.text_primary())
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(theme.surface_bg()))
            .border_style(Style::default().fg(theme.pane_focused_border()));
        let inner = inset_rect(block.inner(popup), 1, 0);
        frame.render_widget(block, popup);

        if inner.width == 0 || inner.height == 0 {
            return Some(ModalOverlayLayout {
                area: popup,
                body_area: inner,
            });
        }

        let header_height = props.header_lines.len().max(1).min(inner.height as usize) as u16;
        let hint_height = if props.hint_line.is_some() { 1 } else { 0 };
        let divider_height = 1;
        let max_reserved = header_height
            .saturating_add(hint_height)
            .saturating_add(divider_height);
        let body_height = inner.height.saturating_sub(max_reserved).max(1);

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(header_height),
                Constraint::Length(divider_height),
                Constraint::Length(body_height),
                Constraint::Length(hint_height),
            ])
            .split(inner);

        let header_area = sections[0];
        let divider_area = sections[1];
        let body_area = sections[2];
        let hint_area = sections.get(3).copied();

        let header_lines = if props.header_lines.is_empty() {
            vec![Line::from(Span::styled(
                "",
                Style::default().fg(theme.text_muted()),
            ))]
        } else {
            props.header_lines.clone()
        };
        frame.render_widget(Paragraph::new(header_lines), header_area);

        let rule_len = divider_area.width as usize;
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "─".repeat(rule_len),
                Style::default().fg(theme.chat_separator_fg()),
            ))),
            divider_area,
        );

        let max_lines = body_area.height as usize;
        let overflow = props.body_lines.len().saturating_sub(max_lines);
        let start = overflow.saturating_sub(props.scroll_lines);
        let visible = props
            .body_lines
            .iter()
            .skip(start)
            .take(max_lines)
            .cloned()
            .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(visible).wrap(Wrap { trim: false }),
            body_area,
        );

        if let (Some(hint_area), Some(hint_line)) = (hint_area, props.hint_line.clone())
            && hint_area.height > 0
        {
            frame.render_widget(Paragraph::new(hint_line), hint_area);
        }

        Some(ModalOverlayLayout {
            area: popup,
            body_area,
        })
    }
}
