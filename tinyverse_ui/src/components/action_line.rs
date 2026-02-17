use std::borrow::Cow;

use crate::render::RenderMode;
use crate::render::{pad_right, RenderContext};
use crate::theme::Tone;

pub struct ActionLine<'a> {
    pub label: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub tone: Tone,
}

impl<'a> ActionLine<'a> {
    pub fn new(
        label: impl Into<Cow<'a, str>>,
        message: impl Into<Cow<'a, str>>,
        tone: Tone,
    ) -> Self {
        Self {
            label: label.into(),
            message: message.into(),
            tone,
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let badge_text = pad_right(&self.label, 7);
        match context.mode {
            RenderMode::Plain => format!("[{badge_text}] {}", self.message),
            RenderMode::Ansi => {
                let badge = context.theme.tone_badge_style(self.tone).paint(badge_text);
                let message = context
                    .theme
                    .tone_text_style(self.tone)
                    .paint(self.message.as_ref());
                format!("[{badge}] {message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::render::{RenderContext, RenderMode};
    use crate::theme::{DefaultTheme, Tone};

    use super::ActionLine;

    #[test]
    fn renders_action_line_in_plain_mode() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, None, &theme);
        let line = ActionLine {
            label: Cow::Borrowed("OK"),
            message: Cow::Borrowed("Session started"),
            tone: Tone::Success,
        };

        let rendered = line.render(&context);
        assert!(rendered.contains("Session started"));
        assert!(!rendered.contains("\u{1b}["));
    }
}
