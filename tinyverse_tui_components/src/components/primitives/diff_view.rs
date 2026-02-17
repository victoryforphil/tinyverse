use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::code_view::{CodeViewComponent, CodeViewLine, CodeViewMode, CodeViewProps};
use crate::theme::ComponentThemeLike;
use ratatui::{Frame, layout::Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
    Modified,
    Header,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineNumberMode {
    Old,
    New,
    Both,
}

#[derive(Debug, Clone)]
pub struct DiffViewProps<'a> {
    pub lines: &'a [DiffLine],
    pub scroll: usize,
    pub horizontal_offset: usize,
    pub title: Option<&'a str>,
    pub mode: CodeViewMode,
    pub line_number_mode: DiffLineNumberMode,
    pub empty_message: &'a str,
}

impl<'a> DiffViewProps<'a> {
    pub fn new(lines: &'a [DiffLine]) -> Self {
        Self {
            lines,
            scroll: 0,
            horizontal_offset: 0,
            title: None,
            mode: CodeViewMode::Normal,
            line_number_mode: DiffLineNumberMode::Both,
            empty_message: "(no diff lines)",
        }
    }
}

pub struct DiffViewComponent;

impl DiffViewComponent {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: DiffViewProps<'_>,
    ) {
        let owned = props
            .lines
            .iter()
            .map(|line| to_code_line(line, theme, props.line_number_mode, props.mode))
            .collect::<Vec<_>>();

        CodeViewComponent::render(
            frame,
            area,
            theme,
            CodeViewProps {
                lines: &owned,
                scroll: props.scroll,
                horizontal_offset: props.horizontal_offset,
                title: props.title,
                show_line_numbers: true,
                mode: props.mode,
                empty_message: props.empty_message,
            },
        );
    }

    pub fn parse_unified(diff: &str) -> Vec<DiffLine> {
        let mut parsed = Vec::new();
        let mut old_cursor = 0usize;
        let mut new_cursor = 0usize;

        for raw_line in diff.lines() {
            if raw_line.starts_with("@@") {
                if let Some((old_start, new_start)) = parse_hunk_header(raw_line) {
                    old_cursor = old_start;
                    new_cursor = new_start;
                }
                parsed.push(DiffLine {
                    kind: DiffLineKind::Header,
                    old_line: None,
                    new_line: None,
                    text: raw_line.to_string(),
                });
                continue;
            }

            if raw_line.starts_with("+++")
                || raw_line.starts_with("---")
                || raw_line.starts_with("diff ")
            {
                parsed.push(DiffLine {
                    kind: DiffLineKind::Header,
                    old_line: None,
                    new_line: None,
                    text: raw_line.to_string(),
                });
                continue;
            }

            let mut chars = raw_line.chars();
            let prefix = chars.next().unwrap_or(' ');
            let text = chars.collect::<String>();

            match prefix {
                '+' => {
                    let line = DiffLine {
                        kind: DiffLineKind::Added,
                        old_line: None,
                        new_line: Some(new_cursor),
                        text,
                    };
                    new_cursor = new_cursor.saturating_add(1);
                    parsed.push(line);
                }
                '-' => {
                    let line = DiffLine {
                        kind: DiffLineKind::Removed,
                        old_line: Some(old_cursor),
                        new_line: None,
                        text,
                    };
                    old_cursor = old_cursor.saturating_add(1);
                    parsed.push(line);
                }
                ' ' => {
                    let line = DiffLine {
                        kind: DiffLineKind::Context,
                        old_line: Some(old_cursor),
                        new_line: Some(new_cursor),
                        text,
                    };
                    old_cursor = old_cursor.saturating_add(1);
                    new_cursor = new_cursor.saturating_add(1);
                    parsed.push(line);
                }
                '~' => {
                    parsed.push(DiffLine {
                        kind: DiffLineKind::Modified,
                        old_line: Some(old_cursor),
                        new_line: Some(new_cursor),
                        text,
                    });
                    old_cursor = old_cursor.saturating_add(1);
                    new_cursor = new_cursor.saturating_add(1);
                }
                _ => parsed.push(DiffLine {
                    kind: DiffLineKind::Context,
                    old_line: None,
                    new_line: None,
                    text: raw_line.to_string(),
                }),
            }
        }

        parsed
    }
}

fn to_code_line(
    line: &DiffLine,
    theme: &impl ComponentThemeLike,
    number_mode: DiffLineNumberMode,
    mode: CodeViewMode,
) -> CodeViewLine {
    let (fg, marker, bold) = match line.kind {
        DiffLineKind::Context => (theme.text_secondary(), ' ', false),
        DiffLineKind::Added => (Color::LightGreen, '+', false),
        DiffLineKind::Removed => (Color::LightRed, '-', false),
        DiffLineKind::Modified => (theme.pill_warn_fg(), '~', false),
        DiffLineKind::Header => (theme.pill_info_fg(), '@', true),
    };

    let mut style = Style::default().fg(fg);
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }

    let gutter_label = match number_mode {
        DiffLineNumberMode::Old => line.old_line.map(|num| num.to_string()),
        DiffLineNumberMode::New => line.new_line.map(|num| num.to_string()),
        DiffLineNumberMode::Both => {
            if matches!(mode, CodeViewMode::Compact) {
                line.new_line.or(line.old_line).map(|num| num.to_string())
            } else {
                Some(format!(
                    "{:>4} {:>4}",
                    line.old_line.map_or(String::new(), |num| num.to_string()),
                    line.new_line.map_or(String::new(), |num| num.to_string())
                ))
            }
        }
    };

    CodeViewLine {
        line_number: None,
        marker: Some(marker),
        marker_style: Some(style.add_modifier(Modifier::BOLD)),
        gutter_label,
        content: Line::from(Span::styled(line.text.clone(), style)),
    }
}

fn parse_hunk_header(header: &str) -> Option<(usize, usize)> {
    let old_idx = header.find('-')?;
    let old_part = &header[old_idx + 1..];
    let old_start = old_part.split([',', ' ']).next()?.parse::<usize>().ok()?;

    let new_idx = header.find('+')?;
    let new_part = &header[new_idx + 1..];
    let new_start = new_part.split([',', ' ']).next()?.parse::<usize>().ok()?;

    Some((old_start, new_start))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_unified_hunk() {
        let diff = "@@ -2,2 +2,3 @@\n line\n-old\n+new\n";
        let lines = DiffViewComponent::parse_unified(diff);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0].kind, DiffLineKind::Header);
        assert_eq!(lines[1].old_line, Some(2));
        assert_eq!(lines[1].new_line, Some(2));
        assert_eq!(lines[2].old_line, Some(3));
        assert_eq!(lines[3].new_line, Some(3));
    }

    #[test]
    fn parses_hunk_line_numbers() {
        assert_eq!(parse_hunk_header("@@ -12,8 +99,20 @@"), Some((12, 99)));
    }
}
