---
name: rust-test-nextest
description: Run Rust tests with `cargo nextest run` first, then fall back to `cargo test` when needed.
---

## What I do

- Standardize Rust test execution around `cargo nextest run`.
- Provide a deterministic fallback to `cargo test` when `nextest` is unavailable or fails due to environment/tooling constraints.

## When to use me

Use this when:

- A task needs Rust tests run for `tinyverse_cli` or other Rust crates in this repo.
- An agent would otherwise choose `cargo test` by default.

## Preferred workflow

1. Run `cargo nextest run` (optionally with `-p <crate>`).
2. If command is not found, unsupported, or fails due to environment/tooling setup, run `cargo test` with the same scope.
3. Report which path was used and why fallback happened when applicable.

## Common commands

- Workspace: `cargo nextest run`
- Crate scoped: `cargo nextest run -p tinyverse_cli`
- Fallback crate scoped: `cargo test -p tinyverse_cli`

## Notes

- Keep fallback behavior explicit in status updates.
- Do not treat regular test failures as a reason to silently switch tools without mentioning the fallback.
