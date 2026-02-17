use std::borrow::Cow;

use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct SectionHeader<'a> {
    pub title: Cow<'a, str>,
}

impl<'a> SectionHeader<'a> {
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let title = self.title.as_ref();
        let underline_len = title.chars().count() + 6; // "== " + title + " =="
        let underline = "\u{2500}".repeat(underline_len);
        match context.mode {
            RenderMode::Plain => format!("== {title} ==\n{}", "-".repeat(underline_len)),
            RenderMode::Ansi => {
                let header = context
                    .theme
                    .section_header_style()
                    .paint(format!("== {title} =="));
                let rule = context.theme.dim_style().paint(underline);
                format!("{header}\n{rule}")
            }
        }
    }
}
