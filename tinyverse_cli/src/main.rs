use anyhow::Result;
use clap::Parser;

mod commands;
mod logging;
mod providers;
mod root;
mod run;

fn main() -> Result<()> {
    logging::init()?;

    run::run(root::Cli::parse())
}
