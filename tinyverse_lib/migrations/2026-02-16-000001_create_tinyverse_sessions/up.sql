CREATE TABLE tinyverse_sessions (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  session_key TEXT NOT NULL UNIQUE,
  session_name TEXT NOT NULL,
  agent_type TEXT NOT NULL,
  description TEXT,
  status_string TEXT NOT NULL,
  tmux_session_name TEXT NOT NULL,
  tmux_session_id TEXT,
  console_pane_id TEXT,
  agent_pane_id TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_message_at TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tinyverse_sessions_status
  ON tinyverse_sessions(status_string);

CREATE INDEX idx_tinyverse_sessions_last_message
  ON tinyverse_sessions(last_message_at);

CREATE INDEX idx_tinyverse_sessions_tmux_name
  ON tinyverse_sessions(tmux_session_name);
