use clap::{Args, Subcommand};

use crate::commands::output::OutputFormat;

#[derive(Debug, Subcommand)]
pub enum DebugCommands {
    /// Inspect current tmux session and pane context
    #[command(name = "self")]
    SelfInfo(DebugSelfArgs),
}

#[derive(Debug, Args)]
pub struct DebugSelfArgs {
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}
