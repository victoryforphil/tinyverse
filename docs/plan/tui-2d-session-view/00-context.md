# Context

## User Intent

- Make a more graphical TUI experience for tinyverse.
- Use a primary/only 2D navigable surface with cards for tinyverse sessions.
- Include an action system, click handling, action menu, and sidebar inspector.
- Take design and architectural inspiration from dark-factory's TUI.
- Keep tinyverse CLI as entrypoint, with a dedicated `tui` command.

## tinyverse Current Architecture (relevant paths)

- CLI entrypoint and dispatch:
  - `tinyverse_cli/src/main.rs`
  - `tinyverse_cli/src/root.rs`
  - `tinyverse_cli/src/run.rs`
  - `tinyverse_cli/src/commands/*`
- Domain/session APIs:
  - `tinyverse_lib/src/session_store/mod.rs`
  - `tinyverse_lib/src/tmux/mod.rs`
  - `tinyverse_lib/src/tmux_helpers.rs`
- Existing output UI (non-interactive ANSI):
  - `tinyverse_ui/src/*`

## dark-factory References (inspected)

- Runtime/event loop:
  - `frontends/dark_tui/src/ui/mod.rs`
  - `frontends/dark_tui/src/main.rs`
- 2D/cards rendering:
  - `frontends/dark_tui/src/ui/render/views/catalog_cards.rs`
  - `frontends/dark_tui/src/ui/render/views/unified_catalog_view.rs`
- Actions/command palette:
  - `frontends/dark_tui/src/ui/command_palette.rs`
- Mouse/hit-testing:
  - `frontends/dark_tui/src/ui/render/mod.rs`
  - `frontends/dark_tui/src/ui/mod.rs`
- Context menu and details panel:
  - `frontends/dark_tui/src/ui/render/panels/context_menu_panel.rs`
  - `frontends/dark_tui/src/ui/render/panels/details_panel.rs`
- Shared TUI components/theme:
  - `lib/dark_tui_components/src/*`

## Constraints and Design Decisions

- Keep existing `tinyverse_ui` as ANSI output library; do not overload it with Ratatui internals.
- Add a new `tinyverse_tui` crate for interactive rendering and runtime loop.
- Keep first milestone narrow and shippable:
  - launchable TUI command
  - refreshable session card surface
  - keyboard navigation
  - clean terminal setup/restore behavior
- Defer advanced flows (context menu, mouse interaction details, spawn form, attach lifecycle transitions) to follow-up phases.

## Data Shape for Initial Cards

Source: `tinyverse_lib::StoredSession`.

- `session_name`
- `session_key`
- `agent_type`
- `status_string`
- `tmux_session_name`
- `created_at`
- `updated_at`

Optional (for inspector/action work in later phases):

- `description`
- `tmux_session_id`
- `console_pane_id`
- `agent_pane_id`
- `last_message_at`

## First Build Scope

Introduce and wire:

- `tinyverse tui` command in CLI.
- `tinyverse_tui::run(options)` runtime entrypoint.
- minimal app state + tick/event loop.
- basic card grid rendering from session data.
- hotkeys: navigate (`j/k` + arrows), refresh (`r`), quit (`q`/`Esc`).
