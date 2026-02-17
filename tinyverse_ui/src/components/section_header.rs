use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct SectionHeader<'a> {
    pub title: &'a str,
}

impl<'a> SectionHeader<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("== {} ==", self.title),
            RenderMode::Ansi => context
                .theme
                .section_header_style()
                .paint(format!("== {} ==", self.title))
                .to_string(),
        }
    }
}
