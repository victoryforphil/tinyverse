use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct SpawnArgs {
    /// Agent provider to use
    #[arg(long, value_enum, default_value_t = Agent::Opencode)]
    pub agent: Agent,
    /// Optional starting prompt or prompt file path
    #[arg(long)]
    pub prompt: Option<String>,
    /// Optional JSON string for provider args
    #[arg(long)]
    pub agent_args: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Agent {
    Opencode,
}
