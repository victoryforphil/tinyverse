use std::path::PathBuf;

use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ViewOutput {
    #[value(alias = "current")]
    Full,
    Raw,
}

#[derive(Debug, Args)]
pub struct ViewArgs {
    /// Panel selector: console, agent, or panel id
    #[arg(long)]
    pub panel: Option<String>,
    /// Session id (defaults to current session when available)
    #[arg(long)]
    pub session: Option<String>,

    /// Output mode: full (panel + metadata) or raw (pane buffer only)
    #[arg(long, value_enum, default_value_t = ViewOutput::Full)]
    pub output: ViewOutput,

    /// Export rendered output to a file (.md auto-appended when missing)
    #[arg(long)]
    pub export: Option<PathBuf>,
}
