mod models;
pub mod schema;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::{info, warn};

use crate::dir_utils::{TinyversePaths, resolve_tinyverse_paths};
use crate::tmux::{ListSessionsOptions, TmuxClient};

use self::models::{NewSessionRecord, SessionRecord};
use self::schema::tinyverse_sessions;

pub use self::models::SessionRecord as StoredSession;

pub const STATUS_ACTIVE: &str = "active";
pub const DEFAULT_RECONCILE_MIN_INTERVAL: Duration = Duration::from_secs(2);
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Clone)]
pub struct CreateSessionInput {
    pub session_name: String,
    pub agent_type: String,
    pub description: Option<String>,
    pub tmux_session_name: String,
    pub tmux_session_id: Option<String>,
    pub console_pane_id: Option<String>,
    pub agent_pane_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResetDbReport {
    pub backup_path: Option<PathBuf>,
    pub database_path: PathBuf,
}

pub struct SessionStore {
    conn: SqliteConnection,
    paths: TinyversePaths,
    tmux_client: TmuxClient,
    reconcile_gate: ReconcileGate,
}

#[derive(Debug, Clone)]
struct ReconcileGate {
    last_run_at: Option<Instant>,
    min_interval: Duration,
}

impl SessionStore {
    pub fn open_default() -> Result<Self> {
        let paths = resolve_tinyverse_paths(None)?;
        Self::open(paths)
    }

    pub fn open(paths: TinyversePaths) -> Result<Self> {
        let mut conn = open_connection(&paths.db_path)?;
        run_pending_migrations(&mut conn)?;

        Ok(Self {
            conn,
            paths,
            tmux_client: TmuxClient::new(),
            reconcile_gate: ReconcileGate {
                last_run_at: None,
                min_interval: DEFAULT_RECONCILE_MIN_INTERVAL,
            },
        })
    }

    pub fn paths(&self) -> &TinyversePaths {
        &self.paths
    }

    pub fn set_reconcile_min_interval(&mut self, min_interval: Duration) {
        self.reconcile_gate.min_interval = min_interval;
    }

    pub fn reconcile_now(&mut self) -> Result<()> {
        self.reconcile_internal(true)
    }

    pub fn list_sessions(&mut self) -> Result<Vec<SessionRecord>> {
        self.reconcile_internal(false)?;

        tinyverse_sessions::table
            .select(SessionRecord::as_select())
            .order_by(tinyverse_sessions::created_at.desc())
            .load::<SessionRecord>(&mut self.conn)
            .context("failed to list sessions from database")
    }

    pub fn find_session(&mut self, query: &str) -> Result<Option<SessionRecord>> {
        self.reconcile_internal(false)?;

        let normalized = query.trim();
        if normalized.is_empty() {
            return Ok(None);
        }

        tinyverse_sessions::table
            .filter(
                tinyverse_sessions::session_key
                    .eq(normalized)
                    .or(tinyverse_sessions::session_name.eq(normalized))
                    .or(tinyverse_sessions::tmux_session_name.eq(normalized)),
            )
            .select(SessionRecord::as_select())
            .first::<SessionRecord>(&mut self.conn)
            .optional()
            .with_context(|| format!("failed to find session `{normalized}`"))
    }

    pub fn create_session(&mut self, input: &CreateSessionInput) -> Result<SessionRecord> {
        let session_key = self.next_available_session_key(&input.session_name)?;

        let record = NewSessionRecord {
            session_key: &session_key,
            session_name: &input.session_name,
            agent_type: &input.agent_type,
            description: input.description.as_deref(),
            status_string: STATUS_ACTIVE,
            tmux_session_name: &input.tmux_session_name,
            tmux_session_id: input.tmux_session_id.as_deref(),
            console_pane_id: input.console_pane_id.as_deref(),
            agent_pane_id: input.agent_pane_id.as_deref(),
        };

        diesel::insert_into(tinyverse_sessions::table)
            .values(&record)
            .execute(&mut self.conn)
            .with_context(|| {
                format!(
                    "failed to insert session `{}` with key `{}`",
                    input.session_name, session_key
                )
            })?;

        tinyverse_sessions::table
            .filter(tinyverse_sessions::session_key.eq(&session_key))
            .select(SessionRecord::as_select())
            .first::<SessionRecord>(&mut self.conn)
            .optional()
            .with_context(|| format!("failed to load inserted session `{session_key}`"))?
            .with_context(|| format!("session `{session_key}` was inserted but not found"))
    }

    pub fn delete_session_by_key(&mut self, session_key: &str) -> Result<bool> {
        let deleted = diesel::delete(
            tinyverse_sessions::table.filter(tinyverse_sessions::session_key.eq(session_key)),
        )
        .execute(&mut self.conn)
        .with_context(|| format!("failed to delete session `{session_key}`"))?;

        Ok(deleted > 0)
    }

    fn next_available_session_key(&mut self, session_name: &str) -> Result<String> {
        let base = sanitize_session_key(session_name);
        let mut candidate = base.clone();
        let mut suffix = 2;

        while self.session_key_exists(&candidate)? {
            candidate = format!("{base}-{suffix}");
            suffix += 1;
        }

        Ok(candidate)
    }

    fn session_key_exists(&mut self, session_key: &str) -> Result<bool> {
        let existing_count: i64 = tinyverse_sessions::table
            .filter(tinyverse_sessions::session_key.eq(session_key))
            .count()
            .get_result(&mut self.conn)
            .with_context(|| format!("failed to check session key `{session_key}`"))?;

        Ok(existing_count > 0)
    }

    fn reconcile_internal(&mut self, force: bool) -> Result<()> {
        if !should_run_reconcile(
            self.reconcile_gate.last_run_at,
            self.reconcile_gate.min_interval,
            force,
        ) {
            return Ok(());
        }

        let tmux_sessions = match self.tmux_client.list_sessions(ListSessionsOptions) {
            Ok(sessions) => sessions,
            Err(error) => {
                warn!("Skipping session reconcile; failed to read tmux sessions ({error})");
                self.reconcile_gate.last_run_at = Some(Instant::now());
                return Ok(());
            }
        };

        let live_names: HashSet<String> = tmux_sessions
            .into_iter()
            .map(|session| session.session_name)
            .collect();

        let db_rows = tinyverse_sessions::table
            .select((
                tinyverse_sessions::session_key,
                tinyverse_sessions::session_name,
                tinyverse_sessions::tmux_session_name,
            ))
            .load::<(String, String, String)>(&mut self.conn)
            .context("failed to read sessions for reconcile")?;

        let mut deleted_count = 0usize;
        for (session_key, session_name, tmux_session_name) in db_rows {
            if live_names.contains(&tmux_session_name) {
                continue;
            }

            let removed = diesel::delete(
                tinyverse_sessions::table.filter(tinyverse_sessions::session_key.eq(&session_key)),
            )
            .execute(&mut self.conn)
            .with_context(|| {
                format!("failed to delete stale session `{session_key}` during reconcile")
            })?;

            if removed > 0 {
                deleted_count += 1;
                warn!(
                    "Deleted stale session (key={}, name={}, tmux={})",
                    session_key, session_name, tmux_session_name
                );
            }
        }

        if deleted_count > 0 {
            info!("Reconcile removed {deleted_count} stale session(s)");
        }

        self.reconcile_gate.last_run_at = Some(Instant::now());
        Ok(())
    }
}

pub fn run_pending_migrations(conn: &mut SqliteConnection) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|error| anyhow::anyhow!("failed to run session database migrations: {error}"))
}

pub fn reset_db_with_backup(paths: &TinyversePaths) -> Result<ResetDbReport> {
    let db_path = paths.db_path.clone();
    let mut backup_path = None;

    if db_path.is_file() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup = paths
            .home_dir
            .join(format!("tinyverse_sessions.backup.{timestamp}.sqlite3"));

        std::fs::copy(&db_path, &backup).with_context(|| {
            format!(
                "failed to backup database from `{}` to `{}`",
                db_path.display(),
                backup.display()
            )
        })?;

        backup_path = Some(backup);
        std::fs::remove_file(&db_path)
            .with_context(|| format!("failed to remove database `{}`", db_path.display()))?;
    }

    let mut conn = open_connection(&db_path)?;
    run_pending_migrations(&mut conn)?;

    Ok(ResetDbReport {
        backup_path,
        database_path: db_path,
    })
}

fn open_connection(db_path: &Path) -> Result<SqliteConnection> {
    let db_url = db_path.to_string_lossy().to_string();
    SqliteConnection::establish(&db_url)
        .with_context(|| format!("failed to open sqlite database `{}`", db_path.display()))
}

fn should_run_reconcile(last_run_at: Option<Instant>, min_interval: Duration, force: bool) -> bool {
    if force {
        return true;
    }

    let Some(last_run_at) = last_run_at else {
        return true;
    };

    last_run_at.elapsed() >= min_interval
}

pub fn sanitize_session_key(value: &str) -> String {
    let lowered = value.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return "session".to_owned();
    }

    let mut output = String::with_capacity(lowered.len());
    let mut previous_dash = false;

    for character in lowered.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            output.push(character);
            previous_dash = false;
            continue;
        }

        if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }

    let trimmed = output.trim_matches('-');
    if trimmed.is_empty() {
        "session".to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{sanitize_session_key, should_run_reconcile};

    #[test]
    fn sanitize_session_key_normalizes_and_collapses_symbols() {
        let result = sanitize_session_key("  Hello, Tinyverse Session!!! ");
        assert_eq!(result, "hello-tinyverse-session");
    }

    #[test]
    fn debounce_skips_reconcile_within_interval() {
        let now = std::time::Instant::now();
        let should_run = should_run_reconcile(Some(now), Duration::from_secs(2), false);
        assert!(!should_run);
    }

    #[test]
    fn debounce_allows_reconcile_after_interval() {
        let past = std::time::Instant::now() - Duration::from_secs(3);
        let should_run = should_run_reconcile(Some(past), Duration::from_secs(2), false);
        assert!(should_run);
    }

    #[test]
    fn force_reconcile_ignores_debounce_window() {
        let now = std::time::Instant::now();
        let should_run = should_run_reconcile(Some(now), Duration::from_secs(10), true);
        assert!(should_run);
    }
}
