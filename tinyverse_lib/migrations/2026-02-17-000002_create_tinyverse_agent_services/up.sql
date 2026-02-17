CREATE TABLE tinyverse_agent_services (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  provider_key TEXT NOT NULL UNIQUE,
  tmux_session_name TEXT NOT NULL,
  tmux_pane_id TEXT,
  hostname TEXT NOT NULL,
  port INTEGER NOT NULL,
  base_url TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tinyverse_agent_services_provider_key
  ON tinyverse_agent_services(provider_key);
