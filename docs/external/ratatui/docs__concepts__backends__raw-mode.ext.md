----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/backends/raw-mode
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, backends, raw mode
- Summary: Raw mode is a mode where the terminal does not perform any processing or handling of the input and
----

Source: https://ratatui.rs/concepts/backends/raw-mode

# Raw Mode

Raw mode is a mode where the terminal does not perform any processing or handling of the input and
output. This means that features such as echoing input characters, line buffering, and special
character processing (e.g., `CTRL-C` or `SIGINT`) are disabled. This is useful for applications that
want to have complete control over the terminal input and output, processing each keystroke
themselves.

For example, in raw mode, the terminal will not perform line buffering on the input, so the
application will receive each key press as it is typed, instead of waiting for the user to press
enter. This makes it suitable for real-time applications like text editors, terminal-based games,
and more.

Each backend handles raw mode differently, so the behavior may vary depending on the backend being
used. Be sure to consult the backend’s specific documentation for exact details on how it implements
raw mode.

- [`CrosstermBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html)

- [`TermionBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html)

- [`TermwizBackend`](https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/backends/raw-mode.md)

 [Previous Comparison](/concepts/backends/comparison/) [Next Alternate Screen](/concepts/backends/alternate-screen/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
