----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /
- Keywords: ratatui, rust, tui, terminal ui, docs
- Summary: Ratatui is a Rust library for building fast, lightweight, and rich terminal user interfaces
----

Source: https://ratatui.rs/

# Ratatui

# Ratatui

Cook up delicious

terminal user interfaces.

Ratatui is a Rust library for building fast, lightweight, and rich terminal user interfaces

 [Get Started](/installation/)[Browse Examples](/examples/)[API Docs](https://docs.rs/ratatui/)

[GitHub](https://github.com/ratatui/ratatui)[Discord](https://discord.gg/pMCEU9hNEj)[Matrix](https://matrix.to/#/#ratatui:matrix.org)[Discourse](https://forum.ratatui.rs)[X.com](https://twitter.com/ratatui_rs)[Bluesky](https://bsky.app/profile/ratatui.rs)[Mastodon](https://fosstodon.org/@ratatui_rs)[LinkedIn](https://www.linkedin.com/company/ratatui-rs)

 [0.30.0](https://github.com/ratatui/ratatui/releases/tag/ratatui-v0.30.0)

```
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};

fn main() -> Result&#x3C;(), Box&#x3C;dyn std::error::Error>> {
    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let block = Block::bordered().title("Welcome");
            let greeting = Paragraph::new("Hello, Ratatui! 🐭")
                .centered()
                .yellow()
                .block(block);
            frame.render_widget(greeting, frame.area());
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
```

Trusted by {developers} building amazing apps

Create powerful TUIs, interactive dashboards, command-line games, and modern terminal experiences with ease.
 [2300+ 📦 crates built with Ratatui](https://crates.io/crates/ratatui/reverse_dependencies)[18.4k ⭐ stars on GitHub](https://github.com/ratatui/ratatui)[17.4M ⬇️ downloads on crates.io](https://crates.io/crates/ratatui)

Powering industry leaders:

## Built with Ratatui

Ratatui has all the ingredients you need to cook up exceptional terminal applications. Always
    fresh and full of cheese.

 [[scope-tui] ⭐ 463 A simple oscilloscope/vectorscope/spectroscope for your terminal](https://github.com/alemidev/scope-tui)

 [[rebels-in-the-sky] ⭐ 580 P2P terminal game about spacepirates](https://github.com/ricott1/rebels-in-the-sky)

 [[binsider] ⭐ 3.9k Perform binary analysis in your terminal](https://github.com/orhun/binsider)

 [[crossword] ⭐ 42 Play crossword puzzles in your terminal](https://github.com/matrixfrog/crossword)

 [[yozefu] ⭐ 268 Explore Kafka clusters interactively](https://github.com/MAIF/yozefu)

 [[openapi-tui] ⭐ 1.1k Terminal UI to list, browse and run APIs defined with OpenAPI](https://github.com/zaghaghi/openapi-tui)

 [[csvlens] ⭐ 3.5k A command line CSV file viewer](https://github.com/YS-L/csvlens)

 [[oxker] ⭐ 1.5k View & control docker containers with a TUI](https://github.com/mrjackwills/oxker)

[Explore the apps ::<>](/showcase/apps/)

## Watch and learn Ratatui

Watch our "recipes" for learning how to cook up TUIs, get inspired by other developers, and join
    conversations that spark new ideas.

 [[Building Embedded TUIs with Rust & Ratatui]](https://www.youtube.com/watch?v=F04kQMKwrwQ)

 Orhun Parmaksız showcases building embedded applications with Ratatui

 [[Are we embedded yet?]](https://www.youtube.com/watch?v=QPjojOuhbe8)

 Jagoda Ślązak explores how Ratatui is moving beyond the terminal

 [[Ratatui Tutorial Beginners Guide]](https://www.youtube.com/watch?v=M-BTpC_BEN0)

 A Ratatui Tutorial by Jonkero to get you up and running

 [[Textual UIs with Orhun Parmaksız]](https://www.youtube.com/watch?v=VbLz79trgz8)

 Bryan and Adam from Oxide Computer discussing TUIs with Orhun Parmaksız

 [[Bringing Terminal Aesthetics to the Web With Rust]](https://www.youtube.com/watch?v=iepbyYrF_YQ)

 What if I told you it is possible to build terminal-like web applications with Rust and vice versa

 [[Renaissance of Terminal User Interfaces with Rust]](https://www.youtube.com/watch?v=hWG51Mc1DlM)

 The terminal is essential for every programmer when it comes to productivity and efficiency

[Explore the tutorials ::<>](/tutorials/)

Cooked to perfection 🍲

 [⚡ [ Fast & Lightweight ]](/concepts/rendering/)

Sub-millisecond rendering with zero-cost abstractions and immediate-mode rendering. Build responsive dashboards and complex terminal applications that feel instant, even with many widgets on screen. No runtime overhead, just pure Rust performance.

 [🎨 [ Rich Widgets ]](/concepts/widgets/)

Everything you need to craft professional TUIs: charts, sparklines, tables, gauges, scrollable lists, progress bars, and more. Mix and match widgets to create interactive dashboards, monitoring tools, games and more. Check out the widget examples to see what's possible.

 [📐 [ Dynamic Layouts ]](/concepts/layout/)

Your UI looks great whether it's in a tiny tmux pane or a big terminal. Constraint-based responsive layouts that automatically adapt to any terminal size. Build complex nested layouts with horizontal and vertical splits, automatically sized containers, and percentage-based constraints. (Think Flexbox, but for the terminal.)

 [🦀 [ Pure Rust Reliability ]](/concepts/backends/)

Memory-safe, thread-safe, and type-safe by design. No C dependencies, no runtime exceptions, no undefined behavior. Just pure Rust goodness that compiles to efficient native code. Now supports embedded targets with no_std compatibility too!

## Ready to start cooking?

Join thousands of {rats} cooking up the next generation of terminal applications.

 [Get Started](/installation/)[Browse Examples](/examples/)[API Docs](https://docs.rs/ratatui/)

[GitHub](https://github.com/ratatui/ratatui)[Discord](https://discord.gg/pMCEU9hNEj)[Matrix](https://matrix.to/#/#ratatui:matrix.org)[Discourse](https://forum.ratatui.rs)[X.com](https://twitter.com/ratatui_rs)[Bluesky](https://bsky.app/profile/ratatui.rs)[Mastodon](https://fosstodon.org/@ratatui_rs)[LinkedIn](https://www.linkedin.com/company/ratatui-rs)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/index.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
