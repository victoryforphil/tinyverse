use nu_ansi_term::{Color, Style};

/// Semantic output tone used by badges and action lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Info,
    Success,
    Warning,
    Error,
    Muted,
}

/// Maps semantic presentation roles to concrete ANSI styles.
pub trait Theme {
    fn section_header_style(&self) -> Style;
    fn label_style(&self) -> Style;
    fn guidance_style(&self) -> Style;
    fn summary_style(&self) -> Style;
    fn table_header_style(&self) -> Style;
    fn table_stripe_style(&self) -> Style;
    fn dim_style(&self) -> Style;
    fn panel_border_style(&self) -> Style;
    fn panel_border_tone_style(&self, tone: Tone) -> Style;
    fn tone_badge_style(&self, tone: Tone) -> Style;
    fn tone_text_style(&self, tone: Tone) -> Style;
}

/// Default colorful tinyverse theme for terminal output.
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

    fn summary_style(&self) -> Style {
        Style::new().fg(Color::Cyan)
    }

    fn table_header_style(&self) -> Style {
        Style::new().bold()
    }

    fn table_stripe_style(&self) -> Style {
        Style::new().dimmed()
    }

    fn dim_style(&self) -> Style {
        Style::new().dimmed()
    }

    fn panel_border_style(&self) -> Style {
        Style::new().dimmed()
    }

    fn panel_border_tone_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Neutral => Style::new().dimmed(),
            Tone::Info => Style::new().fg(Color::Blue).bold(),
            Tone::Success => Style::new().fg(Color::Green).bold(),
            Tone::Warning => Style::new().fg(Color::Yellow).bold(),
            Tone::Error => Style::new().fg(Color::Red).bold(),
            Tone::Muted => Style::new().dimmed(),
        }
    }

    fn tone_badge_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Neutral => Style::new().on(Color::White).fg(Color::Black),
            Tone::Info => Style::new().on(Color::Blue).fg(Color::White).bold(),
            Tone::Success => Style::new().on(Color::Green).fg(Color::Black).bold(),
            Tone::Warning => Style::new().on(Color::Yellow).fg(Color::Black).bold(),
            Tone::Error => Style::new().on(Color::Red).fg(Color::White).bold(),
            Tone::Muted => Style::new().dimmed(),
        }
    }

    fn tone_text_style(&self, tone: Tone) -> Style {
        match tone {
            Tone::Neutral => Style::new().fg(Color::White),
            Tone::Info => Style::new().fg(Color::Blue),
            Tone::Success => Style::new().fg(Color::Green),
            Tone::Warning => Style::new().fg(Color::Yellow),
            Tone::Error => Style::new().fg(Color::Red).bold(),
            Tone::Muted => Style::new().dimmed(),
        }
    }
}

/// Minimal mostly monochrome theme for low-noise output.
#[derive(Debug, Default, Clone, Copy)]
pub struct MinimalTheme;

impl Theme for MinimalTheme {
    fn section_header_style(&self) -> Style {
        Style::new().bold()
    }

    fn label_style(&self) -> Style {
        Style::new().bold()
    }

    fn guidance_style(&self) -> Style {
        Style::new().bold()
    }

    fn summary_style(&self) -> Style {
        Style::new().bold()
    }

    fn table_header_style(&self) -> Style {
        Style::new().bold()
    }

    fn table_stripe_style(&self) -> Style {
        Style::new()
    }

    fn dim_style(&self) -> Style {
        Style::new()
    }

    fn panel_border_style(&self) -> Style {
        Style::new().bold()
    }

    fn panel_border_tone_style(&self, _tone: Tone) -> Style {
        Style::new().bold()
    }

    fn tone_badge_style(&self, _tone: Tone) -> Style {
        Style::new().bold()
    }

    fn tone_text_style(&self, _tone: Tone) -> Style {
        Style::new()
    }
}
