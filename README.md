# tinyverse

`tmux`-based agent + shell "harness" and tools for monitoring and controlling agents.

## Notes

- Idea is to spawn a tmux session per agent session, with starting off two panels:
  - One for the agent TUI (such as OpenCode or Codex)
  - One for a shell, which the agent can use to run commands and scripts. The shell panel will be the "main" panel that the agent interacts with, and the TUI panel will be for visualizing the agent's thought process and actions.
  - Agent can spawn more shells if needed
  - Additional: we can use a oh-my-opencode like plugin to let the agent spawn background agents as opencode panels and such to extend this concept
- This allows the user to easy tab into sessions and manage them
- Allows the agent to (optionall but recommended to) to use tmux buffer / send-keys to interace with the shell vs normal command running or pty.
- The agent can use an MCP or call the CLI directly (knowing its tmux session info) to control tmux or read/write to the shell in place of its existing tools.
- Host CLI will be in charge of tracking (sqlite?) session info, and providing a CLI for users to manage sessions, view logs, etc. It will also be responsible for spawning tmux sessions and panels as needed.
- Host CLI will have basic abstractions for different agent tools. Mostly involves just defining metadata / command to invoke and what to pass prompt info in the args

## Tech Stack / Components

- `tvcli` - the main CLI for managing sessions, spawning agents, etc.
- `tmux` - for session and panel management
- `rust` + `clap` for CLI
- If we need local session storage, use `sqlite` with `rusqlite` crate and some ODM abstraction (what ever is easiest to work with, maybe `sea-orm` or `diesel`)
- Exposed REST and MCP APIs for agent interaction and control, using `warp` or `axum` for the server implementation
- `thiserror` / `anyhow` for error handling in Rust
- `log` + `tracing` + `tracing-subscriber` for logging in Rust


## CLI / Commands

- Root command: `tinyverse`
- In this repo, run commands via: `cargo run -p tinyverse_cli -- <command>`
- Global override for tinyverse home path:
  - `--tinyverse-dir-home <path>`
  - `TINYVERSE_DIR_HOME=<path>`
- Tinyverse path resolution order:
  1. `--tinyverse-dir-home`
  2. `TINYVERSE_DIR_HOME`
  3. repo-local `<repo_root>/.tinyverse` when running in this repo
  4. cwd-local `.tinyverse` if present
  5. `~/.tinyverse`

## Testing

- Preferred Rust test runner: `cargo nextest run`
- Fallback when `cargo-nextest` is unavailable or fails in this environment: `cargo test`

## Moon

- Moon workspace config lives in `.moon/workspace.yml` and `.moon/toolchains.yml`.
- Rust project configs live in `tinyverse_lib/moon.yml` and `tinyverse_cli/moon.yml`.
- Example commands:
  - `moon :install`
  - `moon run tinyverse_lib:check`
  - `moon run tinyverse_cli:test`

### Commands

- `list` // List tmux sessions known to tinyverse (defaults to tinyverse-only sessions).
  - Source of truth is the local tinyverse SQLite session DB.
  - Reconciles DB sessions against tmux before reads (debounced/rate-limited).
  - `--all` includes unmanaged tmux sessions in addition to DB sessions.
  - `--format={table|text|json}` (default: `table`)
- `spawn` // Create a new tinyverse session (console + agent panes).
  - `--agent={opencode}`
  - `--prompt={file_or_string}`
  - `--agent_args={string}` (supports `{prompt}` placeholder)
- `attach <session>` // Attach to an existing session by key or name.
- `kill <session>` // Kill session by key or name.
- `view` // Capture pane output.
  - `--session={key_or_name}` (optional inside tmux, required outside tmux)
  - `--panel={console|agent|%pane_id}`
- `send <command>` // Send keys to pane.
  - `--session={key_or_name}` (optional inside tmux, required outside tmux)
  - `--panel={console|agent|%pane_id}`
- `debug self` // Inspect current tmux context.
  - `--format={table|text|json}`
- `debug reset-db` // Backup and reset local tinyverse session DB.

### Quick Copy/Paste Examples

```bash
# Spawn a new session with default agent
cargo run -p tinyverse_cli -- spawn

# Spawn with a prompt string
cargo run -p tinyverse_cli -- spawn --prompt "you are a helpful coding agent"

# List tinyverse sessions only (default)
cargo run -p tinyverse_cli -- list

# List all tmux sessions
cargo run -p tinyverse_cli -- list --all

# Override tinyverse home path for this invocation
cargo run -p tinyverse_cli -- --tinyverse-dir-home ./.tinyverse list

# List as JSON (for scripting)
cargo run -p tinyverse_cli -- list --format json

# Attach to a session
cargo run -p tinyverse_cli -- attach tinyverse_123

# Send a command to a specific session console pane
cargo run -p tinyverse_cli -- send "pwd" --session tinyverse_123 --panel console

# View latest console output for a session
cargo run -p tinyverse_cli -- view --session tinyverse_123 --panel console

# Debug current context as text/json
cargo run -p tinyverse_cli -- debug self
cargo run -p tinyverse_cli -- debug self --format json

# Backup and reset session DB
cargo run -p tinyverse_cli -- debug reset-db

# Kill a session
cargo run -p tinyverse_cli -- kill tinyverse_123
```
