use anyhow::Result;
use clap::Parser;

mod commands;
mod logging;
mod providers;
mod root;
mod run;

fn main() -> Result<()> {
    let cli = root::Cli::parse();
    logging::init()?;

    run::run(cli)
}
