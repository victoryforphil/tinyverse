use std::borrow::Cow;

use crate::components::GuidanceLine;
use crate::render::RenderContext;
use crate::theme::Tone;

use super::ActionLine;

pub struct ErrorBlock<'a> {
    pub title: Cow<'a, str>,
    pub detail: Option<Cow<'a, str>>,
    pub guidance: Option<Cow<'a, str>>,
}

impl<'a> ErrorBlock<'a> {
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            detail: None,
            guidance: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<Cow<'a, str>>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_guidance(mut self, guidance: impl Into<Cow<'a, str>>) -> Self {
        self.guidance = Some(guidance.into());
        self
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let mut lines = Vec::new();
        lines.push(
            ActionLine {
                label: Cow::Borrowed("ERROR"),
                message: self.title.clone(),
                tone: Tone::Error,
            }
            .render(context),
        );
        if let Some(detail) = &self.detail {
            lines.push(format!("  {detail}"));
        }
        if let Some(guidance) = &self.guidance {
            lines.push(
                GuidanceLine {
                    message: guidance.clone(),
                }
                .render(context),
            );
        }
        lines.join("\n")
    }
}
