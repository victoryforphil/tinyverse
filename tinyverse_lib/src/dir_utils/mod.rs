use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub const TINYVERSE_DIR_HOME_ENV: &str = "TINYVERSE_DIR_HOME";
const TINYVERSE_DIR_NAME: &str = ".tinyverse";
const DEFAULT_DB_FILE_NAME: &str = "tinyverse_sessions.sqlite3";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TinyverseHomeSource {
    ArgOverride,
    EnvOverride,
    RepoLocal,
    CwdLocal,
    UserHome,
}

#[derive(Debug, Clone)]
pub struct TinyversePaths {
    pub home_dir: PathBuf,
    pub db_path: PathBuf,
    pub source: TinyverseHomeSource,
}

pub fn resolve_tinyverse_paths(arg_override: Option<&Path>) -> Result<TinyversePaths> {
    let cwd = std::env::current_dir().context("failed to read current working directory")?;
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME environment variable is not set")?;

    let env_override = std::env::var_os(TINYVERSE_DIR_HOME_ENV).map(PathBuf::from);

    resolve_with_inputs(ResolveInputs {
        arg_override: arg_override.map(PathBuf::from),
        env_override,
        cwd,
        home,
    })
}

#[derive(Debug, Clone)]
struct ResolveInputs {
    arg_override: Option<PathBuf>,
    env_override: Option<PathBuf>,
    cwd: PathBuf,
    home: PathBuf,
}

fn resolve_with_inputs(inputs: ResolveInputs) -> Result<TinyversePaths> {
    let ResolveInputs {
        arg_override,
        env_override,
        cwd,
        home,
    } = inputs;

    let (home_dir_candidate, source) = if let Some(path) = arg_override {
        (
            normalize_override_path(path, &cwd),
            TinyverseHomeSource::ArgOverride,
        )
    } else if let Some(path) = env_override {
        (
            normalize_override_path(path, &cwd),
            TinyverseHomeSource::EnvOverride,
        )
    } else if let Some(repo_root) = find_tinyverse_repo_root(&cwd) {
        (
            repo_root.join(TINYVERSE_DIR_NAME),
            TinyverseHomeSource::RepoLocal,
        )
    } else if cwd.join(TINYVERSE_DIR_NAME).is_dir() {
        (cwd.join(TINYVERSE_DIR_NAME), TinyverseHomeSource::CwdLocal)
    } else {
        (home.join(TINYVERSE_DIR_NAME), TinyverseHomeSource::UserHome)
    };

    std::fs::create_dir_all(&home_dir_candidate).with_context(|| {
        format!(
            "failed to create tinyverse home directory `{}`",
            home_dir_candidate.display()
        )
    })?;

    let canonical_home = home_dir_candidate
        .canonicalize()
        .with_context(|| format!("failed to canonicalize `{}`", home_dir_candidate.display()))?;

    Ok(TinyversePaths {
        db_path: canonical_home.join(DEFAULT_DB_FILE_NAME),
        home_dir: canonical_home,
        source,
    })
}

fn normalize_override_path(path: PathBuf, cwd: &Path) -> PathBuf {
    let expanded = if let Some(stripped) = path.to_string_lossy().strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            PathBuf::from(home).join(stripped)
        } else {
            path
        }
    } else {
        path
    };

    let absolute = if expanded.is_absolute() {
        expanded
    } else {
        cwd.join(expanded)
    };

    if absolute.file_name().and_then(|value| value.to_str()) == Some(TINYVERSE_DIR_NAME) {
        return absolute;
    }

    let nested = absolute.join(TINYVERSE_DIR_NAME);
    if nested.is_dir() {
        return nested;
    }

    absolute
}

fn find_tinyverse_repo_root(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        let looks_like_repo = ancestor.join("tinyverse_lib").is_dir()
            && ancestor.join("tinyverse_cli").is_dir()
            && ancestor.join("scripts").is_dir()
            && ancestor.join("README.md").is_file();

        if looks_like_repo {
            return Some(ancestor.to_path_buf());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{ResolveInputs, TinyverseHomeSource, resolve_with_inputs};

    fn test_root() -> std::path::PathBuf {
        let base = std::env::temp_dir();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let root = base.join(format!("tinyverse_dir_utils_test_{suffix}"));
        std::fs::create_dir_all(&root).expect("test directory should be created");
        root
    }

    #[test]
    fn prefers_arg_override_first() {
        let root = test_root();
        let override_dir = root.join("override");
        let resolved = resolve_with_inputs(ResolveInputs {
            arg_override: Some(override_dir.clone()),
            env_override: Some(root.join("ignored-env")),
            cwd: root,
            home: std::env::temp_dir(),
        })
        .expect("path resolution should succeed");

        assert_eq!(resolved.source, TinyverseHomeSource::ArgOverride);
        assert_eq!(
            resolved.home_dir,
            override_dir.canonicalize().expect("exists")
        );
    }

    #[test]
    fn uses_cwd_local_when_present() {
        let root = test_root();
        let cwd = root.join("workspace");
        let cwd_home = cwd.join(".tinyverse");
        std::fs::create_dir_all(&cwd_home).expect("cwd .tinyverse should exist");

        let resolved = resolve_with_inputs(ResolveInputs {
            arg_override: None,
            env_override: None,
            cwd,
            home: std::env::temp_dir(),
        })
        .expect("path resolution should succeed");

        assert_eq!(resolved.source, TinyverseHomeSource::CwdLocal);
        assert_eq!(resolved.home_dir, cwd_home.canonicalize().expect("exists"));
    }
}
