use anyhow::Result;
use log::info;

use super::args::ViewArgs;

pub fn execute(args: ViewArgs) -> Result<()> {
    info!(
        "CLI // Sessions // View requested (meta={{\"status\":\"skeleton\",\"session\":\"{}\",\"panel\":\"{}\"}})",
        args.session.unwrap_or_else(|| "current".to_string()),
        args.panel.unwrap_or_else(|| "console".to_string())
    );
    println!("view: skeleton");
    Ok(())
}
