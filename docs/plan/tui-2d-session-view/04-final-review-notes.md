# Final Review Notes (tui_designer)

This note captures the most useful recommendations from the final `@tui_designer` pass and how they map to current implementation.

## Implemented in current pass

- mouse capture enable/disable in terminal lifecycle
- click-to-select card hit-testing
- right-click action menu with anchored popup placement
- divider drag-resize between card grid and inspector
- action overlays for send/spawn input
- attach/send/spawn/kill operational wiring
- card truncation with ellipsis for long labels
- scroll-window behavior to keep selected card visible
- runtime module split into `runtime.rs`, `runtime/events.rs`, `runtime/render.rs`, and `runtime/helpers.rs`
- snapshot-style render tests added with Ratatui `TestBackend` + `insta`

## Deferred follow-ups

- add explicit status level model (`info`, `warning`, `error`, `success`) instead of free-form string
- add pointer-only context menu item click dismiss behavior polish
- add compact footer mode for narrow terminals
- add attach/send/spawn failure analytics and richer user-facing diagnostics

## Suggested next polish cycle

1. status-level model (`info`/`warning`/`error`/`success`) for footer messages
2. footer and narrow-terminal readability pass
3. context menu + inspector spacing/visual polish
4. additional action/error diagnostics around attach/send/spawn
