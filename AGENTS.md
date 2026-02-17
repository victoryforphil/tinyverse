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
- Prefer `cargo nextest run` for Rust test execution.
- If `cargo nextest` is unavailable or errors due to environment/tooling, fall back to `cargo test`.

Operational commands:
- Install Moon project dependencies: `moon :install`
- Run Moon Rust checks/tests: `moon run tinyverse_cli:check`, `moon run tinyverse_cli:test`
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
