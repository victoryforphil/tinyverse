# tinyverse_ui

Reusable non-interactive presentation components for tinyverse CLI output.

This crate is currently a standalone design and rendering sandbox. It is intentionally
not wired into `tinyverse_cli` yet.

## Goals

- Provide reusable output primitives for command result rendering.
- Support both `ANSI` and plain fallback output.
- Keep command-specific formatting logic outside command handlers.

## Current Components

- `ActionLine` - status/action row with badge and tone.
- `SectionHeader` - section title with underline rule.
- `LabeledField` - key/value metadata row.
- `StatusBadge` - compact status marker.
- `GuidanceLine` - next-step helper text.
- `ErrorBlock` - grouped error + detail (dimmed) + guidance.
- `DetailSection` - composed field group with header.
- `StyledTable` - width-aware table with bold headers, numeric alignment, optional row striping.
- `SummaryFooter` - report summary line (cyan in ANSI).

## Quick Tour

Run both examples in ANSI mode:

```bash
cargo run -p tinyverse_ui --example primitive_gallery
cargo run -p tinyverse_ui --example list_report_demo
```

Run plain mode:

```bash
cargo run -p tinyverse_ui --example primitive_gallery -- --plain
cargo run -p tinyverse_ui --example list_report_demo -- --plain
```

Run with the alternate minimal theme:

```bash
cargo run -p tinyverse_ui --example primitive_gallery -- --minimal-theme
cargo run -p tinyverse_ui --example list_report_demo -- --minimal-theme
```

## Testing

- Moon (nextest-first): `moon run tinyverse_ui:test`
- Cargo fallback: `cargo test -p tinyverse_ui`
- Timestamped review capture: `bun scripts/insta_review.sh.ts -p tinyverse_ui`
- Render A/B review docs: `bun scripts/render_snapshot_review.sh.ts [timestamp]`

## Integration Status

- `tinyverse_ui` is present in workspace and has examples/tests.
- `tinyverse_cli` remains unchanged and still uses its current output path.

See `docs/examples/tinyverse_ui-tour.md` for generated demo artifacts.
See `tinyverse_ui/docs/options.md` for component option points.
