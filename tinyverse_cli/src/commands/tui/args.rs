use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct TuiArgs {
    /// Session refresh rate in hertz
    #[arg(long)]
    pub refresh_hz: Option<u16>,

    /// Session refresh interval in milliseconds (legacy override)
    #[arg(long, hide = true)]
    pub refresh_interval_ms: Option<u64>,

    /// Theme name or path (e.g. "vfp", "suchblue", or "./my.theme.toml").
    /// Also configurable via TINYVERSE_THEME env var or [tui] theme in config.
    #[arg(long)]
    pub theme: Option<String>,
}

impl Default for TuiArgs {
    fn default() -> Self {
        Self {
            refresh_hz: None,
            refresh_interval_ms: None,
            theme: None,
        }
    }
}
