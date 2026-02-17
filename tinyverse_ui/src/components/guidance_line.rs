use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct GuidanceLine<'a> {
    pub message: &'a str,
}

impl<'a> GuidanceLine<'a> {
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
