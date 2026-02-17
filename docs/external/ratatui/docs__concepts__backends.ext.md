----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/backends
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, backends
- Summary: Ratatui interfaces with the terminal emulator through a backend. These libraries enable Ratatui via
----

Source: https://ratatui.rs/concepts/backends

# Backends

Ratatui interfaces with the terminal emulator through a backend. These libraries enable Ratatui via
the [`Terminal`](https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html) type to draw styled text to the screen, manipulate the cursor, and interrogate
properties of the terminal such as the console or window size. Your application will generally also
use the backend directly to capture keyboard, mouse and window events, and enable raw mode and the
alternate screen.

Ratatui supports the following backends:

- [Crossterm](https://crates.io/crates/crossterm) via [`CrosstermBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html) and the `crossterm` feature (enabled by default). Also see [Crossterm version compatibility](#crossterm-version-compatibility) below for details on selecting specific versions.

- [Termion](https://crates.io/crates/termion) via [`TermionBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html) and the `termion` feature.

- [Termwiz](https://crates.io/crates/termwiz) via [`TermwizBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html) and the `termwiz` feature.

- A [`TestBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html) which can be useful to unit test your application’s UI

For information on how to choose a backend see: [Comparison](./comparison/)

Each backend supports [Raw Mode](./raw-mode/) (which changes how the terminal handles input and
output processing), an [Alternate Screen](./alternate-screen/) which allows it to render to a
separate buffer than your shell commands use, and [Mouse Capture](./mouse-capture/), which allows
your application to capture mouse events.

### Crossterm version compatibility

[Section titled “Crossterm version compatibility”](#crossterm-version-compatibility)

Avoid pulling in multiple semver-incompatible [Crossterm](https://crates.io/crates/crossterm) versions. Different major versions:

- keep separate event queues (which can lead to race conditions and lost events),

- track raw mode separately (so raw mode may not be restored correctly on exit),

- cannot exchange types even when names match (leading to compilation errors).

Also, specific versions may make it difficult to upgrade Ratatui/widgets unless everything is up to
date.

As a mitigation, Ratatui 0.30+ supports multiple [Crossterm](https://crates.io/crates/crossterm) major versions via
`crossterm_{version}` feature flags. You can select which version to use and avoid conflicts in your
dependency graph.

For example:

- ``` ratatui = { version = "0.30", features = ["crossterm_0_28"] }crossterm = "0.28" # orratatui = { version = "0.30", features = ["crossterm_0_29"] }crossterm = "0.29" ``` Note If multiple flags are enabled, Ratatui selects the latest.

- The `ratatui-crossterm` crate exposes the same flags.

- Use `cargo tree -p crossterm` to check your graph and disable default features on dependencies that drag in another Crossterm major.

- Ratatui 0.30+ introduces `ratatui-core`, moving backends into separate crates so backend changes can evolve independently of the main library. This also helps avoid version conflicts in applications that only need one backend.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/backends/index.md)

 [Previous Flux Architecture](/concepts/application-patterns/flux-architecture/) [Next Comparison](/concepts/backends/comparison/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
