use anyhow::Result;
use tinyverse_lib::{SessionStore, TINYVERSE_DIR_HOME_ENV};

use crate::commands;
use crate::commands::tui::args::TuiArgs;
use crate::root::{Cli, Commands};

pub fn run(cli: Cli) -> Result<()> {
    if let Some(path) = cli.tinyverse_dir_home.as_ref() {
        // SAFETY: this process sets one known environment variable during startup,
        // before any command worker threads are spawned.
        unsafe {
            std::env::set_var(TINYVERSE_DIR_HOME_ENV, path);
        }
    }

    let mut store = SessionStore::open_default()?;
    store.reconcile_now()?;

    match cli.command {
        Some(Commands::Providers) => commands::providers::command::execute(),
        Some(Commands::Config { command }) => commands::config::command::execute(command),
        Some(Commands::Path) => commands::path::command::execute(),
        Some(Commands::Prompt { command }) => commands::prompt::command::execute(command),
        Some(Commands::List(args)) => commands::list::command::execute(args),
        Some(Commands::Spawn(args)) => commands::spawn::command::execute(args),
        Some(Commands::Attach(args)) => commands::attach::command::execute(args),
        Some(Commands::Detach(args)) => commands::detach::command::execute(args),
        Some(Commands::Kill(args)) => commands::kill::command::execute(args),
        Some(Commands::View(args)) => commands::view::command::execute(args),
        Some(Commands::Send(args)) => commands::send::command::execute(args),
        Some(Commands::Tui(args)) => commands::tui::command::execute(args),
        Some(Commands::GhaBabysit(args)) => commands::gha_babysit::command::execute(args),
        Some(Commands::Debug { command }) => commands::debug::command::execute(command),
        None => commands::tui::command::execute(TuiArgs::default()),
    }
}
