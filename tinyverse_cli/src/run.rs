use anyhow::Result;
use log::{debug, error, info, warn};
use tinyverse_lib::{SessionStore, TINYVERSE_DIR_HOME_ENV};

use crate::commands;
use crate::commands::config::store;
use crate::commands::tui::args::TuiArgs;
use crate::opencode_service::ensure_managed_opencode_service;
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
    debug!("session store opened and reconciled");

    let config = store::load()?;
    let should_block_on_managed_service = matches!(
        cli.command,
        None | Some(Commands::Tui(_)) | Some(Commands::Spawn(_))
    );
    let mut managed_service_base_url: Option<String> = None;
    match ensure_managed_opencode_service(&mut store, &config) {
        Ok(Some(service)) => {
            info!(
                "managed opencode service ready (base_url={}, tmux_session={})",
                service.base_url, service.tmux_session_name
            );
            managed_service_base_url = Some(service.base_url);
        }
        Ok(None) => {
            debug!("managed opencode service is disabled via config");
        }
        Err(err) => {
            if should_block_on_managed_service {
                error!("managed opencode service unavailable: {err}");
                return Err(err);
            }
            warn!("managed opencode service unavailable; continuing: {err}");
        }
    }

    if matches!(cli.command, None | Some(Commands::Tui(_)))
        && let Some(base_url) = managed_service_base_url.as_deref()
    {
        debug!("setting TINYVERSE_CHAT_OPENCODE_BASE_URL for TUI");
        // SAFETY: this process sets one known environment variable during startup,
        // before the TUI worker loop begins.
        unsafe {
            std::env::set_var("TINYVERSE_CHAT_OPENCODE_BASE_URL", base_url);
        }
    }

    info!("dispatching tinyverse command");
    match cli.command {
        Some(Commands::Providers) => commands::providers::command::execute(),
        Some(Commands::Config { command }) => commands::config::command::execute(command),
        Some(Commands::Path) => commands::path::command::execute(),
        Some(Commands::Prompt { command }) => commands::prompt::command::execute(command),
        Some(Commands::OpencodeServer(args)) => commands::opencode_server::command::execute(args),
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
