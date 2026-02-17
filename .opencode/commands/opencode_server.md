---
description: Inspect or repair TinyVerse managed OpenCode server state
---

Use TinyVerse's managed OpenCode server command group.

Examples:

- Show status: `tinyverse opencode-server status`
- Ensure running: `tinyverse opencode-server ensure`
- Restart service: `tinyverse opencode-server restart`

Useful config keys:

- `opencode.server.enabled`
- `opencode.server.hostname`
- `opencode.server.port`
- `opencode.server.tmux_session_name`

Set config with:

- `tinyverse config set opencode.server.port 4150`
- `tinyverse config set opencode.server.hostname 127.0.0.1`
