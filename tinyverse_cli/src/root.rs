use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tinyverse_lib::TINYVERSE_DIR_HOME_ENV;

use crate::commands::{
    attach::args::AttachArgs, config::command::ConfigCommands, debug::args::DebugCommands,
    detach::args::DetachArgs, gha_babysit::args::GhaBabysitArgs, kill::args::KillArgs,
    list::args::ListArgs, opencode_server::args::OpencodeServerArgs, prompt::args::PromptCommands,
    send::args::SendArgs, spawn::args::SpawnArgs, tui::args::TuiArgs, view::args::ViewArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "tinyverse",
    version,
    about = "tmux-based agent session harness"
)]
pub struct Cli {
    /// Override tinyverse home directory (or a directory containing .tinyverse)
    #[arg(long, global = true, env = TINYVERSE_DIR_HOME_ENV)]
    pub tinyverse_dir_home: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
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
    /// Prompt template utilities
    Prompt {
        #[command(subcommand)]
        command: PromptCommands,
    },
    /// Manage the shared OpenCode service (serve/web) used by TinyVerse
    #[command(name = "opencode-server")]
    OpencodeServer(OpencodeServerArgs),
    /// List known TinyVerse sessions
    List(ListArgs),
    /// Spawn a new TinyVerse session
    Spawn(SpawnArgs),
    /// Attach to a TinyVerse session by key or name
    Attach(AttachArgs),
    /// Detach current tmux client without closing session
    Detach(DetachArgs),
    /// Kill a TinyVerse session by key/name, or all with --all
    Kill(KillArgs),
    /// View panel text buffer
    View(ViewArgs),
    /// Send command to a session panel
    Send(SendArgs),
    /// Launch the interactive TUI dashboard
    Tui(TuiArgs),
    /// Print the GitHub Actions fix/push/watch loop playbook
    #[command(name = "gha-babysit")]
    GhaBabysit(GhaBabysitArgs),
    /// Debugging utilities
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },
}
