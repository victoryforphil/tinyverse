use anyhow::Result;
use clap::Subcommand;

use super::export::ConfigExportArgs;
use super::print::ConfigPrintArgs;
use super::set::ConfigSetArgs;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Export config as TOML
    Export(ConfigExportArgs),
    /// Print config in human format
    Print(ConfigPrintArgs),
    /// Set a config value
    Set(ConfigSetArgs),
}

pub fn execute(command: ConfigCommands) -> Result<()> {
    match command {
        ConfigCommands::Export(args) => super::export::execute(args),
        ConfigCommands::Print(args) => super::print::execute(args),
        ConfigCommands::Set(args) => super::set::execute(args),
    }
}
