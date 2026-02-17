use anyhow::Result;
use log::info;

use super::args::SpawnArgs;

pub fn execute(args: SpawnArgs) -> Result<()> {
    info!(
        "CLI // Sessions // Spawn requested (meta={{\"status\":\"skeleton\",\"agent\":\"{:?}\",\"prompt\":{},\"agent_args\":{}}})",
        args.agent,
        args.prompt.is_some(),
        args.agent_args.is_some()
    );
    println!("spawn: skeleton");
    Ok(())
}
