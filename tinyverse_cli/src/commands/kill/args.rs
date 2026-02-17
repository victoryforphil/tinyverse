use clap::Args;

#[derive(Debug, Args)]
pub struct KillArgs {
    /// TinyVerse session key or name to terminate (interactive picker when omitted)
    pub session: Option<String>,

    /// Kill all TinyVerse sessions
    #[arg(long, short = 'a', default_value_t = false, conflicts_with = "session")]
    pub all: bool,
}
