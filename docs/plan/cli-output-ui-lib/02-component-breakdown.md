# Component and Module Breakdown

## Primitive catalog (MVP)

- `ActionLine` - one-line result headline (`✓ Started session ...`).
- `StatusBadge` - compact status token (`ok`, `warn`, `err`, `info`).
- `SectionHeader` - uppercase label with trailing rule fill.
- `LabeledField` - aligned key/value row (`session`, `pane`, `provider`).
- `DetailSection` - grouped list of `LabeledField` rows.
- `StyledTable` - aligned report rows for `list`/`debug` style outputs.
- `ErrorBlock` - emphasized failure + reason + guidance.
- `GuidanceLine` - muted next-step line.
- `SummaryFooter` - count/totals tail line.

## Proposed crate/module shape

Preferred: a dedicated workspace crate, e.g. `tinyverse_ui`.

Suggested tree:

- `tinyverse_ui/src/lib.rs`
- `tinyverse_ui/src/theme.rs`
- `tinyverse_ui/src/primitives/mod.rs`
- `tinyverse_ui/src/primitives/action_line.rs`
- `tinyverse_ui/src/primitives/section_header.rs`
- `tinyverse_ui/src/primitives/labeled_field.rs`
- `tinyverse_ui/src/primitives/status_badge.rs`
- `tinyverse_ui/src/primitives/error_block.rs`
- `tinyverse_ui/src/primitives/guidance_line.rs`
- `tinyverse_ui/src/composed/mod.rs`
- `tinyverse_ui/src/composed/detail_section.rs`
- `tinyverse_ui/src/composed/styled_table.rs`
- `tinyverse_ui/src/composed/summary_footer.rs`
- `tinyverse_ui/src/render/mod.rs`
- `tinyverse_ui/src/render/plain.rs`
- `tinyverse_ui/src/render/ansi.rs`

## API shape (draft)

- `Theme` trait + `DefaultTheme` implementation.
- Primitive renderers return text primitives (`Line`/`Span`) or strings, depending on render backend choice.
- Renderer layer chooses ANSI or plain output based on terminal capability.

Example usage at call site:

```rust
let output = DetailSection::new("Spawn")
    .field("session", &result.session)
    .field("console", &result.console_pane_id)
    .field("agent", &result.agent_pane_id)
    .guidance("attach with: tv attach <session>")
    .render(&theme, width);
println!("{output}");
```

## Command-to-component mapping

- `spawn`: `ActionLine` + `DetailSection` + `GuidanceLine`
- `list`: `SectionHeader` + `StyledTable` + `SummaryFooter`
- `debug self`: `SectionHeader` + `DetailSection` + `StyledTable`
- `providers`: `SectionHeader` + simple list/table + `GuidanceLine`
- `view`: `ActionLine` + divider + raw captured text + divider
- `send`: `ActionLine` (+ optional command preview field)
- `kill`: destructive `ActionLine`
- `attach` failures: `ErrorBlock` + `GuidanceLine`

## Defer to V2

- Interactive widgets/stateful selection.
- Alternate screen management.
- Animations/spinners.
- Overly dynamic layouts that require event loops.
