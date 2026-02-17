use anyhow::Result;
use clap::Parser;

mod commands;
mod logging;
mod opencode_service;
mod prompts;
mod providers;
mod root;
mod run;

fn main() -> Result<()> {
    let cli = root::Cli::parse();
    let logging_options = if matches!(cli.command, None | Some(root::Commands::Tui(_))) {
        logging::InitOptions::tui_mode()
    } else {
        logging::InitOptions::cli_default()
    };
    logging::init(cli.tinyverse_dir_home.as_deref(), logging_options)?;

    run::run(cli)
}
