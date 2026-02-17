use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct OpencodeServerArgs {
    #[command(subcommand)]
    pub command: Option<OpencodeServerCommands>,
}

#[derive(Debug, Subcommand)]
pub enum OpencodeServerCommands {
    /// Show managed OpenCode service status
    Status,
    /// Ensure managed OpenCode service is running
    Ensure,
    /// Restart managed OpenCode service
    Restart,
}
