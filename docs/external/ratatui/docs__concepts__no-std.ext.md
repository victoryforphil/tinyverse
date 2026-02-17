----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/no-std
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, no std
- Summary: `no_std` is an attribute that disables Rust’s standard library.
----

Source: https://ratatui.rs/concepts/no-std

# Using no_std

Note

`no_std` is an attribute that disables Rust’s standard library.

In this mode your code can only uses a limited `core` crate instead of `std`, so there is no
operating system integration, file I/O, threads, or heap allocator unless you provide one.

Ratatui is `no_std`-compatible, which allows it to run in embedded and other resource-constrained
environments. This means you can run TUIs on a wider range of targets and have widget usable in both
desktop and embedded environments.

## Using Ratatui without `std`

[Section titled “Using Ratatui without std”](#using-ratatui-without-std)

- Disable default features so the crate does not pull in `std`-only dependencies:

- ``` ratatui = { version = "0.30", default-features = false } ``` NoteYou can re-enable only the features you need that don’t depend on the standard library. (e.g. `macros`, `all-widgets`, etc.)Skip features that explicitly require `std` such as `crossterm`, `serde` and so on. CautionOn targets without atomic instructions, you must also enable `portable-atomic` to provide software atomics: ``` ratatui = { version = "0.30", default-features = false, features = ["portable-atomic"] } ``` Additionally, depending on your target, you will have to enable one of the `portable-atomic` features. Refer to the [`portable-atomic` crate documentation](https://docs.rs/portable-atomic) for more information. Choose a backend that works on your platform.

The built-in backends rely on `std`, so `no_std` targets need a custom backend that implements
`ratatui::backend::Backend` or a third-party option like [`mousefood`](/ecosystem/mousefood/) 🧀

- When checking a `no_std` build, compile with a `no_std` target.

For example, on ESP32:

Terminal window

```
cargo check --target riscv32imc-unknown-none-elf
```

## Creating `no_std` widgets

[Section titled “Creating no_std widgets”](#creating-no_std-widgets)

If you already have a Ratatui widget, you can make it `no_std`-compatible with a few small changes.
Even if you haven’t built for embedded before!

Why `no_std` widgets?

The same widget can be reused in a terminal app on Linux, a dashboard on a microcontroller, or a web
app compiled to WebAssembly. Furthermore, it will be smaller (binary size) and more predictable
across platforms because it doesn’t pull in extra OS dependencies and only relies on core Rust
features.

- Opt into `no_std` and add `alloc` crate: lib.rs ``` #![no_std]extern crate alloc; ```

- Depend on `ratatui-core` instead of the full `ratatui` crate to avoid backend dependencies: ``` ratatui-core = { version = "0.1", default-features = false } ```

- Swap `std` types for their `core`/`alloc` equivalents, for example `core::fmt`, `alloc::string::String`, `alloc::vec::Vec`, and `alloc::boxed::Box`.

- Keep a `std` feature (off by default) for conveniences like tests or examples, but write your core widget logic so it also works without it.

- Avoid `std`-only APIs in widget code paths. Examples: use `core::time::Duration` instead of `std::time::Duration`, pass in data rather than reading files, and keep logging behind a feature so it can be disabled on targets without I/O.

Here is a minimal `no_std` widget implementation:

```
#![no_std]extern crate alloc;
use alloc::string::String;
use ratatui_core::buffer::Buffer;use ratatui_core::layout::Rect;use ratatui_core::text::Line;use ratatui_core::widgets::Widget;
struct Greeting {    message: String,}
impl Widget for &#x26;Greeting {    fn render(self, area: Rect, buf: &#x26;mut Buffer) {        Line::raw(&#x26;self.message).render(area, buf);    }}
```

Note

Some tips for testing and maintaining `no_std` compatibility:

- Run `cargo check --no-default-features` (optionally with a `no_std` target) to catch regressions.

- Document which optional features are `no_std`-compatible so users know what to enable.

- Keep features additive: use `cfg(feature = "std")` to layer in extra features (e.g. logging) without breaking `no_std`.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/no-std.md)

 [Previous Builder Lite Pattern](/concepts/builder-lite-pattern/) [Next Rendering](/concepts/rendering/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
