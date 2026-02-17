use crate::render::RenderContext;
use crate::render::RenderMode;

pub struct LabeledField<'a> {
    pub label: &'a str,
    pub value: &'a str,
}

impl<'a> LabeledField<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        match context.mode {
            RenderMode::Plain => format!("{}: {}", self.label, self.value),
            RenderMode::Ansi => {
                let label = context.theme.label_style().paint(self.label);
                format!("{label}: {}", self.value)
            }
        }
    }
}
