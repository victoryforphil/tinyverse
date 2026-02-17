use std::borrow::Cow;

use crate::render::RenderContext;

use super::{LabeledField, SectionHeader};

pub struct DetailSection<'a> {
    pub title: Cow<'a, str>,
    pub fields: Vec<LabeledField<'a>>,
}

impl<'a> DetailSection<'a> {
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, field: LabeledField<'a>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        let mut lines = Vec::new();
        lines.push(
            SectionHeader {
                title: self.title.clone(),
            }
            .render(context),
        );
        for field in &self.fields {
            lines.push(field.render(context));
        }
        lines.join("\n")
    }
}
