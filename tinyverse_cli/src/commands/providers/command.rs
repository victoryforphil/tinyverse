use anyhow::Result;
use log::info;

pub fn execute() -> Result<()> {
    info!("CLI // Providers // Listing providers (meta={{\"status\":\"skeleton\"}})");
    println!("providers: skeleton");
    Ok(())
}
