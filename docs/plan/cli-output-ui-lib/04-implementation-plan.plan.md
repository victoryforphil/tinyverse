# Implementation Plan - tinyverse CLI Output UI Library

## Goal

Introduce a reusable library for richer non-interactive CLI presentation so tinyverse command handlers stop emitting flat log-like user output.

## Constraints

- Keep behavior of `--format json` untouched.
- Keep command handlers returning `anyhow::Result<()>`.
- Avoid introducing a full-screen interactive TUI in this phase.
- Keep migration incremental by command.

## Stage 1 - Foundation crate + primitives

Work:

- Add a workspace crate (`tinyverse_ui`) for output components.
- Implement `Theme` + `DefaultTheme` with semantic colors.
- Implement MVP primitives (`ActionLine`, `SectionHeader`, `LabeledField`, `StatusBadge`, `GuidanceLine`, `ErrorBlock`).
- Implement plain + ANSI render paths.

Pass criteria:

- `cargo check --workspace` passes.
- Primitive snapshot tests pass.

## Stage 2 - Structured blocks and tables

Work:

- Add composed components (`DetailSection`, `StyledTable`, `SummaryFooter`).
- Add width-aware rendering helpers.

Pass criteria:

- `list`/`debug` style reports can be rendered through the new library in unit tests.

## Stage 3 - First command migration (pilot)

Pilot command: `list`.

Work:

- Replace current prettytable/text rendering path in `tinyverse_cli/src/commands/list/command.rs` with new component path for non-JSON formats.
- Preserve `json` via existing serialization report.

Pass criteria:

- Command output is cleaner and consistent in table/text modes.
- Existing list command tests are updated and passing.

## Stage 4 - Sweep remaining commands

Migrate in this order:

1. `providers`
2. `spawn`
3. `send`
4. `kill`
5. `debug self`
6. `view`
7. `attach` failure path

Pass criteria:

- No user-facing `info!` lines remain for migrated command result output.
- Error paths have clear reason + next-step guidance format.

## Stage 5 - Polish and consistency hardening

Work:

- Add style/voice consistency checks in tests.
- Ensure plain fallback behavior for non-TTY output.
- Document component usage patterns in crate docs.

Pass criteria:

- `cargo nextest run` (or `cargo test` fallback) is green for touched crates.
- Visual output snapshots are stable and intentional.

## Verification checklist

- `cargo check --workspace`
- `cargo nextest run -p tinyverse_ui` (fallback: `cargo test -p tinyverse_ui`)
- `cargo nextest run -p tinyverse_cli` (fallback: `cargo test -p tinyverse_cli`)
