use anyhow::Result;
use log::info;
use tinyverse_lib::{resolve_tinyverse_paths, TinyverseHomeSource};

pub fn execute() -> Result<()> {
    let paths = resolve_tinyverse_paths(None)?;
    let source = match paths.source {
        TinyverseHomeSource::ArgOverride => "arg",
        TinyverseHomeSource::EnvOverride => "env",
        TinyverseHomeSource::RepoLocal => "repo_local",
        TinyverseHomeSource::CwdLocal => "cwd_local",
        TinyverseHomeSource::UserHome => "home",
    };

    info!("tinyverse_home: {}", paths.home_dir.display());
    info!("tinyverse_db: {}", paths.db_path.display());
    info!("tinyverse_home_source: {source}");
    Ok(())
}
