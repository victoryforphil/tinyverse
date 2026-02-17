use nu_ansi_term::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}

pub trait Theme {
    fn section_header_style(&self) -> Style;
    fn label_style(&self) -> Style;
    fn guidance_style(&self) -> Style;
    fn tone_badge_style(&self, tone: Tone) -> Style;
    fn tone_text_style(&self, tone: Tone) -> Style;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTheme;

impl Theme for DefaultTheme {
    fn section_header_style(&self) -> Style {
        Style::new().fg(Color::Blue).bold()
    }

    fn label_style(&self) -> Style {
        Style::new().fg(Color::Cyan).bold()
    }

    fn guidance_style(&self) -> Style {
        Style::new().fg(Color::Green)
    }

    fn tone_badge_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Neutral => Style::new().on(Color::White).fg(Color::Black),
            Tone::Info => Style::new().on(Color::Blue).fg(Color::White).bold(),
            Tone::Success => Style::new().on(Color::Green).fg(Color::Black).bold(),
            Tone::Warning => Style::new().on(Color::Yellow).fg(Color::Black).bold(),
            Tone::Error => Style::new().on(Color::Red).fg(Color::White).bold(),
        }
    }

    fn tone_text_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Neutral => Style::new().fg(Color::White),
            Tone::Info => Style::new().fg(Color::Blue),
            Tone::Success => Style::new().fg(Color::Green),
            Tone::Warning => Style::new().fg(Color::Yellow),
            Tone::Error => Style::new().fg(Color::Red).bold(),
        }
    }
}
