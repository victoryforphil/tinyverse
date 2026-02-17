# Ratatui Notes for Non-Interactive CLI Output

## Primary local docs references

- `docs/external/ratatui/docs__examples__apps__inline.ext.md`
- `docs/external/ratatui/docs__recipes__render__style-text.ext.md`
- `docs/external/ratatui/docs__recipes__render__display-text.ext.md`
- `docs/external/ratatui/docs__examples__widgets__paragraph.ext.md`
- `docs/external/ratatui/docs__examples__widgets__table.ext.md`
- `docs/external/ratatui/docs__examples__widgets__block.ext.md`
- `docs/external/ratatui/docs__recipes__widgets__paragraph.ext.md`
- `docs/external/ratatui/docs__recipes__widgets__block.ext.md`
- `docs/external/ratatui/docs__recipes__testing__snapshots.ext.md`
- `docs/external/ratatui/index.ext.md`

## Key capabilities to leverage

Styled text:

- Use `Style` + `Color` + `Modifier` with `Span` and `Line` for most status and label output.
- Mixed-style lines are straightforward and are enough for many non-interactive components.

Paragraph rendering:

- `Paragraph` works well for line blocks and wrapped guidance text.
- Useful options: alignment and wrap behavior.

Table rendering:

- `Table` + `Row` + `Cell` supports consistent report output.
- Explicit constraints are important for predictable widths.

Inline viewport path:

- Ratatui supports `Viewport::Inline(...)`, which fits scrollback-oriented outputs.
- For this effort, we can still start with plain stdout rendering and selectively adopt inline viewport where useful.

## Practical implementation notes

- Keep JSON output completely separate from pretty rendering.
- For non-TTY/piped output, provide plain text fallback (no ANSI styling).
- Width handling is required for section rule fill and table columns.
- Keep raw captured pane output (`view`) mostly untouched to avoid mangling embedded terminal text.

## Testing notes

- Use `TestBackend` and snapshot tests for deterministic visual output.
- Fix terminal dimensions in tests (for example 80x20) to keep snapshots stable.
- Snapshot-test key primitives and at least one full command output per migrated command.
