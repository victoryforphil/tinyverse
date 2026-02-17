use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum PromptCommands {
    /// Render the effective launch prompt for an agent
    Render(RenderPromptArgs),
}

#[derive(Debug, Args)]
pub struct RenderPromptArgs {
    /// Agent provider key (e.g. opencode)
    #[arg(long, default_value = "opencode")]
    pub agent: String,
    /// Optional prompt string or prompt file path
    #[arg(long)]
    pub prompt: Option<String>,
}
