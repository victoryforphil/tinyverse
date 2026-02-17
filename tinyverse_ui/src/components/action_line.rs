use crate::render::RenderMode;
use crate::render::{pad_right, RenderContext};
use crate::theme::Tone;

pub struct ActionLine<'a> {
    pub label: &'a str,
    pub message: &'a str,
    pub tone: Tone,
}

impl<'a> ActionLine<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let badge_text = pad_right(self.label, 7);
        match context.mode {
            RenderMode::Plain => format!("[{badge_text}] {}", self.message),
            RenderMode::Ansi => {
                let badge = context.theme.tone_badge_style(self.tone).paint(badge_text);
                let message = context.theme.tone_text_style(self.tone).paint(self.message);
                format!("[{badge}] {message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::{RenderContext, RenderMode};
    use crate::theme::{DefaultTheme, Tone};

    use super::ActionLine;

    #[test]
    fn renders_action_line_in_plain_mode() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, None, &theme);
        let line = ActionLine {
            label: "OK",
            message: "Session started",
            tone: Tone::Success,
        };

        let rendered = line.render(&context);
        assert!(rendered.contains("Session started"));
        assert!(!rendered.contains("\u{1b}["));
    }
}
