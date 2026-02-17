use std::borrow::Cow;

use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct SummaryFooter<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> SummaryFooter<'a> {
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("Summary: {}", self.message),
            RenderMode::Ansi => context
                .theme
                .guidance_style()
                .paint(format!("Summary: {}", self.message))
                .to_string(),
        }
    }
}
