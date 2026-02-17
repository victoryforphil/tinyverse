use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct OpencodeServerArgs {
    #[command(subcommand)]
    pub command: Option<OpencodeServerCommands>,
}

#[derive(Debug, Subcommand)]
pub enum OpencodeServerCommands {
    /// Show managed OpenCode server status
    Status,
    /// Ensure managed OpenCode server is running
    Ensure,
    /// Restart managed OpenCode server
    Restart,
}
