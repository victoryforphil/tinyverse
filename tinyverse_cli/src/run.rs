use anyhow::Result;
use tinyverse_lib::{SessionStore, TINYVERSE_DIR_HOME_ENV};

use crate::commands;
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
        Commands::Providers => commands::providers::command::execute(),
        Commands::Config { command } => commands::config::command::execute(command),
        Commands::Path => commands::path::command::execute(),
        Commands::List(args) => commands::list::command::execute(args),
        Commands::Spawn(args) => commands::spawn::command::execute(args),
        Commands::Attach(args) => commands::attach::command::execute(args),
        Commands::Kill(args) => commands::kill::command::execute(args),
        Commands::View(args) => commands::view::command::execute(args),
        Commands::Send(args) => commands::send::command::execute(args),
        Commands::Debug { command } => commands::debug::command::execute(command),
    }
}
