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
- Running `tinyverse` with no subcommand launches `tinyverse tui`.
- Global override for tinyverse home path:
  - `--tinyverse-dir-home <path>`
  - `TINYVERSE_DIR_HOME=<path>`
- Tinyverse path resolution order:
  1. `--tinyverse-dir-home`
  2. `TINYVERSE_DIR_HOME`
  3. repo-local `<repo_root>/.tinyverse` when running in this repo
  4. cwd-local `.tinyverse` if present
  5. `~/.tinyverse`

### Config files

- Active config file: `<tinyverse_home>/config.toml`
- Legacy fallback file (read-only fallback): `<tinyverse_home>/tinyverse.toml`
- When active home is not `~/.tinyverse`, config loading merges in this order:
  1. `~/.tinyverse/config.toml` (or legacy `~/.tinyverse/tinyverse.toml`)
  2. active home config (`<tinyverse_home>/config.toml`, legacy fallback if needed)
- `tinyverse config export` writes the effective config to disk (defaults to `active_path`) and prints metadata comments (`selected_source`, `active_path`, `written_to`, `loaded_from`) so you can verify exactly which file(s) were used.

## Testing

- Preferred Rust test runner: `cargo nextest run`
- Fallback when `cargo-nextest` is unavailable or fails in this environment: `cargo test`

## Logging

- Tracing logs write to both stdout and a file.
- Log files are created under `<tinyverse_home>/logs/`.
- File name format: `tinyverse-<unix_millis>.log`.
- `tinyverse_home` follows the same resolution order documented in `CLI / Commands`.

## Moon

- Moon workspace config lives in `.moon/workspace.yml` and `.moon/toolchains.yml`.
- Rust project configs live in `tinyverse_lib/moon.yml` and `tinyverse_cli/moon.yml`.
- Example commands:
  - `moon :install`
  - `moon run tinyverse_lib:check`
  - `moon run tinyverse_cli:test`

## Scripts

- `bun scripts/build.sh.ts` // run `moon :build`
- `bun scripts/check.sh.ts` // run `moon :check`
- `bun scripts/test.sh.ts` // run `moon :test`
- `bun scripts/dev.sh.ts` // run default watch loop for `tinyverse_cli`
- `bun scripts/ci.sh.ts [mode]` // local CI orchestration (same entrypoints used by GitHub Actions)
- `bun scripts/docker_build.sh.ts [targets...]` // docker compose build wrapper
- `bun scripts/docker_ci.sh.ts [run --] [command...]` // run CI command inside `ci` container
- `bun scripts/docker_publish.sh.ts --image-repo <repo> --tag <tag> [--push]`

### Commands

- `list` // List tmux sessions known to tinyverse (defaults to tinyverse-only sessions).
  - Source of truth is the local tinyverse SQLite session DB.
  - Reconciles DB sessions against tmux before reads (debounced/rate-limited).
  - `--all` includes unmanaged tmux sessions in addition to DB sessions.
  - `--format={table|text|json|toml|yaml}` (default: `table`)
- `spawn` // Create a new tinyverse session (console + agent panes).
  - Default pane layout: `agent` on left, `console` on right.
  - Initial tmux window size and split behavior are configurable via `tmux.*` config keys.
  - `--agent={opencode}` (defaults from `spawn.default_agent` config)
  - `--prompt={file_or_string}`
  - OpenCode receives a default TinyVerse context prompt template from `tinyverse_cli/prompts/opencode_default_context.md`.
  - When `--prompt` is provided, TinyVerse appends it as a `Task Request` section to that default OpenCode context.
  - `--model={model}` (defaults from `spawn.default_model` config when set)
  - `--agent_args={string}` (supports `{prompt}` placeholder)
  - `--clean-shell` (starts panes with `zsh -f`, ignores user `~/.zshrc`)
  - `--no-clean-shell` (forces default shell startup behavior)
  - Working directory defaults to `workspace.default_dir` when configured, otherwise current directory.
- `prompt render` // Render effective launch prompt for an agent.
  - `--agent={opencode}` (default: `opencode`)
  - `--prompt={file_or_string}`
- `config print` // Print effective config.
  - `--output={full|raw}` (default: `full`; `raw` prints config data only)
  - `--format={table|text|json|toml|yaml}` (default: `table`)
- `config export` // Export effective config as TOML (with source/path metadata comments).
  - Writes to resolved `active_path` by default.
  - `--path <config_file>` writes to that explicit output file path.
- `config set <key> <value>` // Persist a config value in `<tinyverse_home>/config.toml`.
  - Supported keys:
    - `shell.clean` (`true|false`)
    - `workspace.default_dir` (path string; use `none` to clear)
    - `git.branch_prefix` (string)
    - `spawn.default_agent` (string)
    - `spawn.default_model` (string; use `none` to clear)
    - `tmux.initial_window_width` (integer)
    - `tmux.initial_window_height` (integer)
    - `tmux.layout.direction` (`horizontal|vertical`)
    - `tmux.layout.primary` (`agent|console`)
    - `tmux.layout.secondary_percent` (`1-99`)
- `attach <session>` // Attach to an existing session by key or name.
  - Session lookup first tries an exact match.
  - If exact match is not found, tinyverse also tries `tinyverse_<session>`.
    - Example: `attach redding` resolves `tinyverse_redding` when present.
  - If `<session>` is omitted in an interactive terminal, tinyverse opens a TUI selector.
- `detach` // Detach current tmux client without closing the session.
- `kill <session>` // Kill session by key or name.
  - If `<session>` is omitted in an interactive terminal, tinyverse opens a TUI selector.
- `view` // Capture pane output.
  - `--session={key_or_name}` (optional inside tmux, required outside tmux)
  - `--panel={console|agent|%pane_id}`
  - `--output={full|raw}` (default: `full`; `raw` prints pane buffer text only)
  - `--export=<path>` (writes rendered output to file; adds `.md` when extension is missing)
- `send <command>` // Send keys to pane.
  - `--session={key_or_name}` (optional inside tmux, required outside tmux)
  - `--panel={console|agent|%pane_id}`
- `debug self` // Inspect current tmux context.
  - `--format={table|text|json|toml|yaml}`
- `debug reset-db` // Backup and reset local tinyverse session DB.

### Quick Copy/Paste Examples

```bash
# Spawn a new session with default agent
# (defaults key to an available California city, e.g. tinyverse_san_bernardino)
cargo run -p tinyverse_cli -- spawn

# Spawn with a prompt string
cargo run -p tinyverse_cli -- spawn --prompt "you are a helpful coding agent"

# Preview full rendered launch prompt
cargo run -p tinyverse_cli -- prompt render --agent opencode --prompt "triage failing tests"

# Spawn with clean zsh (no ~/.zshrc)
cargo run -p tinyverse_cli -- spawn --clean-shell

# Persist clean shell as the default for future spawns
cargo run -p tinyverse_cli -- config set shell.clean true

# Set default spawn and workspace values
cargo run -p tinyverse_cli -- config set spawn.default_agent opencode
cargo run -p tinyverse_cli -- config set spawn.default_model gpt-5.3-codex
cargo run -p tinyverse_cli -- config set workspace.default_dir ~/repos/vfp/tinyverse

# Tune default tmux spawn layout
cargo run -p tinyverse_cli -- config set tmux.initial_window_width 220
cargo run -p tinyverse_cli -- config set tmux.initial_window_height 64
cargo run -p tinyverse_cli -- config set tmux.layout.direction horizontal
cargo run -p tinyverse_cli -- config set tmux.layout.primary agent
cargo run -p tinyverse_cli -- config set tmux.layout.secondary_percent 35

# Check effective config and export as TOML
cargo run -p tinyverse_cli -- config print
cargo run -p tinyverse_cli -- config export

# Config print as machine formats
cargo run -p tinyverse_cli -- config print --format yaml
cargo run -p tinyverse_cli -- config print --output raw --format toml

# Export a specific config file
cargo run -p tinyverse_cli -- config export --path ./.tinyverse/config.toml

# Spawn with an explicit key/name
cargo run -p tinyverse_cli -- spawn --key my-session

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

# Attach and choose from interactive TUI selector
cargo run -p tinyverse_cli -- attach

# Attach using implicit tinyverse_ prefix fallback
cargo run -p tinyverse_cli -- attach redding

# Detach from current tmux client (session keeps running)
cargo run -p tinyverse_cli -- detach

# Send a command to a specific session console pane
cargo run -p tinyverse_cli -- send "pwd" --session tinyverse_123 --panel console

# View latest console output for a session
cargo run -p tinyverse_cli -- view --session tinyverse_123 --panel console

# View raw pane buffer only (agent-friendly)
cargo run -p tinyverse_cli -- view --session tinyverse_123 --panel console --output raw

# Export captured output to markdown
cargo run -p tinyverse_cli -- view --session tinyverse_123 --panel console --export ./captures/redding

# Debug current context as text/json
cargo run -p tinyverse_cli -- debug self
cargo run -p tinyverse_cli -- debug self --format json

# Debug current context as YAML/TOML
cargo run -p tinyverse_cli -- debug self --format yaml
cargo run -p tinyverse_cli -- debug self --format toml

# Backup and reset session DB
cargo run -p tinyverse_cli -- debug reset-db

# Kill a session
cargo run -p tinyverse_cli -- kill tinyverse_123

# Kill and choose from interactive TUI selector
cargo run -p tinyverse_cli -- kill
```

### Example outputs

```text
$ cargo run -p tinyverse_cli -- config print --output full --format text
selected_source: repo_local
selected_home: /Users/alex/repos/vfp/tinyverse/.tinyverse
active_path: /Users/alex/repos/vfp/tinyverse/.tinyverse/config.toml
legacy_path: /Users/alex/repos/vfp/tinyverse/.tinyverse/tinyverse.toml
loaded_from: /Users/alex/repos/vfp/tinyverse/.tinyverse/config.toml

shell.clean: false
workspace.default_dir: <unset>
git.branch_prefix: tv/
spawn.default_agent: opencode
spawn.default_model: <unset>
tmux.initial_window_width: 220
tmux.initial_window_height: 64
tmux.layout.direction: horizontal
tmux.layout.primary: agent
tmux.layout.secondary_percent: 35
```

```yaml
$ cargo run -p tinyverse_cli -- config print --output raw --format yaml
shell:
  clean: false
workspace:
  default_dir: null
git:
  branch_prefix: tv/
spawn:
  default_agent: opencode
  default_model: null
tmux:
  initial_window_width: 220
  initial_window_height: 64
  layout:
    direction: horizontal
    primary: agent
    secondary_percent: 35
```
