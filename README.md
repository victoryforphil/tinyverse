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
- `log` + `pretty_env_logger` for logging in Rust