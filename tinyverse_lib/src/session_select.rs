use anyhow::{Result, bail};
use tinyverse_ui::format_display_name;

use crate::{
    ArgSelectOption, RequiredArgSelectConfig, SessionStore, StoredSession, select_required_arg,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredSessionSelectConfig {
    pub picker_title: String,
    pub cli_example: String,
    pub cancelled_message: String,
    pub empty_message: String,
}

impl RequiredSessionSelectConfig {
    pub fn new(picker_title: impl Into<String>, cli_example: impl Into<String>) -> Self {
        Self {
            picker_title: picker_title.into(),
            cli_example: cli_example.into(),
            cancelled_message: "session selection cancelled".to_owned(),
            empty_message: "no sessions available".to_owned(),
        }
    }

    pub fn with_cancelled_message(mut self, message: impl Into<String>) -> Self {
        self.cancelled_message = message.into();
        self
    }

    pub fn with_empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }
}

pub fn resolve_required_session_key(
    explicit_session: Option<&str>,
    store: &mut SessionStore,
    config: RequiredSessionSelectConfig,
) -> Result<String> {
    if let Some(session) = explicit_session {
        let trimmed = session.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_owned());
        }
    }

    let sessions = store.list_sessions()?;
    if sessions.is_empty() {
        bail!(config.empty_message);
    }

    let options = build_session_options(&sessions);
    select_required_arg(
        RequiredArgSelectConfig::new("session", config.picker_title, config.cli_example)
            .with_cancelled_message(config.cancelled_message),
        options,
    )
}

fn build_session_options(sessions: &[StoredSession]) -> Vec<ArgSelectOption> {
    sessions
        .iter()
        .map(|session| {
            ArgSelectOption::new(
                format_display_name(&session.session_name),
                session.session_key.clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::build_session_options;
    use crate::StoredSession;

    fn sample_session(name: &str, key: &str) -> StoredSession {
        let epoch = DateTime::UNIX_EPOCH.naive_utc();
        StoredSession {
            id: 1,
            session_key: key.to_owned(),
            session_name: name.to_owned(),
            agent_type: "opencode".to_owned(),
            description: None,
            status_string: "active".to_owned(),
            tmux_session_name: name.to_owned(),
            tmux_session_id: None,
            console_pane_id: None,
            agent_pane_id: None,
            agent_base_url: None,
            agent_session_id: None,
            created_at: epoch,
            last_message_at: None,
            updated_at: epoch,
        }
    }

    #[test]
    fn build_session_options_formats_display_labels() {
        let options = build_session_options(&[sample_session("tinyverse_redding", "redding")]);
        assert_eq!(options.len(), 1);
        assert_eq!(options[0].label, "Redding do TinyVerse // Redding");
        assert_eq!(options[0].value, "redding");
    }

    #[test]
    fn build_session_options_preserves_order() {
        let options = build_session_options(&[
            sample_session("tinyverse_oakland", "oakland"),
            sample_session("tinyverse_redding", "redding"),
        ]);
        assert_eq!(options[0].value, "oakland");
        assert_eq!(options[1].value, "redding");
    }
}
