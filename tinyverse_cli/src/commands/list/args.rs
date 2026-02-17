use clap::Args;

use crate::commands::output::OutputFormat;

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Include unmanaged tmux sessions alongside DB-backed sessions
    #[arg(long, default_value_t = false, conflicts_with = "tmux")]
    pub all: bool,

    /// Bypass TinyVerse session storage and list tmux sessions directly
    #[arg(long, default_value_t = false)]
    pub tmux: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
