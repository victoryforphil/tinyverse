use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use super::store;

#[derive(Debug, Args)]
pub struct ConfigExportArgs {
    /// Optional output config file path
    #[arg(long)]
    pub path: Option<PathBuf>,
}

pub fn execute(args: ConfigExportArgs) -> Result<()> {
    let loaded = store::load_with_context()?;
    let output_path = args.path.unwrap_or_else(|| loaded.active_path.clone());
    let selected_source = loaded.source_label();

    store::save_to_path(&loaded.config, output_path.clone())?;

    let output = toml::to_string_pretty(&loaded.config)?;
    println!("# TinyVerse config export");
    println!("# selected_source = {selected_source}");
    println!("# selected_home = {}", loaded.selected_home.display());
    println!("# active_path = {}", loaded.active_path.display());
    println!("# legacy_path = {}", loaded.legacy_path.display());
    println!("# written_to = {}", output_path.display());
    if loaded.loaded_paths.is_empty() {
        println!("# loaded_from = <none>");
    } else {
        for path in loaded.loaded_paths {
            println!("# loaded_from = {}", path.display());
        }
    }
    println!();
    println!("{output}");
    Ok(())
}
