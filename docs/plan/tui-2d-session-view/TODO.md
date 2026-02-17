# TODO - tinyverse 2D TUI

## Stage 0 - Planning

- [x] Create planning folder and README.
- [x] Capture context and dark-factory port map.
- [x] Write staged implementation plan and handoff prompt.

## Stage 1 - Command + Crate Scaffold

- [x] Add `tinyverse_tui` workspace member.
- [x] Add `tinyverse_cli` dependency on `tinyverse_tui`.
- [x] Add `tui` command args/module/wiring in CLI.
- [x] Create `tinyverse_tui` crate skeleton (`lib.rs`, `app.rs`, `runtime.rs`, `moon.yml`).
- [x] Implement terminal setup/restore and quit flow (`q`, `Esc`).

## Stage 2 - Session Cards MVP

- [x] Add app state with selected index and session list.
- [x] Implement data refresh from `SessionStore`.
- [x] Render basic 2D session cards with selected highlight.
- [x] Implement keyboard navigation (`j/k/h/l`, arrows).
- [x] Add status line + key hints.

## Stage 3 - Inspector + Actions

- [x] Add inspector sidebar for selected session metadata.
- [x] Add action menu state and keyboard interaction.
- [x] Add confirmation flow for destructive actions.

## Stage 4 - Mouse + Resize

- [ ] Add click-to-select and right-click context menu.
- [ ] Add split resize behavior between cards and inspector.

## Stage 5 - Operational Actions

- [ ] Wire attach, kill, send, spawn actions.
- [ ] Harden terminal state transitions around attach.

## Verification

- [x] `cargo check --workspace`
- [x] `cargo test --workspace`
- [x] manual run: `cargo run -p tinyverse_cli -- tui`
