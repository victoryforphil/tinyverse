use clap::Args;

#[derive(Debug, Args)]
pub struct ViewArgs {
    /// Panel selector: console, agent, or panel id
    #[arg(long)]
    pub panel: Option<String>,
    /// Session id (defaults to current session when available)
    #[arg(long)]
    pub session: Option<String>,
}
