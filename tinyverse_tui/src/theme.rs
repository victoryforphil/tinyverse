use std::path::PathBuf;

use tinyverse_lib::resolve_tinyverse_paths;

use ratatui::style::Color;

const THEME_FILE_NAME: &str = "theme.toml";

#[derive(Debug, Clone)]
pub struct UiTheme {
    pub pill_ok_fg: Color,
    pub pill_ok_bg: Color,
    pub pill_warn_fg: Color,
    pub pill_warn_bg: Color,
    pub pill_err_fg: Color,
    pub pill_err_bg: Color,
    pub pill_info_fg: Color,
    pub pill_info_bg: Color,
    pub pill_muted_fg: Color,
    pub pill_muted_bg: Color,
    pub pill_accent_fg: Color,
    pub pill_accent_bg: Color,
    pub key_hint_key_fg: Color,
    pub key_hint_key_bg: Color,
    pub key_hint_action_fg: Color,
    pub key_hint_bracket_fg: Color,
    pub pane_focused_border: Color,
    pub pane_unfocused_border: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_primary: Color,
    pub selected_card_bg: Color,

    // Chat message card colors
    pub chat_separator_fg: Color,
    pub chat_header_user_bg: Color,
    pub chat_header_agent_bg: Color,
    pub chat_header_system_bg: Color,
    pub chat_collapsible_bg: Color,
    pub chat_collapsible_focused_bg: Color,
    pub chat_collapsible_tag_bg: Color,
    pub chat_code_bg: Color,
    pub path_pill_fg: Color,
    pub path_pill_bg: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            pill_ok_fg: Color::Rgb(168, 214, 162),
            pill_ok_bg: Color::Rgb(28, 52, 29),
            pill_warn_fg: Color::Rgb(202, 176, 116),
            pill_warn_bg: Color::Rgb(58, 44, 20),
            pill_err_fg: Color::Rgb(235, 145, 132),
            pill_err_bg: Color::Rgb(68, 26, 24),
            pill_info_fg: Color::Rgb(178, 162, 196),
            pill_info_bg: Color::Rgb(44, 34, 50),
            pill_muted_fg: Color::Rgb(150, 148, 144),
            pill_muted_bg: Color::Rgb(40, 40, 38),
            pill_accent_fg: Color::Rgb(120, 192, 166),
            pill_accent_bg: Color::Rgb(24, 48, 42),
            key_hint_key_fg: Color::Rgb(222, 218, 210),
            key_hint_key_bg: Color::Rgb(60, 58, 56),
            key_hint_action_fg: Color::Rgb(154, 150, 142),
            key_hint_bracket_fg: Color::Rgb(96, 94, 90),
            pane_focused_border: Color::Rgb(120, 190, 160),
            pane_unfocused_border: Color::Rgb(68, 66, 62),
            text_secondary: Color::Rgb(182, 178, 172),
            text_muted: Color::Rgb(116, 112, 106),
            text_primary: Color::Rgb(232, 228, 220),
            selected_card_bg: Color::Rgb(28, 30, 30),

            // Chat message card colors
            chat_separator_fg: Color::Rgb(62, 60, 56),
            chat_header_user_bg: Color::Rgb(26, 34, 30),
            chat_header_agent_bg: Color::Rgb(34, 28, 38),
            chat_header_system_bg: Color::Rgb(40, 36, 26),
            chat_collapsible_bg: Color::Rgb(26, 26, 24),
            chat_collapsible_focused_bg: Color::Rgb(36, 34, 30),
            chat_collapsible_tag_bg: Color::Rgb(30, 30, 28),
            chat_code_bg: Color::Rgb(24, 24, 24),
            path_pill_fg: Color::Rgb(192, 184, 152),
            path_pill_bg: Color::Rgb(38, 36, 30),
        }
    }
}

pub fn load_theme() -> UiTheme {
    let mut theme = UiTheme::default();
    for path in candidate_paths() {
        if let Ok(raw) = std::fs::read_to_string(&path) {
            apply_theme_overrides(&raw, &mut theme);
        }
    }
    theme
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(resolved) = resolve_tinyverse_paths(None) {
        paths.push(resolved.home_dir.join(THEME_FILE_NAME));
    }
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join(THEME_FILE_NAME));
    }
    paths
}

fn apply_theme_overrides(raw: &str, theme: &mut UiTheme) {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let parsed = unquote(value.trim());
        let Some(color) = parse_color(&parsed) else {
            continue;
        };
        match key.trim() {
            "pill_ok_fg" => theme.pill_ok_fg = color,
            "pill_ok_bg" => theme.pill_ok_bg = color,
            "pill_warn_fg" => theme.pill_warn_fg = color,
            "pill_warn_bg" => theme.pill_warn_bg = color,
            "pill_err_fg" => theme.pill_err_fg = color,
            "pill_err_bg" => theme.pill_err_bg = color,
            "pill_info_fg" => theme.pill_info_fg = color,
            "pill_info_bg" => theme.pill_info_bg = color,
            "pill_muted_fg" => theme.pill_muted_fg = color,
            "pill_muted_bg" => theme.pill_muted_bg = color,
            "pill_accent_fg" => theme.pill_accent_fg = color,
            "pill_accent_bg" => theme.pill_accent_bg = color,
            "key_hint_key_fg" => theme.key_hint_key_fg = color,
            "key_hint_key_bg" => theme.key_hint_key_bg = color,
            "key_hint_action_fg" => theme.key_hint_action_fg = color,
            "key_hint_bracket_fg" => theme.key_hint_bracket_fg = color,
            "pane_focused_border" => theme.pane_focused_border = color,
            "pane_unfocused_border" => theme.pane_unfocused_border = color,
            "text_secondary" => theme.text_secondary = color,
            "text_muted" => theme.text_muted = color,
            "text_primary" => theme.text_primary = color,
            "selected_card_bg" => theme.selected_card_bg = color,
            "chat_separator_fg" => theme.chat_separator_fg = color,
            "chat_header_user_bg" => theme.chat_header_user_bg = color,
            "chat_header_agent_bg" => theme.chat_header_agent_bg = color,
            "chat_header_system_bg" => theme.chat_header_system_bg = color,
            "chat_collapsible_bg" => theme.chat_collapsible_bg = color,
            "chat_collapsible_focused_bg" => theme.chat_collapsible_focused_bg = color,
            "chat_collapsible_tag_bg" => theme.chat_collapsible_tag_bg = color,
            "chat_code_bg" => theme.chat_code_bg = color,
            "path_pill_fg" => theme.path_pill_fg = color,
            "path_pill_bg" => theme.path_pill_bg = color,
            _ => {}
        }
    }
}

fn unquote(value: &str) -> String {
    let unwrapped = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(value);
    unwrapped.replace("\\\"", "\"").replace("\\\\", "\\")
}

fn parse_color(raw: &str) -> Option<Color> {
    let value = raw.trim();
    if let Some(hex) = value.strip_prefix('#')
        && hex.len() == 6
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(Color::Rgb(r, g, b));
    }

    match value.to_ascii_lowercase().replace(['_', '-'], "").as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" | "grey" => Some(Color::Gray),
        "darkgray" | "darkgrey" => Some(Color::DarkGray),
        "lightred" => Some(Color::LightRed),
        "lightgreen" => Some(Color::LightGreen),
        "lightyellow" => Some(Color::LightYellow),
        "lightblue" => Some(Color::LightBlue),
        "lightmagenta" => Some(Color::LightMagenta),
        "lightcyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        _ => None,
    }
}
