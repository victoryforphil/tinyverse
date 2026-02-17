----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/backends/alternate-screen
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, backends, alternate screen
- Summary: The alternate screen is a separate buffer that some terminals provide, distinct from the main
----

Source: https://ratatui.rs/concepts/backends/alternate-screen

# Alternate Screen

The alternate screen is a separate buffer that some terminals provide, distinct from the main
screen. When activated, the terminal will display the alternate screen, hiding the current content
of the main screen. Applications can write to this screen as if it were the regular terminal
display, but when the application exits, the terminal will switch back to the main screen, and the
contents of the alternate screen will be cleared. This is useful for applications like text editors
or terminal games that want to use the full terminal window without disrupting the command line or
other terminal content.

This creates a seamless transition between the application and the regular terminal session, as the
content displayed before launching the application will reappear after the application exits.

Take this “hello world” program below. If we run it with and without the
`std::io::stderr().execute(EnterAlternateScreen)?` (and the corresponding `LeaveAlternateScreen`),
you can see how the program behaves differently.

```
17 collapsed linesuse std::{  io::{stderr, Result},  thread::sleep,  time::Duration,};
use ratatui::crossterm::{  terminal::{EnterAlternateScreen, LeaveAlternateScreen},  ExecutableCommand,};use ratatui::{prelude::*, widgets::*};
fn main() -> Result&#x3C;()> {  let should_enter_alternate_screen = std::env::args().nth(1).unwrap().parse::&#x3C;bool>().unwrap();  if should_enter_alternate_screen {  stderr().execute(EnterAlternateScreen)?; // remove this line  }  let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
  terminal.draw(|f| {    f.render_widget(Paragraph::new("Hello World!"), Rect::new(10, 20, 20, 1));  })?;  sleep(Duration::from_secs(2));
5 collapsed lines  if should_enter_alternate_screen {  stderr().execute(LeaveAlternateScreen)?; // remove this line  }  Ok(())}
```

Try running this code on your own and experiment with `EnterAlternateScreen` and
`LeaveAlternateScreen`.

Note that not all terminal emulators support the alternate screen, and even those that do may handle
it differently. As a result, the behavior may vary depending on the backend being used. Always
consult the specific backend’s documentation to understand how it implements the alternate screen.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/backends/alternate-screen.md)

 [Previous Raw Mode](/concepts/backends/raw-mode/) [Next Mouse Capture](/concepts/backends/mouse-capture/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
