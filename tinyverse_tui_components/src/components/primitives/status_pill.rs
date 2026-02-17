use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::theme::{ComponentThemeLike, Tone};

/// Compact colored badge used for status labels.
pub struct StatusPill {
    label: String,
    fg: Color,
    bg: Color,
    bold: bool,
}

impl StatusPill {
    /// Creates a pill with explicit foreground/background colors.
    pub fn new(label: impl Into<String>, fg: Color, bg: Color) -> Self {
        Self {
            label: label.into(),
            fg,
            bg,
            bold: false,
        }
    }

    /// Creates a pill with explicit colors and optional bold text.
    pub fn custom(label: impl Into<String>, fg: Color, bg: Color, bold: bool) -> Self {
        Self {
            label: label.into(),
            fg,
            bg,
            bold,
        }
    }

    /// Creates a pill by semantic tone.
    pub fn for_tone(label: impl Into<String>, tone: Tone, theme: &impl ComponentThemeLike) -> Self {
        match tone {
            Tone::Ok => Self::ok(label, theme),
            Tone::Warn => Self::warn(label, theme),
            Tone::Error => Self::error(label, theme),
            Tone::Info => Self::info(label, theme),
            Tone::Muted => Self::muted(label, theme),
            Tone::Accent => Self::accent(label, theme),
        }
    }

    /// Creates a success-status pill.
    pub fn ok(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_ok_fg(), theme.pill_ok_bg())
    }

    /// Creates a warning-status pill.
    pub fn warn(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_warn_fg(), theme.pill_warn_bg())
    }

    /// Creates an error-status pill.
    pub fn error(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_err_fg(), theme.pill_err_bg())
    }

    /// Creates an informational pill.
    pub fn info(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_info_fg(), theme.pill_info_bg())
    }

    /// Creates a muted-status pill.
    pub fn muted(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_muted_fg(), theme.pill_muted_bg())
    }

    /// Creates an accent pill.
    pub fn accent(label: impl Into<String>, theme: &impl ComponentThemeLike) -> Self {
        Self::new(label, theme.pill_accent_fg(), theme.pill_accent_bg())
    }

    /// Renders the pill as a padded span.
    pub fn span(&self) -> Span<'static> {
        let mut style = Style::default().fg(self.fg).bg(self.bg);
        if self.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        Span::styled(format!(" {} ", self.label), style)
    }

    /// Renders the pill as an unpadded compact span.
    pub fn span_compact(&self) -> Span<'static> {
        let mut style = Style::default().fg(self.fg).bg(self.bg);
        if self.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        Span::styled(self.label.clone(), style)
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier};

    use super::StatusPill;
    use crate::theme::{ComponentTheme, Tone};

    #[test]
    fn span_has_padded_label() {
        let theme = ComponentTheme::default();
        let pill = StatusPill::ok("clean", &theme);
        assert_eq!(pill.span().content, " clean ");
        assert_eq!(pill.span_compact().content, "clean");
    }

    #[test]
    fn custom_bold_pill_sets_style_modifier() {
        let pill = StatusPill::custom("x", Color::White, Color::Black, true);
        let span = pill.span();
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn for_tone_maps_to_expected_palette() {
        let theme = ComponentTheme::default();
        let pill = StatusPill::for_tone("ready", Tone::Accent, &theme).span();
        assert_eq!(pill.style.fg, Some(theme.pill_accent_fg));
        assert_eq!(pill.style.bg, Some(theme.pill_accent_bg));
    }
}
