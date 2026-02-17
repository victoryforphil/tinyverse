# CLI Output UI Library Plan

This folder captures planning notes for a new tinyverse Rust library that improves non-interactive CLI output using Ratatui-style building blocks.

Goal: replace flat `info!` command output with consistent, readable, themed message components while keeping `--format json` behavior unchanged and avoiding a full interactive TUI for now.

Files:

- `00-context.md` - baseline repo context and integration points.
- `01-design-directions.md` - high-level visual directions and recommendation.
- `02-component-breakdown.md` - component catalog and module/API breakdown.
- `03-ratatui-docs-notes.md` - concrete docs references and implementation notes.
- `04-implementation-plan.plan.md` - staged implementation and migration plan.
- `05-implementation.prompt.md` - handoff prompt for implementation agent.
