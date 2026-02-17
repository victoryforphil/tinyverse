use clap::Args;

#[derive(Debug, Args)]
pub struct SpawnArgs {
    /// Optional session key (auto-generated city name if omitted)
    #[arg(long)]
    pub key: Option<String>,
    /// Agent provider to use (e.g. opencode)
    #[arg(long)]
    pub agent: Option<String>,
    /// Optional starting prompt or prompt file path
    #[arg(long)]
    pub prompt: Option<String>,
    /// Optional model hint for provider templates
    #[arg(long)]
    pub model: Option<String>,
    /// Optional JSON string for provider args
    #[arg(long)]
    pub agent_args: Option<String>,
    /// Start panes with `zsh -f` (ignores user zshrc)
    #[arg(long, action = clap::ArgAction::SetTrue, overrides_with = "no_clean_shell")]
    pub clean_shell: bool,
    /// Start panes with default login shell behavior
    #[arg(long = "no-clean-shell", action = clap::ArgAction::SetTrue, overrides_with = "clean_shell")]
    pub no_clean_shell: bool,
}
