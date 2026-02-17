# Structured Components Scope

## SectionHeader

Responsibilities:

- Visual section boundary with label and trailing rule.

Contract:

- `label`
- `count` (optional)
- `width`

Width behavior:

- Preserve label text; reduce rule length first.

Narrow fallback:

- Label-only line if width is too small.

Example:

```text
  SESSIONS ----------------------------------------
```

## LabeledField

Responsibilities:

- Aligned key/value rows for metadata.

Contract:

- `label`
- `value`
- `label_width` (optional override)

Width behavior:

- Fixed label gutter, value consumes remainder.

Narrow fallback:

- Compact single-line `label: value` format.

Example:

```text
    session      tinyverse_1738000000
```

## DetailSection

Responsibilities:

- Composition of section header + labeled fields (+ optional guidance).

Contract:

- `title` (optional)
- `fields[]`
- `guidance` (optional)
- `width`

Width behavior:

- Compute label width from max label length with sensible clamp.

Narrow fallback:

- Compact field format for all entries.

Example:

```text
  DETAILS -----------------------------------------
    provider     opencode
    console      %1
    agent        %2
```

## StyledTable

Responsibilities:

- Consistent multi-row reports for list/debug/providers.

Contract:

- `columns[]` (name + min/flex metadata)
- `rows[]`
- `width`

Width behavior:

- Keep fixed columns readable; allocate remainder to one flex column.

Narrow fallback:

- Degrade to stacked row format when column layout collapses.

Example:

```text
  ID   NAME                    ATTACHED   WINDOWS
  $1   tinyverse_1738000000    0          1
```

## SummaryFooter

Responsibilities:

- Report tail with counts/totals/filter hints.

Contract:

- `segments[]`
- `note` (optional)

Width behavior:

- Join segments inline with separators when possible.

Narrow fallback:

- One segment per line.

Example:

```text
  showing 2 of 5 · filtered to tinyverse_ prefix · use --all
```
