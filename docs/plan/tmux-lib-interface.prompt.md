# Build Prompt: Implement `tinyverse_lib::tmux`

Implement the staged plan in `docs/plan/tmux-lib-interface.plan.md`.

## Objective

Create a production-usable, minimal tmux module in `tinyverse_lib` that supports:

- spawn session (two-pane tinyverse default)
- list sessions
- kill session
- capture/view pane output
- send keys/commands to pane

Use a clean Rust interface with typed option/result structs and contextual error handling.

## Constraints

- Keep the API minimal; model only what tinyverse currently needs.
- Prefer `std::process::Command`-based tmux execution for now.
- Do not add `crossterm` for this module.
- Keep naming and style aligned with `STYLE.md` and `AGENTS.md`.
- Avoid speculative abstractions and dead code.

## Required File Targets

- Update: `tinyverse_lib/src/lib.rs`
- Add/update:
  - `tinyverse_lib/src/tmux/mod.rs`
  - `tinyverse_lib/src/tmux/options.rs`
  - `tinyverse_lib/src/tmux/types.rs`
  - `tinyverse_lib/src/tmux/error.rs`
- Add tests in the module files (unit tests required; integration-style tests may be `#[ignore]`).

## Required Behavior

1. Expose a primary client (for example `TmuxClient`) with default `tmux` binary and configurable binary path.
2. Provide typed options/results for each operation.
3. Implement robust subprocess execution with command context on failure.
4. Parse `list-sessions -F` output into typed structs.
5. Implement pane target resolution with this precedence:
   - explicit pane id target,
   - role alias (`console` / `agent`) by pane title,
   - safe fallback for console-first behavior where appropriate.
6. Spawn should create the tinyverse default layout (detached + split panes + titles).

## Acceptance Criteria

- `cargo check -p tinyverse_lib` passes.
- `cargo test -p tinyverse_lib` passes.
- Public API is accessible from `tinyverse_lib` root and ready for future CLI wiring.
- Tests cover parsing and command/target formatting logic.

## Deliverable Notes

When done, provide:

- brief summary of implemented API
- list of touched files
- test commands run and results
- any follow-up gaps (if tmux runtime integration was not fully validated)
