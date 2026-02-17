use std::env;
use std::io::{self, IsTerminal};

use crate::theme::{DefaultTheme, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Plain,
    Ansi,
}

pub struct RenderContext<'a> {
    pub mode: RenderMode,
    pub width: Option<usize>,
    pub theme: &'a dyn Theme,
}

impl<'a> RenderContext<'a> {
    pub fn new(mode: RenderMode, width: Option<usize>, theme: &'a dyn Theme) -> Self {
        Self { mode, width, theme }
    }

    pub fn for_stdout(theme: &'a dyn Theme) -> Self {
        Self {
            mode: detect_stdout_render_mode(),
            width: terminal_width(),
            theme,
        }
    }
}

pub fn default_stdout_context() -> RenderContext<'static> {
    static THEME: DefaultTheme = DefaultTheme;
    RenderContext::for_stdout(&THEME)
}

pub fn detect_stdout_render_mode() -> RenderMode {
    if io::stdout().is_terminal() {
        RenderMode::Ansi
    } else {
        RenderMode::Plain
    }
}

pub fn terminal_width() -> Option<usize> {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
}

pub fn truncate_with_ellipsis(value: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    let chars = value.chars().count();
    if chars <= max_width {
        return value.to_owned();
    }
    if max_width <= 3 {
        return ".".repeat(max_width);
    }

    let keep = max_width - 3;
    let mut truncated = String::with_capacity(max_width);
    truncated.extend(value.chars().take(keep));
    truncated.push_str("...");
    truncated
}

pub fn pad_right(value: &str, width: usize) -> String {
    format!("{value:<width$}")
}

pub fn visible_width(value: &str) -> usize {
    let mut chars = value.chars().peekable();
    let mut width = 0usize;

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            let _ = chars.next();
            for esc in chars.by_ref() {
                if esc.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }

        width += 1;
    }

    width
}

#[cfg(test)]
mod tests {
    use super::{pad_right, truncate_with_ellipsis, visible_width};

    #[test]
    fn truncate_with_ellipsis_shortens_text() {
        let output = truncate_with_ellipsis("tinyverse_session_123", 10);
        assert_eq!(output, "tinyver...");
    }

    #[test]
    fn pad_right_adds_spaces() {
        let output = pad_right("id", 4);
        assert_eq!(output, "id  ");
    }

    #[test]
    fn visible_width_ignores_ansi_sequences() {
        let output = visible_width("\u{1b}[31mERROR\u{1b}[0m details");
        assert_eq!(output, 13);
    }
}
