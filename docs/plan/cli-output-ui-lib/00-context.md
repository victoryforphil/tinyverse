# Context Snapshot

## What exists today in tinyverse

- `tinyverse_cli` command handlers currently emit user-facing output through `log::info!` or `log::error!`.
- Shared output formatting exists only for some commands via `render_output` in `tinyverse_cli/src/commands/output.rs`.
- `OutputFormat` currently supports `table | text | json`.
- `json` output is already clean via `serde_json::to_string_pretty` and should remain untouched.

Key files:

- `tinyverse_cli/src/commands/output.rs`
- `tinyverse_cli/src/logging.rs`
- `tinyverse_cli/src/commands/list/command.rs`
- `tinyverse_cli/src/commands/debug/command.rs`
- `tinyverse_cli/src/commands/spawn/command.rs`
- `tinyverse_cli/src/commands/providers/command.rs`
- `tinyverse_cli/src/commands/view/command.rs`
- `tinyverse_cli/src/commands/send/command.rs`
- `tinyverse_cli/src/commands/kill/command.rs`
- `tinyverse_cli/src/commands/attach/command.rs`

## Reference library context (dark-factory)

The path in the request appears to have moved from `libs/dark_tui_lib` to:

- `/Users/alex/repos/vfp/dark-factory/lib/dark_tui_components`

Reusable ideas worth borrowing for non-interactive output:

- Theme contract pattern (`ComponentThemeLike`) plus default palette.
- Stateless text primitives that return `Line`/`Span` (`SectionHeader`, `StatusPill`, `LabeledField`).
- Consistent spacing and muted-vs-primary text hierarchy.

Files inspected:

- `/Users/alex/repos/vfp/dark-factory/lib/dark_tui_components/src/theme.rs`
- `/Users/alex/repos/vfp/dark-factory/lib/dark_tui_components/src/components/section_header.rs`
- `/Users/alex/repos/vfp/dark-factory/lib/dark_tui_components/src/components/status_pill.rs`
- `/Users/alex/repos/vfp/dark-factory/lib/dark_tui_components/src/components/labeled_field.rs`

## Scope boundary for this effort

- In scope: better command output presentation (labels/messages/reports) for one-shot CLI commands.
- Out of scope: event loop, key handling, alternate-screen app shell, full interactive TUI architecture.
