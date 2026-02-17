# Build Prompt: tinyverse 2D TUI

Implement Stage 1 and Stage 2 from `docs/plan/tui-2d-session-view/02-implementation-plan.plan.md`.

## Objective

Add a new `tinyverse tui` command and a `tinyverse_tui` crate that renders a keyboard-first 2D session card surface backed by `SessionStore` data.

## Required Outcomes

1. `tinyverse tui` command exists and is wired into CLI dispatch.
2. Terminal runtime enters alternate screen, handles events, restores terminal on exit.
3. Sessions render as a simple card grid (selected card highlighted).
4. Keyboard support:
   - quit: `q`, `Esc`
   - move selection: arrows + `j/k/h/l`
   - refresh: `r`
5. Manual and timer-based data refresh from `SessionStore`.
6. Code compiles workspace-wide.

## Constraints

- Keep diffs focused.
- Do not modify existing command behavior besides adding `tui`.
- Keep initial TUI implementation synchronous and explicit.
- Keep style consistent with `AGENTS.md` and `STYLE.md`.

## Suggested File Targets

- Workspace:
  - `Cargo.toml` (add `tinyverse_tui` member)
- CLI:
  - `tinyverse_cli/Cargo.toml` (add `tinyverse_tui` dependency)
  - `tinyverse_cli/src/commands/mod.rs`
  - `tinyverse_cli/src/root.rs`
  - `tinyverse_cli/src/run.rs`
  - `tinyverse_cli/src/commands/tui/args.rs`
  - `tinyverse_cli/src/commands/tui/command.rs`
  - `tinyverse_cli/src/commands/tui/mod.rs`
- New crate:
  - `tinyverse_tui/Cargo.toml`
  - `tinyverse_tui/moon.yml`
  - `tinyverse_tui/src/lib.rs`
  - `tinyverse_tui/src/app.rs`
  - `tinyverse_tui/src/runtime.rs`

## Validation

- `cargo check --workspace`
- `cargo run --bin tinyverse -- tui`
