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


## CLI / Commands

- `tinyverse` // Root command / binary 
    - `providers` // List available agent providers (OpenCode, Codex, etc.) and their status (installed, needs config, etc.) + config options
    - `config` // View and edit tinyverse config (like default provider, session settings, etc.)
        - `export` // Export current config as TOML (will start as all default values populated)
        - `print` // Print current config values in a human readable format
    - `path` // Show the `tinyverse` dot-file directory path for storing config, session data, logs, etc. (~/.tinyverse/ but realpath'd)
    - `list` // List all tinyverse sessions, with info on their status, start time, etc. (tmux session list + our metadata)
    - `spawn` // Spawn a new tinyverse session with a given agent (OpenCode, Codex, etc.) and optional config (like starting prompt, tools to enable, etc.)
       - `--agent={opencode}` // Which agent to spawn in the session (future: codex, dark_chat)
       - `--prompt={file_or_string}` // Optional starting prompt for the agent, can be a direct string or a file path to read the prompt from
            - Tool information on CLI use is automatically appeneded and thus is the default value if this additional field is left blank.
       - `--agent_args={json}` // Optional JSON string of additional args to pass to the agent on startup, like tool config, model config, etc. Can use {prompt} as a placeholder for the actual prompt after reading from file or arg which is set to the prompt string.
    - `kill` // Kill a tinyverse session by ID or name (tmux kill-session)
    - `view` // Get the current text buffer of the console panel in a session
        - `--panel={console|agent|(panel_id)}` // Which panel to view, defaults to console when theres just one (or first one)
        - `--session={id}` // Which session to view the console of, defaults to the current session if inside said tmux session (called by the agent itself)
    - `send {command}` // Send a command to the console panel of a session
        - `--session={id}` // Which session to send the command to, defaults to the current session if inside said tmux session (called by the agent itself)
        - `--panel={console|agent|(panel_id)}` // Which panel to send the command to, defaults to console when theres just one (or first one)