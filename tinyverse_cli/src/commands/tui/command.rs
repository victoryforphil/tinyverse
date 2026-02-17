use std::time::Duration;

use anyhow::Result;
use tinyverse_tui::TuiRunOptions;

use crate::commands::config::store;
use crate::commands::tui::args::TuiArgs;

pub fn execute(args: TuiArgs) -> Result<()> {
    let config = store::load()?;
    let refresh_interval = match (args.refresh_interval_ms, args.refresh_hz) {
        (Some(ms), _) => Duration::from_millis(ms.max(1)),
        (None, Some(hz)) => Duration::from_millis(1_000 / u64::from(hz.max(1))),
        (None, None) => Duration::from_millis(1_000 / u64::from(config.tui.refresh_hz.max(1))),
    };

    tinyverse_tui::run(TuiRunOptions { refresh_interval })
}
