use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct TuiArgs {
    /// Session refresh interval in milliseconds
    #[arg(long, default_value_t = 3000)]
    pub refresh_interval_ms: u64,
}
