use anyhow::Result;
use log::info;

pub fn execute() -> Result<()> {
    info!("CLI // Path // Path requested (meta={{\"status\":\"skeleton\"}})");
    println!("path: skeleton");
    Ok(())
}
