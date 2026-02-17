use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelRole {
    Console,
    Agent,
}

impl PanelRole {
    pub fn as_title(self) -> &'static str {
        match self {
            Self::Console => "console",
            Self::Agent => "agent",
        }
    }
}

impl fmt::Display for PanelRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_title())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaneTarget {
    PaneId(String),
    Role(PanelRole),
}

impl PaneTarget {
    pub fn from_selector(selector: &str) -> Self {
        match selector.trim().to_ascii_lowercase().as_str() {
            "console" => Self::Role(PanelRole::Console),
            "agent" => Self::Role(PanelRole::Agent),
            _ => Self::PaneId(selector.trim().to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionTarget(String);

impl SessionTarget {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SessionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for SessionTarget {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SessionTarget {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSummary {
    pub session_id: String,
    pub session_name: String,
    pub attached_clients: u32,
    pub windows: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnSessionResult {
    pub session: SessionTarget,
    pub console_pane_id: String,
    pub agent_pane_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedPane {
    pub session: SessionTarget,
    pub pane_id: String,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::{PaneTarget, PanelRole};

    #[test]
    fn selector_maps_role_aliases() {
        assert_eq!(
            PaneTarget::from_selector("console"),
            PaneTarget::Role(PanelRole::Console)
        );
        assert_eq!(
            PaneTarget::from_selector("AGENT"),
            PaneTarget::Role(PanelRole::Agent)
        );
    }

    #[test]
    fn selector_keeps_explicit_pane_ids() {
        assert_eq!(
            PaneTarget::from_selector("%12"),
            PaneTarget::PaneId("%12".to_owned())
        );
    }
}
