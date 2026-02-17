use anyhow::Result;
use log::info;

use crate::providers;

pub fn execute() -> Result<()> {
    let provider_count = providers::all().len();
    info!("Available providers ({provider_count}):");

    for provider in providers::all() {
        let metadata = provider.metadata();
        info!("- {} ({})", metadata.name, metadata.key);
    }

    Ok(())
}
