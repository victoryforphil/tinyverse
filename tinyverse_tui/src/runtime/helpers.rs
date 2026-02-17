use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders};

pub const ACCENT_PRIMARY: Color = Color::Cyan;
pub const ACCENT_SECONDARY: Color = Color::Blue;
pub const TEXT_DIM: Color = Color::DarkGray;
pub const TEXT_MUTED: Color = Color::Gray;
pub const TEXT_NORMAL: Color = Color::White;

pub fn styled_panel(title: &str, accent: Color, focused: bool) -> Block<'static> {
    let border_color = if focused { accent } else { Color::DarkGray };
    Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title_style(
            Style::default()
                .fg(TEXT_NORMAL)
                .add_modifier(Modifier::BOLD),
        )
}

pub fn status_pill(status: &str) -> Span<'static> {
    let lowered = status.to_ascii_lowercase();
    let (fg, bg) = match lowered.as_str() {
        "active" => (Color::Black, Color::Green),
        "stale" => (Color::Black, Color::Yellow),
        "dead" => (Color::White, Color::Red),
        _ => (Color::White, Color::DarkGray),
    };
    Span::styled(
        format!(" {} ", lowered),
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
    )
}

pub fn tag_pill(label: &str) -> Span<'static> {
    Span::styled(
        format!(" {} ", label),
        Style::default().fg(Color::White).bg(Color::Blue),
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

pub fn line_kv(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}: ", label), Style::default().fg(TEXT_DIM)),
        Span::raw(value.to_owned()),
    ])
}

pub fn key_hint(key: &str, action: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(format!("[{key}]"), Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(action.to_owned(), Style::default().fg(TEXT_MUTED)),
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
