use std::borrow::Cow;

use crate::render::RenderContext;
use crate::render::RenderMode;
use crate::theme::Tone;

pub struct StatusBadge<'a> {
    pub label: Cow<'a, str>,
    pub tone: Tone,
}

impl<'a> StatusBadge<'a> {
    pub fn new(label: impl Into<Cow<'a, str>>, tone: Tone) -> Self {
        Self {
            label: label.into(),
            tone,
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("[{}]", self.label),
            RenderMode::Ansi => context
                .theme
                .tone_badge_style(self.tone)
                .paint(format!(" {} ", self.label))
                .to_string(),
        }
    }
}
