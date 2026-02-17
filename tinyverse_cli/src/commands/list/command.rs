use anyhow::Result;
use log::info;

pub fn execute() -> Result<()> {
    info!("CLI // Sessions // List requested (meta={{\"status\":\"skeleton\"}})");
    println!("list: skeleton");
    Ok(())
}
