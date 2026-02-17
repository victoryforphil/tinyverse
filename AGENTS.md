# AGENTS.md

Minimal guidance for coding agents in `tinyverse`.

## Scope

- Keep changes small and task-focused.
- Prefer updating existing scripts/helpers over adding parallel paths.
- Update this file when conventions or entrypoints change.

## Rules Sources

- Cursor rules directory `.cursor/rules/`: not present.
- Cursor root file `.cursorrules`: not present.
- Copilot instructions `.github/copilot-instructions.md`: not present.
- Project style baseline: `STYLE.md`.

## Repository Shape

- Main code: `scripts/**/*.sh.ts`.
- Shared helpers: `scripts/helpers/`.
- Scraper entrypoints: `scripts/scrapes/`.
- Generated external docs: `docs/external/<source>/*.ext.md` plus `index.ext.md`.
- Rust CLI: `tinyverse_cli/src/`.
- Rust CLI commands: `tinyverse_cli/src/commands/<command>/`.
- Moon workspace config: `.moon/workspace.yml` and `.moon/toolchains.yml`.
- Moon Rust project configs: `tinyverse_lib/moon.yml` and `tinyverse_cli/moon.yml`.

## Setup

- Install managed tools: `proto install`
- Run installer script: `scripts/install.sh.ts`

## Build / Lint / Test

Current state:
- Moon workspace tasks are available for Rust projects.
- No dedicated root lint config.
- Rust CLI tests are available in `tinyverse_cli`.

Testing preference:
- Rust test execution prefers `cargo nextest run`, with automatic fallback to `cargo test` when nextest is unavailable.
- Wrapper script: `scripts/helpers/test_rust.sh.ts` (passes all args transparently).
- Via Moon: `moon run tinyverse_lib:test` or `moon :test` (uses wrapper internally).

Operational commands:
- Install Moon project dependencies: `moon :install`
- Build all projects through script wrapper: `bun scripts/build.sh.ts`
- Check all projects through script wrapper: `bun scripts/check.sh.ts`
- Test all projects through script wrapper: `bun scripts/test.sh.ts`
- Start dev watch loop through script wrapper: `bun scripts/dev.sh.ts`
- Run local CI flow orchestrator: `bun scripts/ci.sh.ts [mode]`
- Build docker images: `bun scripts/docker_build.sh.ts [targets...]`
- Run CI command in docker image: `bun scripts/docker_ci.sh.ts [run --] [command...]`
- Publish docker images (CI/local with auth): `bun scripts/docker_publish.sh.ts --image-repo <repo> --tag <tag> [--push]`
- Run Moon Rust checks/tests: `moon run tinyverse_cli:check`, `moon run tinyverse_cli:test`
- Run timestamped insta review capture: `scripts/insta_review.sh.ts -p tinyverse_ui`
- Render captured snapshot A/B view: `scripts/render_snapshot_review.sh.ts [timestamp]`
- Dispatch scraper: `scripts/scrapes/scrape_docs.sh.ts <source> [output_dir]`
- Run scraper directly: `scripts/scrapes/scrape_<source>_docs.sh.ts [output_dir]`
- Diesel scraper: `scripts/scrapes/scrape_docs.sh.ts diesel [output_dir]`
- Ratatui website scraper: `scripts/scrapes/scrape_docs.sh.ts ratatui [output_dir]`
- Docs.rs scraper: `scripts/scrapes/scrape_docs.sh.ts docsrs [docs_rs_url] [output_dir]`

Single-test note:
- When tests are added, document exact single-file and single-test-by-name commands here.

## Style

- Favor readable, explicit code.
- Keep functions focused; avoid hidden side effects.
- Remove unused imports and dead code.
- Naming: `PascalCase` types, `camelCase` values/functions, `UPPER_SNAKE_CASE` constants.
- Imports at top; use `node:` specifiers for Node built-ins.
- Prefer explicit types for exported contracts.
- Throw contextual `Error` values; do not silently swallow errors.
- Never log secrets or credentials.
- For Rust CLI code, prefer folder modules with one command folder per command.
- For Rust CLI command handlers, prefer `fn execute(args) -> anyhow::Result<()>`.
- Keep Rust CLI structs/types close to one primary type per file where practical.
- Prefer short imports and minimal namespace depth in Rust files (for example `SpawnArgs` via `use` or `super::args::SpawnArgs` instead of deep crate paths).

## Logging

- Prefer short, user-friendly messages at `info` level.
- Keep verbose metadata for `debug`/`trace` when it materially aids troubleshooting.

## Verification Checklist

- Run the touched command path when safe.
- Keep diffs minimal and style-consistent.
- Confirm no obvious runtime breakage from edited flow.

## Agent Reporting

- Default final task reports should be concise and high-signal.
- Prefer a short bullet list covering only:
  - high-level changes (file paths optional)
  - notable challenges/fixes
  - docs/notes added
  - testing performed with pass/fail
  - how to run/use the result
  - next steps or open questions (if any)
- Avoid long narrative summaries unless the user explicitly asks for deep detail.

## Agent Workflows

### TUI refinement (parent-orchestrated)

- Use this flow when refining, cleaning up, or finalizing Ratatui/TUI UX.
- Start with one broad `@tui_designer` pass that returns:
  - prioritized critique notes
  - a desired TODO list split into small tasks
- Parent agent turns that TODO into scoped work items and ownership.
- Parent agent runs `@explore` for file-level context before implementation handoffs.
- Parent agent dispatches smaller, focused tasks to `@tui_designer` (parallel when independent).
- Parent agent assembles changes, resolves overlap, and verifies touched command paths.
- Run one final `@tui_designer` review round for consistency and finishing polish.
