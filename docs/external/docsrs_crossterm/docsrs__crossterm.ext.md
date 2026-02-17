----
## External Docs Snapshot // docsrs_crossterm

- Captured: 2026-02-17T01:47:08.003Z
- Source root: https://docs.rs/crossterm/latest/crossterm/
- Source page: /crossterm/latest/crossterm
- Keywords: docs.rs, rust, crossterm, crossterm, crossterm
- Summary: [§](http://docs.rs/crossterm/latest/crossterm#cross-platform-terminal-manipulation-library)Cross-platform Terminal Manipulation Library
----

Source: https://docs.rs/crossterm/latest/crossterm

Expand description

[§](http://docs.rs/crossterm/latest/crossterm#cross-platform-terminal-manipulation-library)Cross-platform Terminal Manipulation Library
---------------------------------------------------------------------------------------------------------------------------------------

Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces.

This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested see [Tested Terminals](https://github.com/crossterm-rs/crossterm#tested-terminals) for more info).

### [§](http://docs.rs/crossterm/latest/crossterm#command-api)Command API

The command API makes the use of `crossterm` much easier and offers more control over when and how a command is executed. A command is just an action you can perform on the terminal e.g. cursor movement.

The command API offers:

*   Better Performance.
*   Complete control over when to flush.
*   Complete control over where the ANSI escape commands are executed to.
*   Way easier and nicer API.

There are two ways to use the API command:

*   Functions can execute commands on types that implement Write. Functions are easier to use and debug. There is a disadvantage, and that is that there is a boilerplate code involved.
*   Macros are generally seen as more difficult and aren’t always well supported by editors but offer an API with less boilerplate code. If you are not afraid of macros, this is a recommendation.

Linux and Windows 10 systems support ANSI escape codes. Those ANSI escape codes are strings or rather a byte sequence. When we `write` and `flush` those to the terminal we can perform some action. For older windows systems a WinAPI call is made.

#### [§](http://docs.rs/crossterm/latest/crossterm#supported-commands)Supported Commands

*   Module [`cursor`](https://docs.rs/crossterm/latest/crossterm/cursor/index.html)
    *   Visibility - [`Show`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.Show.html), [`Hide`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.Hide.html)
    *   Appearance - [`EnableBlinking`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.EnableBlinking.html), [`DisableBlinking`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.DisableBlinking.html), [`SetCursorStyle`](https://docs.rs/crossterm/latest/crossterm/cursor/enum.SetCursorStyle.html)
    *   Position - [`SavePosition`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.SavePosition.html), [`RestorePosition`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.RestorePosition.html), [`MoveUp`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveUp.html), [`MoveDown`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveDown.html), [`MoveLeft`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveLeft.html), [`MoveRight`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveRight.html), [`MoveTo`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveTo.html), [`MoveToColumn`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveToColumn.html),[`MoveToRow`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveToRow.html), [`MoveToNextLine`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveToNextLine.html), [`MoveToPreviousLine`](https://docs.rs/crossterm/latest/crossterm/cursor/struct.MoveToPreviousLine.html)

*   Module [`event`](https://docs.rs/crossterm/latest/crossterm/event/index.html)
    *   Keyboard events - [`PushKeyboardEnhancementFlags`](https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html), [`PopKeyboardEnhancementFlags`](https://docs.rs/crossterm/latest/crossterm/event/struct.PopKeyboardEnhancementFlags.html)
    *   Mouse events - [`EnableMouseCapture`](https://docs.rs/crossterm/latest/crossterm/event/struct.EnableMouseCapture.html), [`DisableMouseCapture`](https://docs.rs/crossterm/latest/crossterm/event/struct.DisableMouseCapture.html)

*   Module [`style`](https://docs.rs/crossterm/latest/crossterm/style/index.html)
    *   Colors - [`SetForegroundColor`](https://docs.rs/crossterm/latest/crossterm/style/struct.SetForegroundColor.html), [`SetBackgroundColor`](https://docs.rs/crossterm/latest/crossterm/style/struct.SetBackgroundColor.html), [`ResetColor`](https://docs.rs/crossterm/latest/crossterm/style/struct.ResetColor.html), [`SetColors`](https://docs.rs/crossterm/latest/crossterm/style/struct.SetColors.html)
    *   Attributes - [`SetAttribute`](https://docs.rs/crossterm/latest/crossterm/style/struct.SetAttribute.html), [`SetAttributes`](https://docs.rs/crossterm/latest/crossterm/style/struct.SetAttributes.html), [`PrintStyledContent`](https://docs.rs/crossterm/latest/crossterm/style/struct.PrintStyledContent.html)

*   Module [`terminal`](https://docs.rs/crossterm/latest/crossterm/terminal/index.html)
    *   Scrolling - [`ScrollUp`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.ScrollUp.html), [`ScrollDown`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.ScrollDown.html)
    *   Miscellaneous - [`Clear`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.Clear.html), [`SetSize`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.SetSize.html), [`SetTitle`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.SetTitle.html), [`DisableLineWrap`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.DisableLineWrap.html), [`EnableLineWrap`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnableLineWrap.html)
    *   Alternate screen - [`EnterAlternateScreen`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html), [`LeaveAlternateScreen`](https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html)

*   Module [`clipboard`](https://docs.rs/crossterm/latest/crossterm/clipboard/index.html) (requires [`feature = "osc52"`](http://docs.rs/crossterm/latest/crossterm#optional-features)) 
    *   Clipboard - [`CopyToClipboard`](https://docs.rs/crossterm/latest/crossterm/clipboard/struct.CopyToClipboard.html)

#### [§](http://docs.rs/crossterm/latest/crossterm#command-execution)Command Execution

There are two different ways to execute commands:

*   [Lazy Execution](http://docs.rs/crossterm/latest/crossterm#lazy-execution)
*   [Direct Execution](http://docs.rs/crossterm/latest/crossterm#direct-execution)

##### [§](http://docs.rs/crossterm/latest/crossterm#lazy-execution)Lazy Execution

Flushing bytes to the terminal buffer is a heavy system call. If we perform a lot of actions with the terminal, we want to do this periodically - like with a TUI editor - so that we can flush more data to the terminal buffer at the same time.

Crossterm offers the possibility to do this with `queue`. With `queue` you can queue commands, and when you call [Write::flush](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush) these commands will be executed.

You can pass a custom buffer implementing [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) to this `queue` operation. The commands will be executed on that buffer. The most common buffer is [std::io::stdout](https://doc.rust-lang.org/std/io/fn.stdout.html) however, [std::io::stderr](https://doc.rust-lang.org/std/io/fn.stderr.html) is used sometimes as well.

###### [§](http://docs.rs/crossterm/latest/crossterm#examples)Examples

A simple demonstration that shows the command API in action with cursor commands.

Functions:

```
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};

let mut stdout = stdout();
stdout.queue(cursor::MoveTo(5,5));

// some other code ...

stdout.flush();
```

The [queue](https://docs.rs/crossterm/latest/crossterm/trait.QueueableCommand.html) function returns itself, therefore you can use this to queue another command. Like `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.

Macros:

```
use std::io::{Write, stdout};
use crossterm::{queue, QueueableCommand, cursor};

let mut stdout = stdout();
queue!(stdout,  cursor::MoveTo(5, 5));

// some other code ...

// move operation is performed only if we flush the buffer.
stdout.flush();
```

You can pass more than one command into the [queue](https://docs.rs/crossterm/latest/crossterm/macro.queue.html) macro like `queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and they will be executed in the given order from left to right.

##### [§](http://docs.rs/crossterm/latest/crossterm#direct-execution)Direct Execution

For many applications it is not at all important to be efficient with ‘flush’ operations. For this use case there is the `execute` operation. This operation executes the command immediately, and calls the `flush` under water.

You can pass a custom buffer implementing [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) to this `execute` operation. The commands will be executed on that buffer. The most common buffer is [std::io::stdout](https://doc.rust-lang.org/std/io/fn.stdout.html) however, [std::io::stderr](https://doc.rust-lang.org/std/io/fn.stderr.html) is used sometimes as well.

###### [§](http://docs.rs/crossterm/latest/crossterm#examples-1)Examples

Functions:

```
use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, cursor};

let mut stdout = stdout();
stdout.execute(cursor::MoveTo(5,5));
```

The [execute](https://docs.rs/crossterm/latest/crossterm/trait.ExecutableCommand.html) function returns itself, therefore you can use this to queue another command. Like `stdout.execute(Goto(5,5))?.execute(Clear(ClearType::All))`.

Macros:

```
use std::io::{stdout, Write};
use crossterm::{execute, ExecutableCommand, cursor};

let mut stdout = stdout();
execute!(stdout, cursor::MoveTo(5, 5));
```

You can pass more than one command into the [execute](https://docs.rs/crossterm/latest/crossterm/macro.execute.html) macro like `execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and they will be executed in the given order from left to right.

### [§](http://docs.rs/crossterm/latest/crossterm#examples-2)Examples

Print a rectangle colored with magenta and use both direct execution and lazy execution.

Functions:

```
use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}
```

Macros:

```
use std::io::{self, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}
```

### [§](http://docs.rs/crossterm/latest/crossterm#feature-flags)Feature Flags

#### [§](http://docs.rs/crossterm/latest/crossterm#default-features)Default features

*   **`bracketed-paste`**_(enabled by default)_ — Enables triggering [`Event::Paste`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Paste "variant crossterm::event::Event::Paste") when pasting text into the terminal.
*   **`events`**_(enabled by default)_ — Enables reading input/events from the system using the [`event`](https://docs.rs/crossterm/latest/crossterm/event/index.html "mod crossterm::event") module.
*   **`windows`**_(enabled by default)_ — Enables windows specific crates.

#### [§](http://docs.rs/crossterm/latest/crossterm#optional-features)Optional Features

*   **`event-stream`** — Enables the [EventStream](https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html "struct crossterm::event::EventStream") struct for async event reading.
*   **`serde`** — Enables [`serde`](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/index.html "mod serde") for various types.
*   **`use-dev-tty`** — Enables raw file descriptor polling / selecting instead of mio.
*   **`derive-more`**_(enabled by default)_ — Enables `is_*` helper functions for event enums.
*   **`osc52`** — Enables interacting with a host clipboard via [`clipboard`](https://docs.rs/crossterm/latest/crossterm/clipboard/index.html)

[clipboard](https://docs.rs/crossterm/latest/crossterm/clipboard/index.html "mod crossterm::clipboard")A module for clipboard interaction[cursor](https://docs.rs/crossterm/latest/crossterm/cursor/index.html "mod crossterm::cursor")A module to work with the terminal cursor[event](https://docs.rs/crossterm/latest/crossterm/event/index.html "mod crossterm::event")A module to read events.[style](https://docs.rs/crossterm/latest/crossterm/style/index.html "mod crossterm::style")A module to apply attributes and colors on your text.[terminal](https://docs.rs/crossterm/latest/crossterm/terminal/index.html "mod crossterm::terminal")A module to work with the terminal.[tty](https://docs.rs/crossterm/latest/crossterm/tty/index.html "mod crossterm::tty")A module to query if the current instance is a tty. Making it a little more convenient and safe to query whether something is a terminal teletype or not. This module defines the IsTty trait and the is_tty method to return true if the item represents a terminal.[execute](https://docs.rs/crossterm/latest/crossterm/macro.execute.html "macro crossterm::execute")Executes one or more command(s).[queue](https://docs.rs/crossterm/latest/crossterm/macro.queue.html "macro crossterm::queue")Queues one or more command(s) for further execution.[Command](https://docs.rs/crossterm/latest/crossterm/trait.Command.html "trait crossterm::Command")An interface for a command that performs an action on the terminal.[Executable Command](https://docs.rs/crossterm/latest/crossterm/trait.ExecutableCommand.html "trait crossterm::ExecutableCommand")An interface for types that can directly execute commands.[Queueable Command](https://docs.rs/crossterm/latest/crossterm/trait.QueueableCommand.html "trait crossterm::QueueableCommand")An interface for types that can queue commands for further execution.[Synchronized Update](https://docs.rs/crossterm/latest/crossterm/trait.SynchronizedUpdate.html "trait crossterm::SynchronizedUpdate")An interface for types that support synchronized updates.

----
## Notes / Comments / Lessons

- Collection method: docs.rs crate sitemap discovery with in-page link expansion.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
