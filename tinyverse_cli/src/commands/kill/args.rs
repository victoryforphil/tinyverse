use clap::Args;

#[derive(Debug, Args)]
pub struct KillArgs {
    /// Session id or name
    pub session: String,
}
