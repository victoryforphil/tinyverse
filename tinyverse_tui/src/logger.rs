use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use tinyverse_lib::resolve_tinyverse_paths;

static LOG_FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
static LOG_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

pub(crate) fn init_logger() {
    let _ = log_path();
    log_line("tinyverse_tui logger initialized");
}

pub(crate) fn log_line(message: &str) {
    let Some(lock) = log_file().as_ref() else {
        return;
    };
    if let Ok(mut file) = lock.lock() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let _ = writeln!(file, "[{timestamp}] {message}");
    }
}

pub(crate) fn log_path() -> Option<PathBuf> {
    LOG_PATH.get_or_init(resolve_log_path).clone()
}

fn log_file() -> &'static Option<Mutex<File>> {
    LOG_FILE.get_or_init(|| {
        let path = log_path()?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()?;
        Some(Mutex::new(file))
    })
}

fn resolve_log_path() -> Option<PathBuf> {
    if let Ok(override_path) = std::env::var("TINYVERSE_TUI_LOG") {
        let trimmed = override_path.trim();
        if !trimmed.is_empty() {
            let path = PathBuf::from(trimmed);
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            return Some(path);
        }
    }

    if let Ok(paths) = resolve_tinyverse_paths(None) {
        return Some(paths.home_dir.join("tui.log"));
    }

    Some(std::env::temp_dir().join("tinyverse_tui.log"))
}
