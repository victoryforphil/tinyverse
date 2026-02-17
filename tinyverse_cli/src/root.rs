use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tinyverse_lib::TINYVERSE_DIR_HOME_ENV;

use crate::commands::{
    attach::args::AttachArgs, config::command::ConfigCommands, debug::args::DebugCommands,
    kill::args::KillArgs, list::args::ListArgs, send::args::SendArgs, spawn::args::SpawnArgs,
    view::args::ViewArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "tinyverse",
    version,
    about = "tmux-based agent session harness",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Override tinyverse home directory (or a directory containing .tinyverse)
    #[arg(long, global = true, env = TINYVERSE_DIR_HOME_ENV)]
    pub tinyverse_dir_home: Option<PathBuf>,

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
    List(ListArgs),
    /// Spawn a new tinyverse session
    Spawn(SpawnArgs),
    /// Attach to a tinyverse session by id or name
    Attach(AttachArgs),
    /// Kill a tinyverse session by id/name, or all with --all
    Kill(KillArgs),
    /// View panel text buffer
    View(ViewArgs),
    /// Send command to a session panel
    Send(SendArgs),
    /// Debugging utilities
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },
}
