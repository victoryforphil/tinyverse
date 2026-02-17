# Dark Factory - Style Guide

This file is the source of truth for style and implementation conventions in `tinyverse`.

## Coding Style (Minimal Baseline)

Until language/tool-specific configs exist, follow pragmatic defaults:

- Favor readable, explicit code over clever shortcuts.
- Keep functions focused and avoid hidden side effects.
- Avoid dead code, unused imports, and speculative abstractions.
- Keep naming consistent (`PascalCase` types, `camelCase` values/functions, `UPPER_SNAKE_CASE` constants).
- Handle errors with context; do not swallow exceptions silently.
- Never log secrets or credentials.
- Log messages should follow: `System // Optional Sub system // Message (meta={...})`.
  - Prefer structured JSON metadata over comma-delimited `key=value` text so long IDs/paths remain readable.
  - Example: `Core // HTTP // Listening (meta={"env":"development","host":"127.0.0.1","port":4150})`.

## Rust CLI Module Layout

- Prefer folder-based modules for Rust CLI code.
- Use one command folder per command under `tinyverse_cli/src/commands/`.
- Keep files focused and human-readable; target one primary struct/type per file, or close to it.
- Use `fn execute(args) -> anyhow::Result<()>` as the default command handler shape.
- Keep command argument structs in `args.rs` when a command has options/positionals.
- Keep command behavior in `command.rs`; keep leaf subcommand behavior in dedicated files.
- Keep root clap wiring in `tinyverse_cli/src/root.rs` and runtime dispatch in `tinyverse_cli/src/run.rs`.
- Prefer short imports and limited namespaces in Rust code; avoid deep fully-qualified paths when a local `use` or `super::` import keeps code clearer.
  
## Engineering Principles

- **Path Minimalism:** Prefer one maintained "hot path" per workflow and build automation around it.
  - Remove dead or duplicate approaches once a supported path is verified.
  - Avoid adding backup implementations "just in case" when one clean path is sufficient.
  - Optimize for easier human navigation: fewer entrypoints, clearer ownership, less branching process logic.
- **DRY (Don't Repeat Yourself):** Keep a single source of truth for shared behavior, rules, and transformations.
  - When duplication appears across modules, extract reusable helpers or modules instead of repeating logic.
  - Avoid copy-paste updates that can drift; prefer centralizing constants, validation rules, and serialization/parsing logic.
