# tinyverse_lib tmux Module Plan

## Goal

Build a focused `tinyverse_lib::tmux` module that implements the tmux operations defined in `README.md` (spawn, list, kill, view, send) with a clean Rust interface and minimal schemas suitable for later `tinyverse_cli` integration.

## Context Snapshot

- Product intent from `README.md`:
  - One tmux session per agent session.
  - Two starting panels (agent TUI + console shell).
  - CLI commands map to tmux lifecycle and panel I/O operations.
- Current repo shape:
  - `tinyverse_lib/src/lib.rs` is still template code.
  - `tinyverse_cli/src/commands/*` are mostly skeleton handlers.
  - Workspace already includes both `tinyverse_cli` and `tinyverse_lib`.
- External reference available:
  - `docs/external/docsrs_tmux-interface/*` provides inspiration for command naming and builder ergonomics.
  - We should keep our API much smaller than that crate's full surface.
- Additional note:
  - `crossterm` is useful for future interactive terminal UX, but not required for core tmux process control in `tinyverse_lib`.

## Design Principles

- Keep the library path minimal: one maintained way to perform each required operation.
- Prefer typed options/outputs over raw stringly APIs.
- Favor explicit `std::process::Command` invocation of `tmux` for now.
- Return contextual errors with enough metadata to debug target/session failures.
- Avoid over-modeling tmux internals; include only fields we need now.

## Proposed Public API (minimal)

- Module layout:
  - `tinyverse_lib/src/lib.rs` -> `pub mod tmux;`
  - `tinyverse_lib/src/tmux/mod.rs`
  - `tinyverse_lib/src/tmux/options.rs`
  - `tinyverse_lib/src/tmux/types.rs`
  - `tinyverse_lib/src/tmux/error.rs`
- Primary entrypoint:
  - `TmuxClient::new()` and `TmuxClient::with_bin(...)`
- Operations:
  - `spawn_session(options) -> Result<SpawnSessionResult, TmuxError>`
  - `list_sessions(options) -> Result<Vec<SessionSummary>, TmuxError>`
  - `kill_session(target) -> Result<(), TmuxError>`
  - `capture_pane(options) -> Result<CapturedPane, TmuxError>`
  - `send_keys(options) -> Result<(), TmuxError>`
- Minimal types:
  - `SessionTarget`, `PaneTarget`, `PanelRole` (`Console`, `Agent`)
  - `SessionSummary` (id, name, attached, windows)
  - `CapturedPane` (target, text)

## Staged Implementation Plan

### Stage 1: Module and Type Scaffolding

Work:
- Replace template `tinyverse_lib/src/lib.rs` with module exports.
- Add `tmux` module files (`mod.rs`, `options.rs`, `types.rs`, `error.rs`).
- Define option/result structs and enums with docs and sane defaults.
- Add compile-only stubs for methods with `todo!` replaced by `Err(TmuxError::...)` placeholders.

Pass criteria:
- `cargo check -p tinyverse_lib` passes.
- Public API compiles and is discoverable from `tinyverse_lib` root.

### Stage 2: Command Runner + Error Handling Core

Work:
- Implement internal command runner around `std::process::Command`.
- Standardize command execution result handling:
  - include command args in error context,
  - include stdout/stderr snippets on failure,
  - detect binary-not-found clearly.
- Add helper(s) for target serialization (`session`, `session:pane`, `%pane`).

Pass criteria:
- Unit tests for command argument assembly and error formatting pass.
- `cargo test -p tinyverse_lib` passes without tmux dependency for unit-only tests.

### Stage 3: List/Kill/View/Send Operations

Work:
- Implement `list_sessions` with `tmux list-sessions -F ...` and parser.
- Implement `kill_session` via `tmux kill-session -t ...`.
- Implement `capture_pane` via `tmux capture-pane -p` with configurable line window.
- Implement `send_keys` via `tmux send-keys` with optional Enter.
- Implement panel alias resolution behavior:
  - explicit pane id wins,
  - role-based lookup (`console`/`agent`) by pane title,
  - fallback to first pane where appropriate.

Pass criteria:
- Parser unit tests cover valid and malformed `list-sessions` lines.
- `cargo test -p tinyverse_lib` passes.
- Optional local smoke (if tmux installed) succeeds for create/list/send/capture/kill flow.

### Stage 4: Spawn Session Flow (two-pane tinyverse default)

Work:
- Implement `spawn_session` with tinyverse defaults:
  - detached session creation,
  - two-pane split,
  - pane titles set to `console` and `agent`,
  - optional initial commands for each pane.
- Return enough identifiers in `SpawnSessionResult` for later CLI use.

Pass criteria:
- Integration-style test (ignored by default) validates session creation and two-pane layout on tmux-enabled environments.
- `cargo test -p tinyverse_lib` passes in normal CI mode.

### Stage 5: Harden + Document for CLI Adoption

Work:
- Add rustdoc examples for each primary API operation.
- Ensure naming and signatures align with expected `tinyverse_cli` command arguments.
- Add an implementation note describing current assumptions (pane title aliasing, fallback rules).

Pass criteria:
- `cargo check --workspace` passes.
- No dead code or unused imports in touched files.
- API is stable enough for CLI command handlers to consume directly.

## Test Strategy

### Unit tests (always-on)

- Argument construction and target formatting.
- Output parsing for list sessions and pane selection helpers.
- Error mapping and context propagation.

### Integration-style tests (best-effort)

- Marked `#[ignore]` so they run only when explicitly requested.
- Create ephemeral tmux session, execute send/capture, then kill.
- Skip gracefully when `tmux` is unavailable.

Suggested commands:

- `cargo check -p tinyverse_lib`
- `cargo test -p tinyverse_lib`
- `cargo test -p tinyverse_lib -- --ignored` (developer local only)

## Definition of Done

- `tinyverse_lib` exposes a compact tmux API with typed options/results for all required README operations.
- Core behavior is covered by deterministic unit tests.
- Optional tmux integration tests validate real-world behavior without forcing CI dependence.
- Implementation is minimal, explicit, and ready for future `tinyverse_cli` wiring.
