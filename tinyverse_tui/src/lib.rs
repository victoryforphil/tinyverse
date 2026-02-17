mod app;
mod prefs;
mod runtime;

use std::time::Duration;

use anyhow::Result;

pub use app::App;

#[derive(Debug, Clone)]
pub struct TuiRunOptions {
    pub refresh_interval: Duration,
}

impl Default for TuiRunOptions {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_millis(1200),
        }
    }
}

pub fn run(options: TuiRunOptions) -> Result<()> {
    runtime::run(options)
}
