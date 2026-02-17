mod app;
mod chat;
mod chat_bridge;
mod logger;
mod prefs;
mod runtime;
mod theme;

use std::time::Duration;

use anyhow::Result;

pub use app::App;
pub use theme::resolve_theme_selector;

#[derive(Debug, Clone)]
pub struct TuiRunOptions {
    pub refresh_interval: Duration,
    /// Theme selector: short name, path, or `None` for default `theme.toml`.
    pub theme: Option<String>,
}

impl Default for TuiRunOptions {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_millis(1200),
            theme: None,
        }
    }
}

pub fn run(options: TuiRunOptions) -> Result<()> {
    runtime::run(options)
}
