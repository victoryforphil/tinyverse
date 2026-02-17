use clap::Args;

#[derive(Debug, Args)]
pub struct SpawnArgs {
    /// Optional session key/name
    #[arg(long)]
    pub key: Option<String>,
    /// Agent provider to use
    #[arg(long, default_value = "opencode")]
    pub agent: String,
    /// Optional starting prompt or prompt file path
    #[arg(long)]
    pub prompt: Option<String>,
    /// Optional model hint for provider templates
    #[arg(long)]
    pub model: Option<String>,
    /// Optional JSON string for provider args
    #[arg(long)]
    pub agent_args: Option<String>,
}
