use clap::Args;

#[derive(Debug, Args)]
pub struct AttachArgs {
    /// Session id or name
    pub session: String,
}
