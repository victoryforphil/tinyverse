# Status Components Scope

## ActionLine

Responsibilities:

- Primary one-line outcome statement per command.
- Carries status glyph and verb.

Minimal contract:

- `level` (`ok|warn|err|info|muted`)
- `verb` (required)
- `subject` (optional)
- `detail` (optional short tail)
- `width` (optional, default 80)

Render variants:

- ANSI: themed color + optional bold on subject.
- Plain: ASCII-safe equivalent, no escape codes.

Width behavior:

- Never truncate glyph or verb.
- Truncate detail first, then subject if necessary.

Example:

```text
  ✓ Started session tinyverse_1738000000
```

## StatusBadge

Responsibilities:

- Compact inline status token for table cells/labels.

Minimal contract:

- `level`
- `label`
- `compact` flag

Render variants:

- ANSI: fg/bg semantic colors.
- Plain: bracketed label (`[ok]`, `[err]`).

Width behavior:

- Prefer compact form when narrow.

Example:

```text
  [ok]
```

## ErrorBlock

Responsibilities:

- Structured failure output with reason and recovery guidance.

Minimal contract:

- `headline`
- `reason`
- `details[]` (optional)
- `guidance` (optional)

Render variants:

- ANSI: red-emphasis headline + muted metadata labels.
- Plain: simple text block without styling.

Width behavior:

- Wrap reason/details lines; cap vertical detail spam.

Example:

```text
  ✗ Attach failed
    reason   not an interactive terminal (TTY required)

    run directly in your shell, not piped or scripted
```

## GuidanceLine

Responsibilities:

- Muted next-step hint or suggestion line.

Minimal contract:

- `text`
- `emphasis_tokens[]` (optional command snippets)

Render variants:

- ANSI: muted base text, accent for command snippets.
- Plain: plain text only.

Width behavior:

- Soft-wrap by word boundaries.

Example:

```text
  attach with: tv attach tinyverse_1738000000
```
