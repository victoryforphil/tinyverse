use clap::Args;

#[derive(Debug, Args)]
pub struct SendArgs {
    /// Command to send to the panel
    pub command: String,
    /// Session id (defaults to current session when available)
    #[arg(long)]
    pub session: Option<String>,
    /// Panel selector: console, agent, or panel id
    #[arg(long)]
    pub panel: Option<String>,
}
