----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/backends/comparison
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, backends, comparison
- Summary: Choose [Crossterm](https://crates.io/crates/crossterm) for most tasks.
----

Source: https://ratatui.rs/concepts/backends/comparison

# Comparison of Backends

TLDR

Choose [Crossterm](https://crates.io/crates/crossterm) for most tasks.

Ratatui interfaces with the terminal emulator through its “backends”. These are powerful libraries
that grant `ratatui` the ability to capture keypresses, maneuver the cursor, style the text with
colors and other features. As of now, `ratatui` supports three backends:

- [Crossterm](https://crates.io/crates/crossterm)

- [Termion](https://crates.io/crates/termion)

- [Termwiz](https://crates.io/crates/termwiz)

Selecting a backend does influence your project’s structure, but the core functionalities remain
consistent across all options. Here’s a flowchart that can help you make your decision.

Though we try to make sure that all backends are fully-supported, the most commonly-used backend is
Crossterm. If you have no particular reason to use Termion or Termwiz, you will find it easiest to
learn Crossterm simply due to its popularity.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/backends/comparison.md)

 [Previous Backends](/concepts/backends/) [Next Raw Mode](/concepts/backends/raw-mode/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
