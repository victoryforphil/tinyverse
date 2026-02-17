use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct TuiArgs {
    /// Session refresh rate in hertz
    #[arg(long)]
    pub refresh_hz: Option<u16>,

    /// Session refresh interval in milliseconds (legacy override)
    #[arg(long, hide = true)]
    pub refresh_interval_ms: Option<u64>,
}

impl Default for TuiArgs {
    fn default() -> Self {
        Self {
            refresh_hz: None,
            refresh_interval_ms: None,
        }
    }
}
