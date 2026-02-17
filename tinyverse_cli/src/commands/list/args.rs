use clap::Args;

use crate::commands::output::OutputFormat;

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show all tmux sessions, not only tinyverse sessions
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
