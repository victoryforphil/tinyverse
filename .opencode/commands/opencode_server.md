---
description: Inspect or repair TinyVerse managed OpenCode server state
---

Use TinyVerse's managed OpenCode service command group.

Web mode docs: https://opencode.ai/docs/web/

Examples:

- Show status: `tinyverse opencode-server status`
- Ensure running: `tinyverse opencode-server ensure`
- Restart service: `tinyverse opencode-server restart`

Useful config keys:

- `opencode.server.enabled`
- `opencode.server.mode` (`serve` or `web`)
- `opencode.server.hostname`
- `opencode.server.port`
- `opencode.server.tmux_session_name`

Set config with:

- `tinyverse config set opencode.server.port 4150`
- `tinyverse config set opencode.server.hostname 127.0.0.1`
- `tinyverse config set opencode.server.mode web`
- `tinyverse config set opencode.server.hostname 0.0.0.0`
