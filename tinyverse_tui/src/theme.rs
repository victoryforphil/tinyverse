use std::path::PathBuf;

use tinyverse_lib::resolve_tinyverse_paths;
use tinyverse_tui_components::{ComponentTheme, load_theme_from_paths, resolve_theme_paths};

const ENV_THEME: &str = "TINYVERSE_THEME";

pub type UiTheme = ComponentTheme;

/// Load a theme by optional selector (name or path).
///
/// Resolution order:
///   1. If `selector` looks like a path (`/` or `.toml`), use it directly.
///   2. If `selector` is a short name, look for `<name>.theme.toml` in the
///      tinyverse home dir, then cwd, then fall back to `theme.toml` in each.
///   3. If `selector` is `None`, look for `theme.toml` in home dir then cwd.
pub fn load_theme(selector: Option<&str>) -> UiTheme {
    let paths = resolve_theme_paths(selector, &search_dirs());
    load_theme_from_paths(paths)
}

/// Resolve the effective theme selector from (in priority order):
///   1. Explicit CLI/caller value
///   2. `TINYVERSE_THEME` env var
///   3. Config `tui.theme` value
///   4. `None` (plain `theme.toml` fallback)
pub fn resolve_theme_selector(
    cli_theme: Option<&str>,
    config_theme: Option<&str>,
) -> Option<String> {
    if let Some(val) = cli_theme {
        let trimmed = val.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }
    if let Ok(val) = std::env::var(ENV_THEME) {
        let trimmed = val.trim().to_owned();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    if let Some(val) = config_theme {
        let trimmed = val.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }
    None
}

fn search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(resolved) = resolve_tinyverse_paths(None) {
        dirs.push(resolved.home_dir);
    }
    if let Ok(cwd) = std::env::current_dir() {
        dirs.push(cwd);
    }
    dirs
}
