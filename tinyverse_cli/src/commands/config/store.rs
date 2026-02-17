use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tinyverse_lib::{TinyverseHomeSource, resolve_tinyverse_paths};

const CONFIG_FILE_NAME: &str = "config.toml";
const LEGACY_CONFIG_FILE_NAME: &str = "tinyverse.toml";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TinyverseConfig {
    #[serde(default)]
    pub shell: ShellConfig,
    #[serde(default)]
    pub workspace: WorkspaceConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub spawn: SpawnConfig,
    #[serde(default)]
    pub tmux: TmuxConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShellConfig {
    #[serde(default)]
    pub clean: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub default_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitConfig {
    #[serde(default = "default_branch_prefix")]
    pub branch_prefix: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            branch_prefix: default_branch_prefix(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpawnConfig {
    #[serde(default = "default_spawn_agent")]
    pub default_agent: String,
    #[serde(default)]
    pub default_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TmuxConfig {
    #[serde(default = "default_tmux_initial_window_width")]
    pub initial_window_width: u16,
    #[serde(default = "default_tmux_initial_window_height")]
    pub initial_window_height: u16,
    #[serde(default)]
    pub layout: TmuxLayoutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TmuxLayoutConfig {
    #[serde(default = "default_tmux_layout_direction")]
    pub direction: TmuxLayoutDirection,
    #[serde(default = "default_tmux_layout_primary")]
    pub primary: TmuxLayoutPrimary,
    #[serde(default = "default_tmux_layout_secondary_percent")]
    pub secondary_percent: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TmuxLayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TmuxLayoutPrimary {
    Agent,
    Console,
}

impl Default for TmuxConfig {
    fn default() -> Self {
        Self {
            initial_window_width: default_tmux_initial_window_width(),
            initial_window_height: default_tmux_initial_window_height(),
            layout: TmuxLayoutConfig::default(),
        }
    }
}

impl Default for TmuxLayoutConfig {
    fn default() -> Self {
        Self {
            direction: default_tmux_layout_direction(),
            primary: default_tmux_layout_primary(),
            secondary_percent: default_tmux_layout_secondary_percent(),
        }
    }
}

impl Default for SpawnConfig {
    fn default() -> Self {
        Self {
            default_agent: default_spawn_agent(),
            default_model: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadedConfig {
    pub config: TinyverseConfig,
    pub selected_home: PathBuf,
    pub selected_source: TinyverseHomeSource,
    pub active_path: PathBuf,
    pub legacy_path: PathBuf,
    pub loaded_paths: Vec<PathBuf>,
}

impl LoadedConfig {
    pub fn source_label(&self) -> &'static str {
        match self.selected_source {
            TinyverseHomeSource::ArgOverride => "arg_override",
            TinyverseHomeSource::EnvOverride => "env_override",
            TinyverseHomeSource::RepoLocal => "repo_local",
            TinyverseHomeSource::CwdLocal => "cwd_local",
            TinyverseHomeSource::UserHome => "user_home",
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct PartialTinyverseConfig {
    shell: Option<PartialShellConfig>,
    workspace: Option<PartialWorkspaceConfig>,
    git: Option<PartialGitConfig>,
    spawn: Option<PartialSpawnConfig>,
    tmux: Option<PartialTmuxConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialShellConfig {
    clean: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialWorkspaceConfig {
    default_dir: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialGitConfig {
    branch_prefix: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialSpawnConfig {
    default_agent: Option<String>,
    default_model: Option<String>,
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

pub fn load() -> Result<TinyverseConfig> {
    Ok(load_with_context()?.config)
}

pub fn load_with_context() -> Result<LoadedConfig> {
    let resolved = resolve_tinyverse_paths(None)?;
    let selected_home = resolved.home_dir.clone();
    let selected_source = resolved.source;
    let active_path = selected_home.join(CONFIG_FILE_NAME);
    let legacy_path = selected_home.join(LEGACY_CONFIG_FILE_NAME);

    let mut config = TinyverseConfig::default();
    let mut loaded_paths = Vec::new();

    let home_base = user_home_dir().map(|home| home.join(".tinyverse"));
    if let Some(home_base) = home_base.as_ref() {
        if *home_base != selected_home {
            load_first_existing_into(&mut config, home_base, &mut loaded_paths)?;
        }
    }

    load_first_existing_into(&mut config, &selected_home, &mut loaded_paths)?;

    Ok(LoadedConfig {
        config,
        selected_home,
        selected_source,
        active_path,
        legacy_path,
        loaded_paths,
    })
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn load_from_path(path: PathBuf) -> Result<TinyverseConfig> {
    if !path.exists() {
        return Ok(TinyverseConfig::default());
    }

    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config `{}`", path.display()))?;
    let parsed = toml::from_str::<TinyverseConfig>(&raw)
        .with_context(|| format!("failed to parse config `{}`", path.display()))?;
    Ok(parsed)
}

pub fn save_to_path(config: &TinyverseConfig, path: PathBuf) -> Result<PathBuf> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create config parent directory `{}`",
                parent.display()
            )
        })?;
    }

    let content = toml::to_string_pretty(config).context("failed to serialize config")?;
    std::fs::write(&path, content)
        .with_context(|| format!("failed to write config `{}`", path.display()))?;
    Ok(path)
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn load_first_existing_into(
    target: &mut TinyverseConfig,
    home_dir: &Path,
    loaded_paths: &mut Vec<PathBuf>,
) -> Result<()> {
    let primary = home_dir.join(CONFIG_FILE_NAME);
    if primary.is_file() {
        let partial = parse_partial(&primary)?;
        apply_partial(target, partial);
        loaded_paths.push(primary);
        return Ok(());
    }

    let legacy = home_dir.join(LEGACY_CONFIG_FILE_NAME);
    if legacy.is_file() {
        let partial = parse_partial(&legacy)?;
        apply_partial(target, partial);
        loaded_paths.push(legacy);
    }

    Ok(())
}

fn parse_partial(path: &Path) -> Result<PartialTinyverseConfig> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config `{}`", path.display()))?;
    toml::from_str::<PartialTinyverseConfig>(&raw)
        .with_context(|| format!("failed to parse config `{}`", path.display()))
}

fn apply_partial(target: &mut TinyverseConfig, partial: PartialTinyverseConfig) {
    let PartialTinyverseConfig {
        shell,
        workspace,
        git,
        spawn,
        tmux,
    } = partial;

    if let Some(shell) = shell
        && let Some(clean) = shell.clean
    {
        target.shell.clean = clean;
    }

    if let Some(workspace) = workspace {
        target.workspace.default_dir = workspace.default_dir;
    }

    if let Some(git) = git
        && let Some(branch_prefix) = git.branch_prefix
    {
        target.git.branch_prefix = branch_prefix;
    }

    if let Some(spawn) = spawn {
        if let Some(default_agent) = spawn.default_agent {
            target.spawn.default_agent = default_agent;
        }
        if let Some(default_model) = spawn.default_model {
            target.spawn.default_model = Some(default_model);
        }
    }

    if let Some(tmux) = tmux {
        if let Some(initial_window_width) = tmux.initial_window_width {
            target.tmux.initial_window_width = initial_window_width;
        }
        if let Some(initial_window_height) = tmux.initial_window_height {
            target.tmux.initial_window_height = initial_window_height;
        }
        if let Some(layout) = tmux.layout {
            if let Some(direction) = layout.direction {
                target.tmux.layout.direction = direction;
            }
            if let Some(primary) = layout.primary {
                target.tmux.layout.primary = primary;
            }
            if let Some(secondary_percent) = layout.secondary_percent {
                target.tmux.layout.secondary_percent = secondary_percent;
            }
        }
    }
}

fn default_branch_prefix() -> String {
    "tv/".to_owned()
}

fn default_spawn_agent() -> String {
    "opencode".to_owned()
}

fn default_tmux_initial_window_width() -> u16 {
    180
}

fn default_tmux_initial_window_height() -> u16 {
    54
}

fn default_tmux_layout_secondary_percent() -> u8 {
    40
}

fn default_tmux_layout_direction() -> TmuxLayoutDirection {
    TmuxLayoutDirection::Horizontal
}

fn default_tmux_layout_primary() -> TmuxLayoutPrimary {
    TmuxLayoutPrimary::Agent
}

#[cfg(test)]
mod tests {
    use super::{TinyverseConfig, load_from_path, save_to_path};

    #[test]
    fn load_missing_config_returns_defaults() {
        let file_path = std::env::temp_dir().join(format!(
            "tinyverse-config-test-missing-{}",
            std::process::id()
        ));
        if file_path.exists() {
            let _ = std::fs::remove_file(&file_path);
        }

        let loaded = load_from_path(file_path).expect("loading missing config should succeed");
        assert_eq!(loaded, TinyverseConfig::default());
    }

    #[test]
    fn save_then_load_round_trip() {
        let dir =
            std::env::temp_dir().join(format!("tinyverse-config-test-{}", std::process::id()));
        let path = dir.join("config.toml");
        let config = TinyverseConfig {
            shell: super::ShellConfig { clean: true },
            workspace: super::WorkspaceConfig {
                default_dir: Some("./workspace".to_owned()),
            },
            git: super::GitConfig {
                branch_prefix: "feature/".to_owned(),
            },
            spawn: super::SpawnConfig {
                default_agent: "opencode".to_owned(),
                default_model: Some("gpt-5".to_owned()),
            },
            tmux: super::TmuxConfig::default(),
        };

        let written = save_to_path(&config, path.clone()).expect("save should succeed");
        assert_eq!(written, path);

        let loaded = load_from_path(path).expect("load should succeed");
        assert_eq!(loaded, config);

        let _ = std::fs::remove_file(written);
        let _ = std::fs::remove_dir_all(dir);
    }
}
