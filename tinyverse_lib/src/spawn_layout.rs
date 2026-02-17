use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::dir_utils::resolve_tinyverse_paths;
use crate::tmux::{PanelRole, SplitDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxSpawnLayout {
    pub initial_window_width: u16,
    pub initial_window_height: u16,
    pub split_direction: SplitDirection,
    pub primary_role: PanelRole,
    pub secondary_size_percent: u8,
}

impl Default for TmuxSpawnLayout {
    fn default() -> Self {
        Self {
            initial_window_width: 220,
            initial_window_height: 64,
            split_direction: SplitDirection::Horizontal,
            primary_role: PanelRole::Agent,
            secondary_size_percent: 35,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct PartialTinyverseConfig {
    tmux: Option<PartialTmuxConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialTmuxConfig {
    initial_window_width: Option<u16>,
    initial_window_height: Option<u16>,
    layout: Option<PartialTmuxLayoutConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialTmuxLayoutConfig {
    direction: Option<TmuxLayoutDirection>,
    primary: Option<TmuxLayoutPrimary>,
    secondary_percent: Option<u8>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TmuxLayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TmuxLayoutPrimary {
    Agent,
    Console,
}

pub fn load_tmux_spawn_layout() -> TmuxSpawnLayout {
    let mut layout = TmuxSpawnLayout::default();

    let resolved = match resolve_tinyverse_paths(None) {
        Ok(value) => value,
        Err(_) => return layout,
    };

    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        let home_cfg_dir = home.join(".tinyverse");
        if home_cfg_dir != resolved.home_dir {
            apply_first_existing(&mut layout, &home_cfg_dir);
        }
    }

    apply_first_existing(&mut layout, &resolved.home_dir);
    layout
}

fn apply_first_existing(layout: &mut TmuxSpawnLayout, home_dir: &Path) {
    let primary = home_dir.join("config.toml");
    if primary.is_file() {
        apply_from_file(layout, &primary);
        return;
    }

    let legacy = home_dir.join("tinyverse.toml");
    if legacy.is_file() {
        apply_from_file(layout, &legacy);
    }
}

fn apply_from_file(layout: &mut TmuxSpawnLayout, path: &Path) {
    let Ok(raw) = std::fs::read_to_string(path) else {
        return;
    };
    let Ok(parsed) = toml::from_str::<PartialTinyverseConfig>(&raw) else {
        return;
    };

    let Some(tmux) = parsed.tmux else {
        return;
    };

    if let Some(width) = tmux.initial_window_width
        && width > 0
    {
        layout.initial_window_width = width;
    }
    if let Some(height) = tmux.initial_window_height
        && height > 0
    {
        layout.initial_window_height = height;
    }

    if let Some(layout_cfg) = tmux.layout {
        if let Some(direction) = layout_cfg.direction {
            layout.split_direction = match direction {
                TmuxLayoutDirection::Horizontal => SplitDirection::Horizontal,
                TmuxLayoutDirection::Vertical => SplitDirection::Vertical,
            };
        }
        if let Some(primary) = layout_cfg.primary {
            layout.primary_role = match primary {
                TmuxLayoutPrimary::Agent => PanelRole::Agent,
                TmuxLayoutPrimary::Console => PanelRole::Console,
            };
        }
        if let Some(percent) = layout_cfg.secondary_percent
            && (1..=99).contains(&percent)
        {
            layout.secondary_size_percent = percent;
        }
    }
}
