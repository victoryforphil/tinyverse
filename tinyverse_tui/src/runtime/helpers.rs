use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::theme::UiTheme;

pub fn styled_panel(title: &str, focused: bool, theme: &UiTheme) -> Block<'static> {
    let border_color = if focused {
        theme.pane_focused_border
    } else {
        theme.pane_unfocused_border
    };
    Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title_style(
            Style::default()
                .fg(theme.text_primary)
                .add_modifier(Modifier::BOLD),
        )
}

pub fn status_pill(status: &str, theme: &UiTheme) -> Span<'static> {
    let lowered = status.to_ascii_lowercase();
    let (fg, bg) = match lowered.as_str() {
        "active" => (theme.pill_ok_fg, theme.pill_ok_bg),
        "stale" => (theme.pill_warn_fg, theme.pill_warn_bg),
        "dead" => (theme.pill_err_fg, theme.pill_err_bg),
        _ => (theme.pill_muted_fg, theme.pill_muted_bg),
    };
    Span::styled(
        format!(" {} ", lowered),
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
    )
}

pub fn tag_pill(label: &str, theme: &UiTheme) -> Span<'static> {
    Span::styled(
        format!(" {} ", label),
        Style::default()
            .fg(theme.pill_accent_fg)
            .bg(theme.pill_accent_bg),
    )
}

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let popup_width = width.min(area.width.saturating_sub(2));
    let popup_height = height.min(area.height.saturating_sub(2));

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    }
}

pub fn anchored_rect(width: u16, height: u16, x: u16, y: u16, bounds: Rect) -> Rect {
    let popup_width = width.min(bounds.width.saturating_sub(1));
    let popup_height = height.min(bounds.height.saturating_sub(1));

    let max_x = bounds.right().saturating_sub(popup_width);
    let max_y = bounds.bottom().saturating_sub(popup_height);

    let anchor_x = x.min(max_x);
    let anchor_y = if y + popup_height <= bounds.bottom() {
        y
    } else {
        y.saturating_sub(popup_height)
    }
    .min(max_y);

    Rect {
        x: anchor_x,
        y: anchor_y,
        width: popup_width,
        height: popup_height,
    }
}

pub fn inset_rect(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    let x = area.x.saturating_add(horizontal);
    let y = area.y.saturating_add(vertical);
    let width = area.width.saturating_sub(horizontal.saturating_mul(2));
    let height = area.height.saturating_sub(vertical.saturating_mul(2));
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn line_kv(label: &str, value: &str, theme: &UiTheme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{}: ", label),
            Style::default().fg(theme.text_muted),
        ),
        Span::styled(value.to_owned(), Style::default().fg(theme.text_secondary)),
    ])
}

pub fn key_hint(key: &str, action: &str, theme: &UiTheme) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            format!(" {key} "),
            Style::default()
                .fg(theme.key_hint_key_fg)
                .bg(theme.key_hint_key_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            action.to_owned(),
            Style::default().fg(theme.key_hint_action_fg),
        ),
    ]
}

pub fn truncate_to(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_owned();
    }

    let mut truncated = String::new();
    for ch in value.chars().take(max_len.saturating_sub(1)) {
        truncated.push(ch);
    }
    truncated.push('…');
    truncated
}

pub fn rect_contains(rect: Rect, x: u16, y: u16) -> bool {
    x >= rect.x && x < rect.right() && y >= rect.y && y < rect.bottom()
}
