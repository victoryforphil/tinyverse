use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::SectionHeader;
use crate::theme::ComponentThemeLike;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeViewMode {
    Normal,
    Compact,
}

#[derive(Debug, Clone)]
pub struct CodeViewLine {
    pub line_number: Option<usize>,
    pub marker: Option<char>,
    pub marker_style: Option<Style>,
    pub gutter_label: Option<String>,
    pub content: Line<'static>,
}

impl CodeViewLine {
    pub fn plain(line_number: usize, text: impl Into<String>) -> Self {
        Self {
            line_number: Some(line_number),
            marker: None,
            marker_style: None,
            gutter_label: None,
            content: Line::from(text.into()),
        }
    }

    pub fn with_style(line_number: usize, text: impl Into<String>, style: Style) -> Self {
        Self {
            line_number: Some(line_number),
            marker: None,
            marker_style: None,
            gutter_label: None,
            content: Line::from(Span::styled(text.into(), style)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeViewProps<'a> {
    pub lines: &'a [CodeViewLine],
    pub scroll: usize,
    pub horizontal_offset: usize,
    pub title: Option<&'a str>,
    pub show_line_numbers: bool,
    pub mode: CodeViewMode,
    pub empty_message: &'a str,
}

impl<'a> CodeViewProps<'a> {
    pub fn new(lines: &'a [CodeViewLine]) -> Self {
        Self {
            lines,
            scroll: 0,
            horizontal_offset: 0,
            title: None,
            show_line_numbers: true,
            mode: CodeViewMode::Normal,
            empty_message: "(no lines)",
        }
    }
}

pub struct CodeViewComponent;

impl CodeViewComponent {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        theme: &impl ComponentThemeLike,
        props: CodeViewProps<'_>,
    ) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let body_area = if let Some(title) = props.title {
            if area.height == 1 {
                area
            } else {
                let header =
                    SectionHeader::new(title, theme.pane_focused_border()).line(area.width, theme);
                let header_area = Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: 1,
                };
                frame.render_widget(Paragraph::new(header), header_area);
                Rect {
                    x: area.x,
                    y: area.y.saturating_add(1),
                    width: area.width,
                    height: area.height.saturating_sub(1),
                }
            }
        } else {
            area
        };

        if body_area.width == 0 || body_area.height == 0 {
            return;
        }

        if props.lines.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    props.empty_message.to_string(),
                    Style::default().fg(theme.text_muted()),
                ))),
                body_area,
            );
            return;
        }

        let gutter_width = gutter_width(
            props.lines,
            props.show_line_numbers,
            props.mode,
            body_area.width,
        );

        let [gutter_area, content_area] =
            Layout::horizontal([Constraint::Length(gutter_width), Constraint::Min(1)])
                .areas(body_area);

        let scroll = props
            .scroll
            .min(props.lines.len().saturating_sub(body_area.height as usize));
        let y_scroll = scroll.min(u16::MAX as usize) as u16;
        let x_scroll = props.horizontal_offset.min(u16::MAX as usize) as u16;

        let gutter_lines = props
            .lines
            .iter()
            .map(|line| render_gutter_line(line, props.show_line_numbers, props.mode, theme))
            .collect::<Vec<_>>();
        let content_lines = props
            .lines
            .iter()
            .map(|line| line.content.clone())
            .collect::<Vec<_>>();

        frame.render_widget(
            Paragraph::new(gutter_lines).scroll((y_scroll, 0)),
            gutter_area,
        );
        frame.render_widget(
            Paragraph::new(content_lines).scroll((y_scroll, x_scroll)),
            content_area,
        );
    }
}

fn render_gutter_line(
    line: &CodeViewLine,
    show_line_numbers: bool,
    mode: CodeViewMode,
    theme: &impl ComponentThemeLike,
) -> Line<'static> {
    let marker = line.marker.unwrap_or(' ');

    let label = if show_line_numbers {
        line.gutter_label
            .clone()
            .or_else(|| line.line_number.map(|num| num.to_string()))
            .unwrap_or_default()
    } else {
        String::new()
    };

    let number_style = Style::default().fg(theme.text_muted());
    let marker_style = line.marker_style.unwrap_or_else(|| {
        Style::default()
            .fg(theme.text_secondary())
            .add_modifier(Modifier::BOLD)
    });
    let sep_style = Style::default().fg(theme.text_muted());

    if matches!(mode, CodeViewMode::Compact) {
        let compact = if show_line_numbers {
            format!("{label:>3}")
        } else {
            "   ".to_string()
        };
        return Line::from(vec![
            Span::styled(compact, number_style),
            Span::styled(marker.to_string(), marker_style),
            Span::styled("| ".to_string(), sep_style),
        ]);
    }

    let padded = if show_line_numbers {
        format!("{label:>8}")
    } else {
        "        ".to_string()
    };

    Line::from(vec![
        Span::styled(padded, number_style),
        Span::styled(marker.to_string(), marker_style),
        Span::styled("| ".to_string(), sep_style),
    ])
}

fn gutter_width(
    lines: &[CodeViewLine],
    show_line_numbers: bool,
    mode: CodeViewMode,
    area_width: u16,
) -> u16 {
    if !show_line_numbers {
        return 3;
    }

    let max_label = lines
        .iter()
        .filter_map(|line| {
            line.gutter_label
                .as_ref()
                .map(|label| label.chars().count())
                .or_else(|| {
                    line.line_number
                        .map(|number| number.to_string().chars().count())
                })
        })
        .max()
        .unwrap_or(1);

    let desired = match mode {
        CodeViewMode::Compact => 5,
        CodeViewMode::Normal => (max_label + 3).max(7) as u16,
    };
    desired.min(area_width.saturating_sub(1).max(3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_line_sets_number_and_content() {
        let line = CodeViewLine::plain(7, "hello");
        assert_eq!(line.line_number, Some(7));
        assert!(line.gutter_label.is_none());
        assert_eq!(line.content.spans[0].content.as_ref(), "hello");
    }

    #[test]
    fn gutter_width_uses_labels() {
        let lines = vec![CodeViewLine {
            line_number: None,
            marker: Some('@'),
            marker_style: None,
            gutter_label: Some("100  200".to_string()),
            content: Line::from("x"),
        }];

        assert_eq!(gutter_width(&lines, true, CodeViewMode::Normal, 80), 11);
    }
}
