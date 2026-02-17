use crate::render::RenderContext;

use super::{LabeledField, SectionHeader};

pub struct DetailSection<'a> {
    pub title: &'a str,
    pub fields: Vec<LabeledField<'a>>,
}

impl<'a> DetailSection<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let mut lines = Vec::new();
        lines.push(SectionHeader { title: self.title }.render(context));
        for field in &self.fields {
            lines.push(field.render(context));
        }
        lines.join("\n")
    }
}
