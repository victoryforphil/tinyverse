use std::fs;
use std::path::{Path, PathBuf};

use ratatui::style::Color;

use super::ComponentTheme;

/// Loads theme overrides from one or more candidate files.
pub fn load_theme_from_paths<I, P>(paths: I) -> ComponentTheme
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut theme = ComponentTheme::default();
    for path in paths {
        if let Ok(raw) = fs::read_to_string(path.as_ref()) {
            apply_theme_overrides(&raw, &mut theme);
        }
    }
    theme
}

/// Resolves a theme selector (name or path) into candidate file paths.
///
/// When the selector looks like a path (contains `/` or ends with `.toml`),
/// it is treated as an explicit path and returned directly.
///
/// Otherwise it is treated as a short name and expanded to
/// `<name>.theme.toml` and looked up in each of the provided search
/// directories and their `themes/` subdirectories.
///
/// If `selector` is `None`, falls back to the plain `theme.toml` filename
/// in each search directory (and `themes/` subdirectory).
pub fn resolve_theme_paths(selector: Option<&str>, search_dirs: &[PathBuf]) -> Vec<PathBuf> {
    match selector {
        Some(sel) if looks_like_path(sel) => {
            vec![PathBuf::from(sel)]
        }
        Some(name) => {
            let file_name = format!("{name}.theme.toml");
            let mut paths = Vec::new();
            // First load base theme.toml in each dir (home then cwd, etc).
            // Then apply named theme overrides so selected theme wins.
            for dir in search_dirs {
                paths.push(dir.join("theme.toml"));
            }
            for dir in search_dirs {
                paths.push(dir.join("themes").join("theme.toml"));
            }
            for dir in search_dirs {
                paths.push(dir.join(&file_name));
            }
            for dir in search_dirs {
                paths.push(dir.join("themes").join(&file_name));
            }
            paths
        }
        None => {
            let mut paths = Vec::new();
            for dir in search_dirs {
                paths.push(dir.join("theme.toml"));
            }
            for dir in search_dirs {
                paths.push(dir.join("themes").join("theme.toml"));
            }
            paths
        }
    }
}

/// Returns `true` if the selector string looks like a file path rather than
/// a short theme name.
fn looks_like_path(sel: &str) -> bool {
    sel.contains('/') || sel.contains('\\') || sel.ends_with(".toml")
}

/// Applies key-value theme overrides from a TOML-like source.
pub fn apply_theme_overrides(raw: &str, theme: &mut ComponentTheme) {
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
            "base_bg" => theme.base_bg = color,
            "surface_bg" => theme.surface_bg = color,
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
            "chat_card_border_fg" => theme.chat_card_border_fg = color,
            "chat_header_user_bg" => theme.chat_header_user_bg = color,
            "chat_header_agent_bg" => theme.chat_header_agent_bg = color,
            "chat_header_system_bg" => theme.chat_header_system_bg = color,
            "chat_collapsible_bg" => theme.chat_collapsible_bg = color,
            "chat_collapsible_focused_bg" => theme.chat_collapsible_focused_bg = color,
            "chat_collapsible_tag_bg" => theme.chat_collapsible_tag_bg = color,
            "chat_code_bg" => theme.chat_code_bg = color,
            "path_pill_fg" => theme.path_pill_fg = color,
            "path_pill_bg" => theme.path_pill_bg = color,
            "tree_tint_session" => theme.tree_tint_session = color,
            "tree_tint_console" => theme.tree_tint_console = color,
            "tree_tint_agent" => theme.tree_tint_agent = color,
            "tree_tint_chat" => theme.tree_tint_chat = color,
            "tree_tint_thread" => theme.tree_tint_thread = color,
            "tree_badge_fg" => theme.tree_badge_fg = color,
            "tree_badge_bg" => theme.tree_badge_bg = color,
            _ => {}
        }
    }
}

/// Parses a color value from hex or named variants.
pub fn parse_color(raw: &str) -> Option<Color> {
    let value = raw.trim();

    if let Ok(index) = value.parse::<u8>() {
        return Some(Color::Indexed(index));
    }

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

fn unquote(value: &str) -> String {
    let unwrapped = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(value);
    unwrapped.replace("\\\"", "\"").replace("\\\\", "\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_none_returns_theme_toml_in_each_dir() {
        let dirs = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        let paths = resolve_theme_paths(None, &dirs);
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/a/theme.toml"),
                PathBuf::from("/b/theme.toml"),
                PathBuf::from("/a/themes/theme.toml"),
                PathBuf::from("/b/themes/theme.toml"),
            ]
        );
    }

    #[test]
    fn resolve_short_name_returns_base_then_named_overrides() {
        let dirs = vec![PathBuf::from("/home"), PathBuf::from("/cwd")];
        let paths = resolve_theme_paths(Some("vfp"), &dirs);
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/home/theme.toml"),
                PathBuf::from("/cwd/theme.toml"),
                PathBuf::from("/home/themes/theme.toml"),
                PathBuf::from("/cwd/themes/theme.toml"),
                PathBuf::from("/home/vfp.theme.toml"),
                PathBuf::from("/cwd/vfp.theme.toml"),
                PathBuf::from("/home/themes/vfp.theme.toml"),
                PathBuf::from("/cwd/themes/vfp.theme.toml"),
            ]
        );
    }

    #[test]
    fn resolve_explicit_path_with_slash() {
        let dirs = vec![PathBuf::from("/home")];
        let paths = resolve_theme_paths(Some("/etc/my/theme.toml"), &dirs);
        assert_eq!(paths, vec![PathBuf::from("/etc/my/theme.toml")]);
    }

    #[test]
    fn resolve_explicit_path_ending_in_toml() {
        let dirs = vec![PathBuf::from("/home")];
        let paths = resolve_theme_paths(Some("custom.theme.toml"), &dirs);
        assert_eq!(paths, vec![PathBuf::from("custom.theme.toml")]);
    }

    #[test]
    fn looks_like_path_detects_slashes_and_toml() {
        assert!(looks_like_path("./foo.toml"));
        assert!(looks_like_path("/abs/path.toml"));
        assert!(looks_like_path("dir/name"));
        assert!(looks_like_path("my.theme.toml"));
        assert!(!looks_like_path("vfp"));
        assert!(!looks_like_path("suchblue"));
    }

    #[test]
    fn load_named_theme_from_real_file() {
        let dir = std::env::temp_dir().join(format!("tv-theme-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("test.theme.toml");
        std::fs::write(&file, "pill_ok_fg = \"#FF0000\"\n").unwrap();

        let paths = resolve_theme_paths(Some("test"), &[dir.clone()]);
        let theme = load_theme_from_paths(&paths);
        assert_eq!(theme.pill_ok_fg, Color::Rgb(255, 0, 0));

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn named_theme_layers_on_base_theme_toml() {
        let dir = std::env::temp_dir().join(format!("tv-theme-layer-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();

        // Base theme.toml sets pill_ok_bg
        std::fs::write(dir.join("theme.toml"), "pill_ok_bg = \"#001100\"\n").unwrap();

        // Named theme sets pill_ok_fg only
        std::fs::write(dir.join("myname.theme.toml"), "pill_ok_fg = \"#00FF00\"\n").unwrap();

        let paths = resolve_theme_paths(Some("myname"), &[dir.clone()]);
        let theme = load_theme_from_paths(&paths);

        // Named theme value applied
        assert_eq!(theme.pill_ok_fg, Color::Rgb(0, 255, 0));
        // Base theme.toml value also applied (layered via fallback)
        assert_eq!(theme.pill_ok_bg, Color::Rgb(0, 17, 0));

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn applies_background_layer_tokens() {
        let mut theme = ComponentTheme::default();
        apply_theme_overrides(
            r##"
base_bg = "#101112"
surface_bg = "237"
chat_card_border_fg = "#2A2E32"
tree_badge_fg = "248"
"##,
            &mut theme,
        );

        assert_eq!(theme.base_bg, Color::Rgb(16, 17, 18));
        assert_eq!(theme.surface_bg, Color::Indexed(237));
        assert_eq!(theme.chat_card_border_fg, Color::Rgb(42, 46, 50));
        assert_eq!(theme.tree_badge_fg, Color::Indexed(248));
    }
}
