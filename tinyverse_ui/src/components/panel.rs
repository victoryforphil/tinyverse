use std::borrow::Cow;

use crate::render::{RenderContext, RenderMode, truncate_with_ellipsis, visible_width};
use crate::theme::Tone;

pub struct Panel<'a> {
    pub title: Option<Cow<'a, str>>,
    pub tone: Option<Tone>,
    pub body: Cow<'a, str>,
    pub padding: PanelPadding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanelPadding {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl Default for PanelPadding {
    fn default() -> Self {
        Self {
            top: 0,
            bottom: 0,
            left: 1,
            right: 1,
        }
    }
}

impl<'a> Panel<'a> {
    pub fn new(body: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: None,
            tone: None,
            body: body.into(),
            padding: PanelPadding::default(),
        }
    }

    pub fn with_title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    pub fn with_padding(mut self, padding: PanelPadding) -> Self {
        self.padding = padding;
        self
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let body_lines: Vec<&str> = self.body.lines().collect();
        let max_body_width = body_lines
            .iter()
            .map(|line| visible_width(line))
            .max()
            .unwrap_or(0);

        let title_width = self
            .title
            .as_ref()
            .map(|title| title.chars().count() + 3)
            .unwrap_or(0);

        let mut content_width = max_body_width.max(title_width);
        if let Some(total_width) = context.width {
            let reserved = self.padding.left + self.padding.right + 2;
            let max_content = total_width.saturating_sub(reserved);
            if max_content > 0 {
                content_width = content_width.min(max_content);
            }
        }

        let width = content_width.max(1);
        let (tl, tr, bl, br, h, v) = match context.mode {
            RenderMode::Ansi => ('╭', '╮', '╰', '╯', '─', '│'),
            RenderMode::Plain => ('+', '+', '+', '+', '-', '|'),
        };

        let border_style = self
            .tone
            .map(|tone| context.theme.panel_border_tone_style(tone))
            .unwrap_or_else(|| context.theme.panel_border_style());

        let horizontal_inner_width = width + self.padding.left + self.padding.right;
        let top_inner = render_top_inner(self.title.as_deref(), horizontal_inner_width, h);
        let top = styled_border(format!("{tl}{top_inner}{tr}"), context, border_style);

        let mut lines = vec![top];

        for _ in 0..self.padding.top {
            lines.push(styled_empty_line(
                horizontal_inner_width,
                v,
                context,
                border_style,
            ));
        }

        if body_lines.is_empty() {
            lines.push(render_body_line(
                "",
                width,
                self.padding,
                v,
                context,
                border_style,
            ));
        } else {
            for line in body_lines {
                lines.push(render_body_line(
                    line,
                    width,
                    self.padding,
                    v,
                    context,
                    border_style,
                ));
            }
        }

        for _ in 0..self.padding.bottom {
            lines.push(styled_empty_line(
                horizontal_inner_width,
                v,
                context,
                border_style,
            ));
        }

        let bottom = styled_border(
            format!("{bl}{}{br}", h.to_string().repeat(horizontal_inner_width)),
            context,
            border_style,
        );
        lines.push(bottom);

        lines.join("\n")
    }
}

fn render_top_inner(title: Option<&str>, width: usize, h: char) -> String {
    if let Some(title) = title {
        let max_title = width.saturating_sub(3);
        let title_text = truncate_with_ellipsis(title, max_title);
        let prefix = format!("{h} {title_text} ");
        let fill = width.saturating_sub(prefix.chars().count());
        return format!("{prefix}{}", h.to_string().repeat(fill));
    }

    h.to_string().repeat(width)
}

fn render_body_line(
    line: &str,
    width: usize,
    padding: PanelPadding,
    v: char,
    context: &RenderContext<'_>,
    border_style: nu_ansi_term::Style,
) -> String {
    let border = styled_border(v.to_string(), context, border_style);
    let visible = visible_width(line);
    let fill = width.saturating_sub(visible);
    let left = " ".repeat(padding.left);
    let right = " ".repeat(padding.right + fill);
    format!("{border}{left}{line}{right}{border}")
}

fn styled_empty_line(
    width: usize,
    v: char,
    context: &RenderContext<'_>,
    border_style: nu_ansi_term::Style,
) -> String {
    let border = styled_border(v.to_string(), context, border_style);
    format!("{border}{}{border}", " ".repeat(width))
}

fn styled_border(value: String, context: &RenderContext<'_>, style: nu_ansi_term::Style) -> String {
    match context.mode {
        RenderMode::Plain => value,
        RenderMode::Ansi => style.paint(value).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::render::{RenderContext, RenderMode};
    use crate::theme::DefaultTheme;

    use super::Panel;

    #[test]
    fn renders_panel_in_plain_mode() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, Some(40), &theme);
        let output = Panel::new("hello\nworld")
            .with_title("Result")
            .render(&context);
        assert!(output.contains("+- Result"));
        assert!(output.contains("| hello"));
        assert!(output.contains("| world"));
    }

    #[test]
    fn renders_panel_with_tone_in_ansi_mode() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Ansi, Some(40), &theme);
        let output = Panel::new("hello")
            .with_tone(crate::theme::Tone::Error)
            .render(&context);
        assert!(output.contains("\u{1b}["));
        assert!(output.contains("hello"));
    }
}
