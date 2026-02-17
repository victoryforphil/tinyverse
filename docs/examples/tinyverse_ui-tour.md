# tinyverse_ui Tour

Visual output tour for `tinyverse_ui` -- the non-interactive presentation library
for tinyverse CLI output. Each example runs in two modes: ANSI (color terminal)
and plain (redirected / piped output).

## Examples

### Primitive Gallery

Shows every component in one page: action lines (all four tones), status badges,
detail sections, tables, error blocks, summary footers, and guidance lines.

```bash
cargo run -p tinyverse_ui --example primitive_gallery          # ANSI
cargo run -p tinyverse_ui --example primitive_gallery -- --plain  # plain
cargo run -p tinyverse_ui --example primitive_gallery -- --minimal-theme
```

**Plain output:**

```
== Action Lines ==
------------------
[INFO   ] Scanning for tinyverse sessions
[DONE   ] Session tinyverse_213 attached
[WARN   ] Session tinyverse_old has no windows
[ERROR  ] Session tinyverse_404 not found

== Status Badges ==
-------------------
[ACTIVE]  [IDLE]  [FAILED]

== Session Details ==
---------------------
ID: $3
Name: tinyverse_213
Status: attached
Windows: 2

== Session Table ==
-------------------
ID | NAME            | STATUS   | WINDOWS
---+-----------------+----------+--------
$1 | tinyverse_alpha | attached |       2
$2 | tinyverse_beta  | idle     |       1

[ERROR  ] Unable to attach to tinyverse_404
  tmux session was not found on this host
Next: Run tinyverse list --all to see available sessions

Summary: 2 active, 1 idle, 1 failed
Next: Run tinyverse attach <name> to connect
```

### List Report

Simulates a `tinyverse list` report with a session table, inline badges,
summary, and next-step guidance. Demonstrates width-aware column truncation.

```bash
cargo run -p tinyverse_ui --example list_report_demo           # ANSI
cargo run -p tinyverse_ui --example list_report_demo -- --plain  # plain
cargo run -p tinyverse_ui --example list_report_demo -- --minimal-theme
```

**Plain output:**

```
== Sessions ==
--------------
[INFO   ] Found 3 tinyverse sessions

ID | NAME                                       | STATUS   | WINDOWS
---+--------------------------------------------+----------+--------
$1 | tinyverse_alpha                            | attached |       3
$2 | tinyverse_beta                             | idle     |       1
$3 | tinyverse_long_project_name_for_truncation | attached |       2

[2 attached]  [1 idle]
Summary: 3 sessions (filtered by prefix tinyverse_)
Next: Use --all to include every tmux session
```

## ANSI vs Plain comparison

| Feature             | ANSI mode                          | Plain mode                |
|---------------------|------------------------------------|---------------------------|
| Section headers     | Bold blue + dim underline rule     | `== Title ==` + dashes    |
| Action line badges  | Colored background (tone-matched)  | `[LABEL  ]` brackets      |
| Action line text    | Colored foreground (tone-matched)  | Plain text                |
| Status badges       | Background pill with padding       | `[LABEL]` brackets        |
| Field labels        | Bold cyan                          | Plain `Label: value`      |
| Table headers       | Bold text                          | Plain text                |
| Numeric columns     | Right-aligned (when configured)    | Right-aligned             |
| Row striping        | Optional dim striping              | N/A                       |
| Error detail        | Dimmed text, indented              | Indented plain text       |
| Guidance lines      | Green `Next:` prefix               | Plain `Next:` prefix      |
| Summary footer      | Cyan `Summary:` prefix             | Plain `Summary:` prefix   |

## Component catalog

| Component        | Purpose                              | Inputs                          |
|------------------|--------------------------------------|---------------------------------|
| `SectionHeader`  | Section title with underline rule    | `title`                         |
| `ActionLine`     | Status row with badge and message    | `label`, `message`, `tone`      |
| `StatusBadge`    | Compact state marker                 | `label`, `tone`                 |
| `LabeledField`   | Key-value metadata row               | `label`, `value`                |
| `DetailSection`  | Titled group of `LabeledField`s      | `title`, `.with_field(...)`     |
| `StyledTable`    | Width-aware table with truncation    | `headers`, `.with_row(...)`, `.with_numeric_columns(...)`, `.with_stripe_mode(...)` |
| `ErrorBlock`     | Error + detail + guidance group      | `title`, `.with_detail/guidance`|
| `SummaryFooter`  | Report summary line                  | `message`                       |
| `GuidanceLine`   | Next-step follow-up hint             | `message`                       |

## Tones

Tones control color mapping for `ActionLine`, `StatusBadge`, and `ErrorBlock`:

| Tone        | Badge background | Text color  | Typical label |
|-------------|------------------|-------------|---------------|
| `Info`      | Blue             | Blue        | `INFO`        |
| `Success`   | Green            | Green       | `DONE`        |
| `Warning`   | Yellow           | Yellow      | `WARN`        |
| `Error`     | Red              | Red (bold)  | `ERROR`       |
| `Neutral`   | White            | White       | `IDLE`        |

## Theme Variants

- `DefaultTheme`: colorful UI palette for primary CLI presentation.
- `MinimalTheme`: monochrome-first fallback for low-noise terminal output.
- Example binaries support `--minimal-theme` to preview the alternate theme.

Minimal-theme output samples are captured at:

- `docs/examples/tinyverse_ui/primitive_gallery-minimal.txt`
- `docs/examples/tinyverse_ui/list_report_demo-minimal.txt`

## Snapshot artifacts

Canonical output artifacts live in `tinyverse_ui/tests/snapshots/` and are generated by
`insta` snapshot tests:

| Snapshot file                                    | Content                        |
|--------------------------------------------------|--------------------------------|
| `example_snapshots__primitive_gallery_ansi.snap` | Gallery with ANSI escape codes |
| `example_snapshots__primitive_gallery_plain.snap`| Gallery plain text             |
| `example_snapshots__list_report_demo_ansi.snap`  | List report with ANSI codes    |
| `example_snapshots__list_report_demo_plain.snap` | List report plain text         |

## Refresh commands

From repository root:

```bash
# Run all tinyverse_ui tests including snapshot checks
cargo test -p tinyverse_ui

# Review and accept intentional snapshot updates
cargo insta review -p tinyverse_ui

# Capture before/after snapshots into timestamped docs artifacts
bun scripts/insta_review.sh.ts -p tinyverse_ui

# Build a side-by-side HTML + Markdown review from latest capture
bun scripts/render_snapshot_review.sh.ts
```

Timestamped review artifacts are written under:

- `docs/examples/tinyverse_ui/snapshot_reviews/<timestamp>/before`
- `docs/examples/tinyverse_ui/snapshot_reviews/<timestamp>/after`
- `docs/examples/tinyverse_ui/snapshot_reviews/<timestamp>/index.html`
- `docs/examples/tinyverse_ui/snapshot_reviews/index.md`
