# Component Options

This document shows practical option points for each component so CLI migrations can
choose consistent patterns.

## Rendering Modes

- `RenderMode::Ansi` for TTY output with color styling.
- `RenderMode::Plain` for redirected output and logs.
- `RenderContext { width }` enables width-aware table truncation.
- `RenderContext::for_stdout(theme)` auto-detects mode and terminal width.
- `default_stdout_context()` creates a context with the default theme.

## Theme Trait

The `Theme` trait provides all style mappings. `DefaultTheme` implements colorful defaults and
`MinimalTheme` provides mostly monochrome output.

| Method                | DefaultTheme style            | Used by                      |
|-----------------------|-------------------------------|------------------------------|
| `section_header_style`| Bold blue                     | `SectionHeader`              |
| `label_style`         | Bold cyan                     | `LabeledField`               |
| `guidance_style`      | Green                         | `GuidanceLine`               |
| `summary_style`       | Cyan                          | `SummaryFooter`              |
| `table_header_style`  | Bold                          | `StyledTable` header row     |
| `table_stripe_style`  | Dimmed                        | `StyledTable` striped rows   |
| `dim_style`           | Dimmed                        | `SectionHeader` underline, `ErrorBlock` detail |
| `tone_badge_style`    | Background color per tone     | `ActionLine`, `StatusBadge`  |
| `tone_text_style`     | Foreground color per tone     | `ActionLine`                 |

## Primitive Components

### `ActionLine`

- Inputs: `label`, `message`, `tone`.
- Use for command status updates (`INFO`, `DONE`, `ERROR`, `WARN`).
- The `label` is padded to 7 characters for alignment.

### `SectionHeader`

- Input: `title`.
- Renders as `== title ==` with an underline rule below.
- Use for report boundaries and major sections.

### `LabeledField`

- Inputs: `label`, `value`.
- Use for compact metadata rows.

### `StatusBadge`

- Inputs: `label`, `tone`.
- Use for short state markers (`IDLE`, `ACTIVE`, `FAILED`).
- ANSI mode adds padding spaces around the label.

### `GuidanceLine`

- Input: `message`.
- Renders with a `Next:` prefix.
- Use for explicit next-step follow-ups.

### `ErrorBlock`

- Required: `title`.
- Optional: `.with_detail(...)`, `.with_guidance(...)`.
- Detail text renders indented and dimmed (ANSI) or indented plain.
- Use for one clear failure reason plus recovery hint.

## Composed Components

### `DetailSection`

- Start with `DetailSection::new(title)`.
- Add fields with `.with_field(LabeledField::new(...))`.
- Renders a `SectionHeader` followed by `LabeledField` rows.

### `StyledTable`

- Start with `StyledTable::new(headers)`.
- Add rows with `.with_row(vec![...])`.
- Right-align numeric-like columns with `.with_numeric_columns(&[...])`.
- Add alternating ANSI stripes with `.with_stripe_mode(StripeMode::DimEvenRows)`.
- Headers render bold in ANSI mode.
- Width handling: columns shrink and ellipsize when `context.width` is small.

## Example Theme Selection

- Example binaries support `--minimal-theme` in addition to the default theme.
- Programmatic demo helpers accept `ExampleTheme::{Default,Minimal}` via
  `primitive_gallery_output_with_theme(...)` and `list_report_demo_output_with_theme(...)`.

### `SummaryFooter`

- Input: `message`.
- Renders with a `Summary:` prefix (cyan in ANSI, plain in plain mode).
- Use as final report summary line.
