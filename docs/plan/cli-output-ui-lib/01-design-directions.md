# Design Directions

## Direction A - Terminal Ledger

- Structured receipt-like output.
- Uppercase section titles + horizontal rules.
- Minimal color; high legibility; low visual noise.

Good for conservative UX, but less distinctive.

## Direction B - Dark Console (recommended)

- Keep dark_tui_components language in static CLI output.
- Use status pills/dots, section headers, aligned labeled fields.
- Render as inline blocks in scrollback (not alternate screen).

Why this is best for tinyverse now:

- Reuses proven vocabulary from dark-factory components.
- Gives stronger visual hierarchy than plain logs.
- Preserves a migration path to richer Ratatui surfaces later.

## Direction C - Signal Minimal

- Almost no chrome; glyph-first lines (`✓`, `✗`, `·`).
- Fast to implement and scan.
- Lowest branding and weakest structure for complex outputs.

## Recommended visual system

Tone:

- Direct and operational (no chatty filler).
- Keep verbs concise: `Started`, `Sent`, `Killed`, `Failed`.

Spacing:

- 2-space left padding for content blocks.
- Single blank line between logical sections.

Border philosophy:

- Prefer rules and spacing; avoid heavy boxes for standard outputs.
- Use box framing only for explicit error blocks when needed.

Color semantics:

- `ok`: green
- `warn`: yellow
- `err`: red
- `info`: blue/cyan
- `muted`: gray for labels/rules/hints

Message hierarchy:

1. Action line (headline)
2. Detail fields (metadata)
3. Optional report/table/list section
4. Optional guidance footer
