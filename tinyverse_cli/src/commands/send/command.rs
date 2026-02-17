use anyhow::Result;
use log::info;

use super::args::SendArgs;

pub fn execute(args: SendArgs) -> Result<()> {
    info!(
        "CLI // Sessions // Send requested (meta={{\"status\":\"skeleton\",\"session\":\"{}\",\"panel\":\"{}\"}})",
        args.session.unwrap_or_else(|| "current".to_string()),
        args.panel.unwrap_or_else(|| "console".to_string())
    );
    println!("send: skeleton -> {}", args.command);
    Ok(())
}
