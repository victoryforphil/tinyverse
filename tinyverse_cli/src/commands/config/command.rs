use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Export config as TOML
    Export,
    /// Print config in human format
    Print,
}

pub fn execute(command: ConfigCommands) -> Result<()> {
    match command {
        ConfigCommands::Export => super::export::execute(),
        ConfigCommands::Print => super::print::execute(),
    }
}
