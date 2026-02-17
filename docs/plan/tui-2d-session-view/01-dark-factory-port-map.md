# dark-factory Port Map

## Port Strategy

Use dark-factory as a pattern library and selectively adapt reusable Ratatui components.

- Copy where domain-neutral and low-risk.
- Adapt where layout/runtime pattern is strong but domain model differs.
- Rewrite where dark-factory's product/variant/actor hierarchy does not map to tinyverse sessions.

## Tier 1 (MVP)

### Runtime shell

- Source:
  - `frontends/dark_tui/src/ui/mod.rs`
  - `frontends/dark_tui/src/main.rs`
- tinyverse target:
  - `tinyverse_tui/src/runtime.rs`
- plan:
  - adapt terminal setup/restore + poll/render loop
  - keep tinyverse runtime synchronous initially

### App state and command mapping

- Source:
  - `frontends/dark_tui/src/app/state.rs`
  - `frontends/dark_tui/src/ui/command_palette.rs`
- tinyverse target:
  - `tinyverse_tui/src/app.rs`
- plan:
  - rewrite to flat `Vec<StoredSession>` model
  - keep simple action handling for nav/refresh/quit

### Component primitives

- Source:
  - `lib/dark_tui_components/src/components/status_pill.rs`
  - `lib/dark_tui_components/src/components/key_hint_bar.rs`
  - `lib/dark_tui_components/src/components/pane_block_component.rs`
  - `lib/dark_tui_components/src/components/labeled_field.rs`
  - `lib/dark_tui_components/src/components/section_header.rs`
  - `lib/dark_tui_components/src/theme.rs`
- tinyverse target:
  - `tinyverse_tui/src/components/*`
  - `tinyverse_tui/src/theme.rs`
- plan:
  - start with local equivalents; port more directly as needed during V1

### Card view baseline

- Source:
  - `frontends/dark_tui/src/ui/render/views/catalog_cards.rs`
  - `lib/dark_tui_components/src/components/card_grid_component.rs`
- tinyverse target:
  - `tinyverse_tui/src/view.rs` (initial)
  - future split: `tinyverse_tui/src/views/session_cards.rs`
- plan:
  - adapt grid spacing/selection pattern
  - render tinyverse session card fields

## Tier 2 (V1)

- Context menu:
  - source: `frontends/dark_tui/src/ui/render/panels/context_menu_panel.rs`
  - target: `tinyverse_tui/src/panels/context_menu.rs`
- Inspector/details:
  - source: `frontends/dark_tui/src/ui/render/panels/details_panel.rs`
  - target: `tinyverse_tui/src/panels/inspector.rs`
- Mouse/hit-testing:
  - source: `frontends/dark_tui/src/ui/render/mod.rs`
  - target: `tinyverse_tui/src/input/mouse.rs`
- Split layout resizing:
  - source: `lib/dark_tui_components/src/utils/split_layout.rs`
  - target: `tinyverse_tui/src/layout/split.rs`

## Tier 3 (polish)

- Theme config file loading
- spawn/action overlays
- richer status/activity widgets

## Domain Bridging Notes

- dark-factory selection model is hierarchical and graph-oriented.
- tinyverse selection model is flat session-oriented.
- keep UI architecture pattern; replace entity-specific state and transitions with session-focused actions.
