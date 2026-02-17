use anyhow::Result;
use log::info;

use super::args::KillArgs;

pub fn execute(args: KillArgs) -> Result<()> {
    info!(
        "CLI // Sessions // Kill requested (meta={{\"status\":\"skeleton\",\"session\":\"{}\"}})",
        args.session
    );
    println!("kill: skeleton");
    Ok(())
}
