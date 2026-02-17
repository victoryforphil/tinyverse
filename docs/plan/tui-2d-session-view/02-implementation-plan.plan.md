# tinyverse TUI Implementation Plan

## Goal

Ship a new `tinyverse tui` command with a production-safe terminal runtime and a usable 2D session card experience, then iteratively add inspector and action flows.

## Architecture

- Add crate: `tinyverse_tui`.
- Keep `tinyverse_cli` as root executable and command router.
- Keep `tinyverse_lib` as data/actions source (`SessionStore`, `TmuxClient`).
- Keep `tinyverse_ui` unchanged for non-interactive output.

## Stage 0: Plan Artifacts

Work:

- Add planning folder under `docs/plan/tui-2d-session-view/`.
- Capture context, port map, staged implementation, and execution TODO.

Pass criteria:

- Plan docs are committed and actionable.

## Stage 1: Command + Crate Scaffold (MVP gate A)

Work:

- Add workspace member `tinyverse_tui`.
- Add `tinyverse_cli` `tui` command wiring:
  - `tinyverse_cli/src/commands/tui/args.rs`
  - `tinyverse_cli/src/commands/tui/command.rs`
  - `tinyverse_cli/src/commands/tui/mod.rs`
  - update `tinyverse_cli/src/commands/mod.rs`
  - update `tinyverse_cli/src/root.rs`
  - update `tinyverse_cli/src/run.rs`
- Scaffold `tinyverse_tui`:
  - `src/lib.rs`
  - `src/app.rs`
  - `src/runtime.rs`
  - `Cargo.toml`
  - `moon.yml`

Pass criteria:

- `cargo check --workspace` passes.
- `tinyverse tui` launches and exits cleanly via `q`.

## Stage 2: Session Data + Basic 2D Cards (MVP gate B)

Work:

- Load sessions from `SessionStore`.
- Render card grid with selected state and keyboard navigation.
- Add periodic refresh and manual refresh key.
- Add top status line and bottom key-hint line.

Pass criteria:

- Cards render from real session data.
- `j/k` and arrow keys move selection.
- `r` refreshes data and status.

## Stage 3: Inspector + Actions (V1 gate A)

Work:

- Add right sidebar inspector for selected session details.
- Add basic action menu (keyboard first).
- Wire initial actions (read-only first): view details, refresh.
- Add destructive action confirmations before kill.

Pass criteria:

- Inspector updates with selection.
- Action menu and confirmation states are functional.

## Stage 4: Mouse + Context Menu + Split Resize (V1 gate B)

Work:

- Implement click-to-select cards.
- Add right-click context menu.
- Add split divider drag for card/inspector widths.

Pass criteria:

- Mouse input works with keyboard fallback.
- Layout remains stable across resize events.

## Stage 5: Operational Actions (V1 gate C)

Work:

- Wire attach/kill/send/spawn flows through `tinyverse_lib` APIs.
- Harden terminal restore path around attach transitions.

Pass criteria:

- End-to-end user workflows function from TUI.
- No terminal corruption on failure paths.

## Verification Commands

- `cargo check --workspace`
- `cargo test --workspace`
- `cargo run --bin tinyverse -- tui`
