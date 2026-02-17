use std::process::Command;

use anyhow::{Context, Result, bail};
use log::info;

use super::args::DetachArgs;

pub fn execute(_args: DetachArgs) -> Result<()> {
    if std::env::var_os("TMUX").is_none() {
        bail!("detach must be run from an attached tmux client");
    }

    let status = Command::new("tmux")
        .args(["detach-client"])
        .status()
        .context("failed to run tmux detach-client")?;

    if !status.success() {
        bail!(
            "failed to detach tmux client (exit code: {:?})",
            status.code()
        );
    }

    info!("Detached from tmux client; session remains running");
    Ok(())
}
