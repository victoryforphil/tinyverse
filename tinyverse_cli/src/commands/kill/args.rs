use clap::Args;

#[derive(Debug, Args)]
pub struct KillArgs {
    /// Session id or name
    #[arg(required_unless_present = "all")]
    pub session: Option<String>,

    /// Kill all tinyverse sessions
    #[arg(long, short = 'a', default_value_t = false, conflicts_with = "session")]
    pub all: bool,
}
