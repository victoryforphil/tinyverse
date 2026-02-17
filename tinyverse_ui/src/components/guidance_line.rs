use std::borrow::Cow;

use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct GuidanceLine<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> GuidanceLine<'a> {
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("Next: {}", self.message),
            RenderMode::Ansi => {
                let prefix = context.theme.guidance_style().paint("Next:");
                format!("{prefix} {}", self.message)
            }
        }
    }
}
