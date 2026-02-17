use anyhow::Result;
use clap::Parser;

mod commands;
mod root;
mod run;

fn main() -> Result<()> {
    pretty_env_logger::init();
    run::run(root::Cli::parse())
}
