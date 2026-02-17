use crate::components::GuidanceLine;
use crate::render::RenderContext;
use crate::theme::Tone;

use super::ActionLine;

pub struct ErrorBlock<'a> {
    pub title: &'a str,
    pub detail: Option<&'a str>,
    pub guidance: Option<&'a str>,
}

impl<'a> ErrorBlock<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let mut lines = Vec::new();
        lines.push(
            ActionLine {
                label: "ERROR",
                message: self.title,
                tone: Tone::Error,
            }
            .render(context),
        );
        if let Some(detail) = self.detail {
            lines.push(format!("  {detail}"));
        }
        if let Some(guidance) = self.guidance {
            lines.push(GuidanceLine { message: guidance }.render(context));
        }
        lines.join("\n")
    }
}
