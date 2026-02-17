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
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            pill_ok_fg: Color::Rgb(180, 230, 180),
            pill_ok_bg: Color::Rgb(30, 60, 30),
            pill_warn_fg: Color::Rgb(230, 210, 140),
            pill_warn_bg: Color::Rgb(60, 50, 20),
            pill_err_fg: Color::Rgb(240, 160, 150),
            pill_err_bg: Color::Rgb(70, 25, 25),
            pill_info_fg: Color::Rgb(150, 190, 230),
            pill_info_bg: Color::Rgb(25, 40, 65),
            pill_muted_fg: Color::Rgb(140, 140, 140),
            pill_muted_bg: Color::Rgb(40, 40, 40),
            pill_accent_fg: Color::Rgb(160, 220, 230),
            pill_accent_bg: Color::Rgb(25, 55, 60),
            key_hint_key_fg: Color::Rgb(220, 220, 220),
            key_hint_key_bg: Color::Rgb(55, 55, 70),
            key_hint_action_fg: Color::Rgb(150, 150, 160),
            key_hint_bracket_fg: Color::Rgb(90, 90, 110),
            pane_focused_border: Color::Cyan,
            pane_unfocused_border: Color::DarkGray,
            text_secondary: Color::Gray,
            text_muted: Color::DarkGray,
            text_primary: Color::White,
            selected_card_bg: Color::Rgb(12, 28, 32),
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
