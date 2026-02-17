use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::theme::ComponentThemeLike;

/// Builds consistent bordered pane containers.
pub struct PaneBlockComponent;

impl PaneBlockComponent {
    /// Creates a bordered block with focus-aware border styling.
    pub fn build<'a>(title: &'a str, focused: bool, theme: &impl ComponentThemeLike) -> Block<'a> {
        Self::build_with_bg(title, focused, theme, Some(theme.surface_bg()))
    }

    /// Creates a bordered block without filling the interior background.
    pub fn build_transparent<'a>(
        title: &'a str,
        focused: bool,
        theme: &impl ComponentThemeLike,
    ) -> Block<'a> {
        Self::build_with_bg(title, focused, theme, None)
    }

    fn build_with_bg<'a>(
        title: &'a str,
        focused: bool,
        theme: &impl ComponentThemeLike,
        background: Option<ratatui::style::Color>,
    ) -> Block<'a> {
        let border_style = if focused {
            Style::default()
                .fg(theme.pane_focused_border())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.pane_unfocused_border())
        };

        let mut block = Block::default()
            .title(format!(" {title} "))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title_style(
                Style::default()
                    .fg(theme.text_primary())
                    .add_modifier(Modifier::BOLD),
            );

        if let Some(bg) = background {
            block = block.style(Style::default().bg(bg));
        }

        block
    }
}
