use ratatui::style::Color;

pub mod loader;
mod palette;

pub use palette::ComponentTheme;

/// Semantic color tone used by shared components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Ok,
    Warn,
    Error,
    Info,
    Muted,
    Accent,
}

/// Theme contract consumed by shared components.
pub trait ComponentThemeLike {
    /// Base background color for full-frame fills.
    fn base_bg(&self) -> Color;
    /// Surface background color for pane/popup containers.
    fn surface_bg(&self) -> Color;
    /// Foreground color for success pills.
    fn pill_ok_fg(&self) -> Color;
    /// Background color for success pills.
    fn pill_ok_bg(&self) -> Color;
    /// Foreground color for warning pills.
    fn pill_warn_fg(&self) -> Color;
    /// Background color for warning pills.
    fn pill_warn_bg(&self) -> Color;
    /// Foreground color for error pills.
    fn pill_err_fg(&self) -> Color;
    /// Background color for error pills.
    fn pill_err_bg(&self) -> Color;
    /// Foreground color for info pills.
    fn pill_info_fg(&self) -> Color;
    /// Background color for info pills.
    fn pill_info_bg(&self) -> Color;
    /// Foreground color for muted pills.
    fn pill_muted_fg(&self) -> Color;
    /// Background color for muted pills.
    fn pill_muted_bg(&self) -> Color;
    /// Foreground color for accent pills.
    fn pill_accent_fg(&self) -> Color;
    /// Background color for accent pills.
    fn pill_accent_bg(&self) -> Color;
    /// Foreground color for key labels in key hint bars.
    fn key_hint_key_fg(&self) -> Color;
    /// Background color for key labels in key hint bars.
    fn key_hint_key_bg(&self) -> Color;
    /// Foreground color for action labels in key hint bars.
    fn key_hint_action_fg(&self) -> Color;
    /// Foreground color for separators in key hint bars.
    fn key_hint_bracket_fg(&self) -> Color;
    /// Border color for focused pane blocks.
    fn pane_focused_border(&self) -> Color;
    /// Border color for unfocused pane blocks.
    fn pane_unfocused_border(&self) -> Color;
    /// Secondary text color.
    fn text_secondary(&self) -> Color;
    /// Muted text color.
    fn text_muted(&self) -> Color;
    /// Primary text color.
    fn text_primary(&self) -> Color;
    /// Selection background color.
    fn selected_card_bg(&self) -> Color;
    /// Chat separator color.
    fn chat_separator_fg(&self) -> Color;
    /// Chat message card border color.
    fn chat_card_border_fg(&self) -> Color;
    /// Chat user header background color.
    fn chat_header_user_bg(&self) -> Color;
    /// Chat assistant header background color.
    fn chat_header_agent_bg(&self) -> Color;
    /// Chat system header background color.
    fn chat_header_system_bg(&self) -> Color;
    /// Chat collapsible row background color.
    fn chat_collapsible_bg(&self) -> Color;
    /// Chat collapsible focused row background color.
    fn chat_collapsible_focused_bg(&self) -> Color;
    /// Chat collapsible tag background color.
    fn chat_collapsible_tag_bg(&self) -> Color;
    /// Chat code block background color.
    fn chat_code_bg(&self) -> Color;
    /// Path badge foreground color.
    fn path_pill_fg(&self) -> Color;
    /// Path badge background color.
    fn path_pill_bg(&self) -> Color;
    /// Session-root row selection tint in tree mode.
    fn tree_tint_session(&self) -> Color;
    /// Console-pane row selection tint in tree mode.
    fn tree_tint_console(&self) -> Color;
    /// Agent-pane row selection tint in tree mode.
    fn tree_tint_agent(&self) -> Color;
    /// Chat-pane row selection tint in tree mode.
    fn tree_tint_chat(&self) -> Color;
    /// Chat-thread row selection tint in tree mode.
    fn tree_tint_thread(&self) -> Color;
    /// Tree badge foreground color.
    fn tree_badge_fg(&self) -> Color;
    /// Tree badge background color.
    fn tree_badge_bg(&self) -> Color;
}

impl ComponentThemeLike for ComponentTheme {
    fn base_bg(&self) -> Color {
        self.base_bg
    }
    fn surface_bg(&self) -> Color {
        self.surface_bg
    }
    fn pill_ok_fg(&self) -> Color {
        self.pill_ok_fg
    }
    fn pill_ok_bg(&self) -> Color {
        self.pill_ok_bg
    }
    fn pill_warn_fg(&self) -> Color {
        self.pill_warn_fg
    }
    fn pill_warn_bg(&self) -> Color {
        self.pill_warn_bg
    }
    fn pill_err_fg(&self) -> Color {
        self.pill_err_fg
    }
    fn pill_err_bg(&self) -> Color {
        self.pill_err_bg
    }
    fn pill_info_fg(&self) -> Color {
        self.pill_info_fg
    }
    fn pill_info_bg(&self) -> Color {
        self.pill_info_bg
    }
    fn pill_muted_fg(&self) -> Color {
        self.pill_muted_fg
    }
    fn pill_muted_bg(&self) -> Color {
        self.pill_muted_bg
    }
    fn pill_accent_fg(&self) -> Color {
        self.pill_accent_fg
    }
    fn pill_accent_bg(&self) -> Color {
        self.pill_accent_bg
    }
    fn key_hint_key_fg(&self) -> Color {
        self.key_hint_key_fg
    }
    fn key_hint_key_bg(&self) -> Color {
        self.key_hint_key_bg
    }
    fn key_hint_action_fg(&self) -> Color {
        self.key_hint_action_fg
    }
    fn key_hint_bracket_fg(&self) -> Color {
        self.key_hint_bracket_fg
    }
    fn pane_focused_border(&self) -> Color {
        self.pane_focused_border
    }
    fn pane_unfocused_border(&self) -> Color {
        self.pane_unfocused_border
    }
    fn text_secondary(&self) -> Color {
        self.text_secondary
    }
    fn text_muted(&self) -> Color {
        self.text_muted
    }
    fn text_primary(&self) -> Color {
        self.text_primary
    }
    fn selected_card_bg(&self) -> Color {
        self.selected_card_bg
    }
    fn chat_separator_fg(&self) -> Color {
        self.chat_separator_fg
    }
    fn chat_card_border_fg(&self) -> Color {
        self.chat_card_border_fg
    }
    fn chat_header_user_bg(&self) -> Color {
        self.chat_header_user_bg
    }
    fn chat_header_agent_bg(&self) -> Color {
        self.chat_header_agent_bg
    }
    fn chat_header_system_bg(&self) -> Color {
        self.chat_header_system_bg
    }
    fn chat_collapsible_bg(&self) -> Color {
        self.chat_collapsible_bg
    }
    fn chat_collapsible_focused_bg(&self) -> Color {
        self.chat_collapsible_focused_bg
    }
    fn chat_collapsible_tag_bg(&self) -> Color {
        self.chat_collapsible_tag_bg
    }
    fn chat_code_bg(&self) -> Color {
        self.chat_code_bg
    }
    fn path_pill_fg(&self) -> Color {
        self.path_pill_fg
    }
    fn path_pill_bg(&self) -> Color {
        self.path_pill_bg
    }
    fn tree_tint_session(&self) -> Color {
        self.tree_tint_session
    }
    fn tree_tint_console(&self) -> Color {
        self.tree_tint_console
    }
    fn tree_tint_agent(&self) -> Color {
        self.tree_tint_agent
    }
    fn tree_tint_chat(&self) -> Color {
        self.tree_tint_chat
    }
    fn tree_tint_thread(&self) -> Color {
        self.tree_tint_thread
    }
    fn tree_badge_fg(&self) -> Color {
        self.tree_badge_fg
    }
    fn tree_badge_bg(&self) -> Color {
        self.tree_badge_bg
    }
}
