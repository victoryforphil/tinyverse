use clap::{Parser, Subcommand};

use crate::commands::{
    config::command::ConfigCommands, kill::args::KillArgs, send::args::SendArgs,
    spawn::args::SpawnArgs, view::args::ViewArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "tinyverse",
    version,
    about = "tmux-based agent session harness",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List available agent providers and status
    Providers,
    /// View and edit tinyverse config
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Show tinyverse data directory
    Path,
    /// List known tinyverse sessions
    List,
    /// Spawn a new tinyverse session
    Spawn(SpawnArgs),
    /// Kill a tinyverse session by id or name
    Kill(KillArgs),
    /// View panel text buffer
    View(ViewArgs),
    /// Send command to a session panel
    Send(SendArgs),
}
