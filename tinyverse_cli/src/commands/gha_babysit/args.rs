use clap::Args;

use crate::commands::output::OutputFormat;

#[derive(Debug, Clone, Args)]
pub struct GhaBabysitArgs {
    /// Branch to inspect when listing recent runs
    #[arg(long, default_value = "main")]
    pub branch: String,

    /// Optional run id to target when inspecting failed logs
    #[arg(long)]
    pub run_id: Option<u64>,

    /// Maximum fix/push/watch iterations before escalating
    #[arg(long, default_value_t = 5)]
    pub max_attempts: u8,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
