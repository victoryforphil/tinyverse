use std::time::Duration;

use anyhow::Result;
use tinyverse_tui::TuiRunOptions;

use crate::commands::tui::args::TuiArgs;

pub fn execute(args: TuiArgs) -> Result<()> {
    tinyverse_tui::run(TuiRunOptions {
        refresh_interval: Duration::from_millis(args.refresh_interval_ms),
    })
}
