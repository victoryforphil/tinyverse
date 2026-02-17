use clap::{Args, Subcommand};

use crate::commands::output::OutputFormat;

#[derive(Debug, Subcommand)]
pub enum DebugCommands {
    /// Inspect current tmux session and pane context
    #[command(name = "self")]
    SelfInfo(DebugSelfArgs),
    /// Backup and reset local session database
    #[command(name = "reset-db")]
    ResetDb,
}

#[derive(Debug, Args)]
pub struct DebugSelfArgs {
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
