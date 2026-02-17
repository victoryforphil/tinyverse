use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct SummaryFooter<'a> {
    pub message: &'a str,
}

impl<'a> SummaryFooter<'a> {
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
