# Build Prompt - Implement tinyverse CLI Output UI Library

Implement the plan in `docs/plan/cli-output-ui-lib/04-implementation-plan.plan.md`.

## Objective

Create a new Rust workspace crate for non-interactive CLI presentation components and migrate tinyverse command output away from plain `info!` lines to reusable styled components.

## Requirements

1. Keep `--format json` behavior identical.
2. Keep command execution signatures (`fn execute(...) -> anyhow::Result<()>`).
3. Start with an incremental migration (pilot: `list` command).
4. Use reusable primitives and composed blocks; do not inline formatting logic per command.
5. Support plain fallback for non-TTY output.

## Target outcomes

- Shared primitives exist for status/action lines, section headers, labeled fields, guidance lines, and error blocks.
- Table/list style reports can be rendered consistently.
- At least one command (`list`) is fully migrated end-to-end.
- Tests cover primitive rendering and migrated command outputs.

## Constraints

- Keep diffs focused and minimal.
- Avoid introducing interactive event-loop dependencies in this phase.
- Follow existing style in `AGENTS.md` and `STYLE.md`.

## Deliverable checklist

- Brief summary of architecture and migrated commands.
- List of touched files.
- Test commands run and results.
- Follow-up items deferred to V2.
