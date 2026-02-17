use ratatui::layout::Rect;

/// Returns whether a terminal cell lies inside the rectangle.
pub fn rect_contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x && col < area.x + area.width && row >= area.y && row < area.y + area.height
}

/// Returns an inner rect inset by one cell on each edge.
pub fn inner_rect(area: Rect) -> Rect {
    Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

/// Returns a centered rectangle clamped to the parent area.
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

/// Returns an anchored rectangle clamped to the provided bounds.
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

/// Insets a rectangle by horizontal and vertical margins.
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

/// Appends a block cursor marker to the end of text.
pub fn with_cursor_tail(text: &str) -> String {
    let trimmed = text.trim_end();
    if trimmed.is_empty() {
        "\u{2588}".to_string()
    } else {
        format!("{trimmed}\u{2588}")
    }
}

#[cfg(test)]
mod tests {
    use super::{anchored_rect, centered_rect, inset_rect};
    use ratatui::layout::Rect;

    #[test]
    fn centered_rect_stays_within_area() {
        let area = Rect::new(2, 2, 20, 10);
        let centered = centered_rect(12, 6, area);
        assert!(centered.x >= area.x);
        assert!(centered.y >= area.y);
        assert!(centered.right() <= area.right());
        assert!(centered.bottom() <= area.bottom());
    }

    #[test]
    fn anchored_rect_clamps_to_bounds() {
        let bounds = Rect::new(0, 0, 40, 12);
        let anchored = anchored_rect(20, 8, 99, 99, bounds);
        assert!(anchored.right() <= bounds.right());
        assert!(anchored.bottom() <= bounds.bottom());
    }

    #[test]
    fn inset_rect_shrinks_dimensions() {
        let area = Rect::new(4, 3, 30, 12);
        assert_eq!(inset_rect(area, 2, 1), Rect::new(6, 4, 26, 10));
    }
}
