use clap::Args;

#[derive(Debug, Args)]
pub struct AttachArgs {
    /// TinyVerse session key or name to attach to (interactive picker when omitted)
    pub session: Option<String>,
}
