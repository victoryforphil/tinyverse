use anyhow::Result;
use clap::Parser;

mod commands;
mod logging;
mod prompts;
mod providers;
mod root;
mod run;

fn main() -> Result<()> {
    let cli = root::Cli::parse();
    logging::init(cli.tinyverse_dir_home.as_deref())?;

    run::run(cli)
}
