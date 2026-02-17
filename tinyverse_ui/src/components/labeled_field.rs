use std::borrow::Cow;

use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct LabeledField<'a> {
    pub label: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> LabeledField<'a> {
    pub fn new(label: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("{}: {}", self.label, self.value),
            RenderMode::Ansi => {
                let label = context.theme.label_style().paint(self.label.as_ref());
                format!("{label}: {}", self.value)
            }
        }
    }
}
