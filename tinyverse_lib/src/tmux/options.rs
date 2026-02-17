use std::path::PathBuf;

use super::types::{PaneTarget, SessionTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnSessionOptions {
    pub session_name: String,
    pub working_dir: Option<PathBuf>,
    pub pane_shell_command: Option<String>,
    pub console_command: Option<String>,
    pub agent_command: Option<String>,
}

impl SpawnSessionOptions {
    pub fn new(session_name: impl Into<String>) -> Self {
        Self {
            session_name: session_name.into(),
            working_dir: None,
            pane_shell_command: None,
            console_command: None,
            agent_command: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ListSessionsOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePaneOptions {
    pub session: SessionTarget,
    pub pane: Option<PaneTarget>,
    pub start_line: Option<i32>,
    pub end_line: Option<i32>,
}

impl CapturePaneOptions {
    pub fn new(session: impl Into<SessionTarget>) -> Self {
        Self {
            session: session.into(),
            pane: None,
            start_line: None,
            end_line: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendKeysOptions {
    pub session: SessionTarget,
    pub pane: Option<PaneTarget>,
    pub command: String,
    pub press_enter: bool,
}

impl SendKeysOptions {
    pub fn new(session: impl Into<SessionTarget>, command: impl Into<String>) -> Self {
        Self {
            session: session.into(),
            pane: None,
            command: command.into(),
            press_enter: true,
        }
    }
}
