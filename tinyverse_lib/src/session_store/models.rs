use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use super::schema::tinyverse_sessions;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = tinyverse_sessions)]
pub struct SessionRecord {
    pub id: i32,
    pub session_key: String,
    pub session_name: String,
    pub agent_type: String,
    pub description: Option<String>,
    pub status_string: String,
    pub tmux_session_name: String,
    pub tmux_session_id: Option<String>,
    pub console_pane_id: Option<String>,
    pub agent_pane_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_message_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = tinyverse_sessions)]
pub struct NewSessionRecord<'a> {
    pub session_key: &'a str,
    pub session_name: &'a str,
    pub agent_type: &'a str,
    pub description: Option<&'a str>,
    pub status_string: &'a str,
    pub tmux_session_name: &'a str,
    pub tmux_session_id: Option<&'a str>,
    pub console_pane_id: Option<&'a str>,
    pub agent_pane_id: Option<&'a str>,
}
