use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::theme::ComponentThemeLike;
use crate::{StatusPill, compact_text};

// ── Tree connector glyphs (Unicode box-drawing) ───────────────────
const GLYPH_ROOT: &str = "● ";
const GLYPH_BRANCH: &str = "├─ ";
const GLYPH_LAST_BRANCH: &str = "└─ ";
const GLYPH_PIPE: &str = "│  ";
const GLYPH_SPACE: &str = "   ";

// ── Selection caret ───────────────────────────────────────────────
const CARET_SELECTED: &str = "▸ ";
const CARET_NORMAL: &str = "  ";

/// Budget reserved for suffix elements (status pills, meta text,
/// spacing, caret) so label truncation is depth-aware.
const SUFFIX_BUDGET: usize = 28;

/// Minimum characters the label column keeps even in narrow terminals.
const LABEL_MIN_WIDTH: usize = 8;

/// A small colored tag rendered inline in a tree row.
///
/// Badges appear between the status pill and meta text.  They are
/// intentionally generic so any tree consumer can attach contextual
/// metadata (thread counts, role markers, etc.) without coupling
/// the shared component to domain types.
#[derive(Debug, Clone)]
pub struct FileTreeBadge {
    pub label: String,
    pub fg: Color,
    pub bg: Color,
}

/// One display row in a generic file-style tree list.
#[derive(Debug, Clone)]
pub struct FileTreeRow {
    pub label: String,
    pub depth: usize,
    pub is_last: bool,
    pub ancestors_are_last: Vec<bool>,
    pub icon: Option<String>,
    pub is_active: bool,
    pub status: Option<String>,
    pub meta: Option<String>,
    /// Optional per-row background tint applied when the row is
    /// selected.  Falls back to `theme.selected_card_bg()` when
    /// `None`.
    pub selected_bg: Option<Color>,
    /// Extra inline badges rendered after the status pill.
    pub badges: Vec<FileTreeBadge>,
}

/// Render options for the tree list.
#[derive(Debug, Clone)]
pub struct FileTreeProps<'a> {
    pub rows: &'a [FileTreeRow],
    pub selected: usize,
    pub scroll: usize,
    pub empty_message: &'a str,
}

/// Row hitbox for click mapping.
#[derive(Debug, Clone, Copy)]
pub struct FileTreeRowHitbox {
    pub row_index: usize,
    pub rect: Rect,
}

/// Render metadata returned from tree drawing.
#[derive(Debug, Clone, Default)]
pub struct FileTreeLayout {
    pub row_hitboxes: Vec<FileTreeRowHitbox>,
    pub body_rect: Option<Rect>,
    pub scroll: usize,
}

/// Shared file-tree renderer with dark-factory style connectors.
pub struct FileTreeComponent;

impl FileTreeComponent {
    /// Renders rows and returns hitboxes/scroll state.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        props: FileTreeProps<'_>,
        theme: &impl ComponentThemeLike,
    ) -> FileTreeLayout {
        let mut layout = FileTreeLayout {
            body_rect: Some(area),
            ..FileTreeLayout::default()
        };

        if area.width == 0 || area.height == 0 {
            return layout;
        }

        if props.rows.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    props.empty_message.to_owned(),
                    Style::default().fg(theme.text_secondary()),
                ))),
                area,
            );
            return layout;
        }

        let show_hint = props.rows.len() > area.height as usize;
        let body_height = if show_hint {
            area.height.saturating_sub(1).max(1)
        } else {
            area.height
        };
        let visible_count = body_height as usize;
        let max_window_start = props.rows.len().saturating_sub(visible_count);

        let selected = props.selected.min(props.rows.len().saturating_sub(1));
        let mut scroll = props.scroll.min(max_window_start);
        if selected < scroll {
            scroll = selected;
        }
        if selected >= scroll + visible_count {
            scroll = selected.saturating_add(1).saturating_sub(visible_count);
        }
        scroll = scroll.min(max_window_start);
        layout.scroll = scroll;

        let window_end = (scroll + visible_count).min(props.rows.len());
        let mut lines = Vec::with_capacity(visible_count);

        for (offset, row) in props.rows[scroll..window_end].iter().enumerate() {
            let row_index = scroll + offset;
            let is_selected = row_index == selected;
            let row_area = Rect {
                x: area.x,
                y: area.y + offset as u16,
                width: area.width,
                height: 1,
            };
            layout.row_hitboxes.push(FileTreeRowHitbox {
                row_index,
                rect: row_area,
            });
            lines.push(render_row(row, is_selected, area.width as usize, theme));
        }

        while lines.len() < visible_count {
            lines.push(Line::from(""));
        }

        let body_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: body_height,
        };
        layout.body_rect = Some(body_area);
        frame.render_widget(Paragraph::new(lines), body_area);

        if show_hint {
            let hint = Paragraph::new(format!(
                "showing {}-{} of {}",
                scroll + 1,
                window_end,
                props.rows.len()
            ))
            .style(
                Style::default()
                    .fg(theme.text_muted())
                    .add_modifier(Modifier::DIM),
            );
            let hint_area = Rect {
                x: area.x,
                y: area.y.saturating_add(area.height.saturating_sub(1)),
                width: area.width,
                height: 1,
            };
            frame.render_widget(hint, hint_area);
        }

        layout
    }
}

fn render_row(
    row: &FileTreeRow,
    is_selected: bool,
    width: usize,
    theme: &impl ComponentThemeLike,
) -> Line<'static> {
    let row_bg = is_selected.then(|| row.selected_bg.unwrap_or_else(|| theme.selected_card_bg()));
    let mut spans = Vec::with_capacity(14);
    let mut used: usize = 0;

    // ── Selection caret ────────────────────────────────────────────
    let caret = if is_selected {
        CARET_SELECTED
    } else {
        CARET_NORMAL
    };
    let caret_fg = if is_selected {
        theme.text_primary()
    } else {
        theme.text_muted()
    };
    used += caret.chars().count();
    spans.push(span_fg_bg(caret.to_owned(), caret_fg, row_bg));

    // ── Prefix: tree connectors ────────────────────────────────────
    let prefix = tree_prefix(row.depth, row.is_last, &row.ancestors_are_last);
    used += prefix.chars().count();
    spans.push(span_fg_bg_with_modifier(
        prefix,
        theme.text_muted(),
        row_bg,
        Modifier::DIM,
    ));

    // ── Icon (optional, with trailing space) ───────────────────────
    if let Some(icon) = row.icon.as_ref() {
        used += icon.chars().count() + 1;
        spans.push(span_fg_bg(icon.clone(), theme.text_secondary(), row_bg));
        spans.push(span_fg_bg(" ".to_owned(), theme.text_secondary(), row_bg));
    }

    // ── Label ──────────────────────────────────────────────────────
    let label_max = width
        .saturating_sub(used + SUFFIX_BUDGET)
        .max(LABEL_MIN_WIDTH);
    let label_style = if row.is_active {
        Style::default()
            .fg(theme.text_primary())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.text_secondary())
    };
    let label_text = compact_text(&row.label, label_max);
    used += label_text.chars().count();
    spans.push(Span::styled(label_text, apply_bg(label_style, row_bg)));

    // ── Suffix: pills and meta ─────────────────────────────────────
    used += push_suffix_spans(&mut spans, row, row_bg, theme);

    // ── Selection highlight fill ───────────────────────────────────
    if is_selected && used < width {
        spans.push(span_fg_bg(
            " ".repeat(width - used),
            theme.text_muted(),
            row_bg,
        ));
    }

    Line::from(spans)
}

/// Appends status/active pills, badges, and meta text, returning the
/// total character width consumed by those additions.
fn push_suffix_spans(
    spans: &mut Vec<Span<'static>>,
    row: &FileTreeRow,
    row_bg: Option<Color>,
    theme: &impl ComponentThemeLike,
) -> usize {
    let mut used = 0;

    if row.is_active {
        spans.push(span_fg_bg("  ".to_owned(), theme.text_muted(), row_bg));
        spans.push(StatusPill::accent("active", theme).span_compact());
        used += 2 + "active".len();
    }

    if let Some(status) = row.status.as_deref() {
        spans.push(span_fg_bg_with_modifier(
            " · ".to_owned(),
            theme.text_muted(),
            row_bg,
            Modifier::DIM,
        ));
        spans.push(status_pill_for(status, theme).span_compact());
        used += 3 + status.len();
    }

    for badge in &row.badges {
        spans.push(span_fg_bg(" ".to_owned(), theme.text_muted(), row_bg));
        spans.push(Span::styled(
            badge.label.clone(),
            Style::default().fg(badge.fg).bg(badge.bg),
        ));
        used += 1 + badge.label.chars().count();
    }

    if let Some(meta) = row.meta.as_ref() {
        spans.push(span_fg_bg_with_modifier(
            " · ".to_owned(),
            theme.text_muted(),
            row_bg,
            Modifier::DIM,
        ));
        spans.push(span_fg_bg_with_modifier(
            meta.clone(),
            theme.text_muted(),
            row_bg,
            Modifier::DIM,
        ));
        used += 3 + meta.chars().count();
    }

    used
}

// ── Span helpers ───────────────────────────────────────────────────

/// Creates a styled span with foreground and optional background.
fn span_fg_bg(value: String, fg: Color, bg: Option<Color>) -> Span<'static> {
    let mut style = Style::default().fg(fg);
    if let Some(bg) = bg {
        style = style.bg(bg);
    }
    Span::styled(value, style)
}

fn span_fg_bg_with_modifier(
    value: String,
    fg: Color,
    bg: Option<Color>,
    modifier: Modifier,
) -> Span<'static> {
    let mut style = Style::default().fg(fg).add_modifier(modifier);
    if let Some(bg) = bg {
        style = style.bg(bg);
    }
    Span::styled(value, style)
}

/// Applies an optional background to an existing style.
fn apply_bg(mut style: Style, bg: Option<Color>) -> Style {
    if let Some(bg) = bg {
        style = style.bg(bg);
    }
    style
}

/// Maps a status string to the corresponding pill tone.
fn status_pill_for(status: &str, theme: &impl ComponentThemeLike) -> StatusPill {
    match status.trim().to_ascii_lowercase().as_str() {
        "idle" | "ready" | "active" => StatusPill::ok(status, theme),
        "busy" | "running" => StatusPill::info(status, theme),
        "retry" | "retrying" | "stale" => StatusPill::warn(status, theme),
        "error" | "failed" | "dead" => StatusPill::error(status, theme),
        _ => StatusPill::muted(status, theme),
    }
}

/// Builds a Unicode box-drawing tree connector prefix.
pub fn tree_prefix(depth: usize, is_last: bool, ancestors_are_last: &[bool]) -> String {
    let mut prefix = String::with_capacity(depth * 3 + 3);

    for ancestor_is_last in ancestors_are_last.iter().take(depth.saturating_sub(1)) {
        prefix.push_str(if *ancestor_is_last {
            GLYPH_SPACE
        } else {
            GLYPH_PIPE
        });
    }

    if depth == 0 {
        prefix.push_str(GLYPH_ROOT);
    } else if is_last {
        prefix.push_str(GLYPH_LAST_BRANCH);
    } else {
        prefix.push_str(GLYPH_BRANCH);
    }

    prefix
}
