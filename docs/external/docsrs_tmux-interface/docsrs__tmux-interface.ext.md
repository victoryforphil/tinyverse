----
## External Docs Snapshot // docsrs_tmux-interface

- Captured: 2026-02-17T01:08:35.151Z
- Source root: https://docs.rs/tmux_interface/latest/tmux_interface/
- Source page: /tmux_interface/latest/tmux_interface
- Keywords: docs.rs, rust, tmux_interface, tmux_interface, tmux interface
- Summary: [Source](https://docs.rs/tmux_interface/latest/src/tmux_interface/lib.rs.html#1-508)
----

Source: https://docs.rs/tmux_interface/latest/tmux_interface

Crate tmux_ interface
---------------------

[Source](https://docs.rs/tmux_interface/latest/src/tmux_interface/lib.rs.html#1-508)

Expand description

`tmux_interface` is a library for communication with [TMUX](https://github.com/tmux/tmux) via CLI.

[§](http://docs.rs/tmux_interface/latest/tmux_interface#on-this-page)On This Page
---------------------------------------------------------------------------------

*       1.   [Description](http://docs.rs/tmux_interface/latest/tmux_interface#1-description)

*       1.   [Quick Start](http://docs.rs/tmux_interface/latest/tmux_interface#2-quick-start)

*       1.   [Package Compilation Features](http://docs.rs/tmux_interface/latest/tmux_interface#3-package-compilation-features)

    *   3.1. [Tmux Version](http://docs.rs/tmux_interface/latest/tmux_interface#31-tmux-version)
    *   3.2. [Tmux Command Alias](http://docs.rs/tmux_interface/latest/tmux_interface#32-tmux-command-alias)
    *   3.3. [Repository](http://docs.rs/tmux_interface/latest/tmux_interface#3-3-repository)
        *   3.3.1 [Using Crates Repository](http://docs.rs/tmux_interface/latest/tmux_interface#331-using-crates-repository)
        *   3.3.2 [Using Local Repository](http://docs.rs/tmux_interface/latest/tmux_interface#332-using-local-repository)
        *   3.3.3 [Using Remote Repository](http://docs.rs/tmux_interface/latest/tmux_interface#333-using-remote-repository)

*       1.   [Modules Overview](http://docs.rs/tmux_interface/latest/tmux_interface#4-modules-overview)

*       1.   [Modules and Levels Hierarchy](http://docs.rs/tmux_interface/latest/tmux_interface#5-modules-and-levels-hierarchy)

[§](http://docs.rs/tmux_interface/latest/tmux_interface#1-description)1. Description
------------------------------------------------------------------------------------

Main purpose of the `tmux_interface` library is to implement simple sending and receiving data mechanisms for intercommunication with `TMUX` only via standard streams (`stdin`, `stdout`, `stderr`).

[§](http://docs.rs/tmux_interface/latest/tmux_interface#2-quick-start)2. Quick Start
------------------------------------------------------------------------------------

1.   Add a dependency in your `Cargo.toml`. Versions below `1.0.0` are mostly for development and testing purposes (use them in your projects on your own risk, further versions may have different API).

```
[dependencies]
tmux_interface = "1.0.0"
``` 
2.   Add extern crate in your source file.

`extern crate tmux_interface;` 
3.   Use it’s functions

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#example-1)Example 1

```
use tmux_interface::{HasSession, KillSession, NewSession, NewWindow, SplitWindow, Tmux};

let target_session = "example_1";

// tmux new -d -s example_1 ; neww ; splitw -v
Tmux::new()
    .add_command(NewSession::new().detached().session_name(target_session))
    .add_command(NewWindow::new())
    .add_command(SplitWindow::new().vertical())
    .output()
    .unwrap();

// tmux has -t example_1
let status = Tmux::with_command(HasSession::new().target_session(target_session))
    .status()
    .unwrap()
    .success();

assert!(status);

// tmux kill-session -t example_1
Tmux::with_command(KillSession::new().target_session(target_session))
    .output()
    .unwrap();
``` 

[§](http://docs.rs/tmux_interface/latest/tmux_interface#3-package-compilation-features)3. Package Compilation Features
----------------------------------------------------------------------------------------------------------------------

### [§](http://docs.rs/tmux_interface/latest/tmux_interface#31-tmux-version)3.1 Tmux Version

Different tmux versions may have incompatible CLI changes. Following versions features are currently supported:

**Table 3.1:**`Cargo.toml` features list with corresponding tmux versions

| Feature Name | Tmux Version | CI Tests | Comment |
| --- | --- | --- | --- |
| `tmux_0_8` | `tmux 0.8` |  |  |
| `tmux_0_9` | `tmux 0.9` |  |  |
| `tmux_1_0` | `tmux 1.0` |  |  |
| `tmux_1_1` | `tmux 1.1` |  |  |
| `tmux_1_2` | `tmux 1.2` |  |  |
| `tmux_1_3` | `tmux 1.3` |  |  |
| `tmux_1_4` | `tmux 1.4` |  |  |
| `tmux_1_5` | `tmux 1.5` |  |  |
| `tmux_1_6` | `tmux 1.6` |  | Ubuntu 11.04 LTS Precise Pangolin, CentOS 6 |
| `tmux_1_7` | `tmux 1.7` |  | Ubuntu 14.04 LTS Trusty Tahr, CentOS 7 |
| `tmux_1_8` | `tmux 1.8` | x |  |
| `tmux_1_9` | `tmux 1.9` | x | Debian Jessie |
| `tmux_1_9a` | `tmux 1.9a` | x |  |
| `tmux_2_0` | `tmux 2.0` | x |  |
| `tmux_2_1` | `tmux 2.1` | x | Ubuntu 16.04 LTS Xenial Xerus |
| `tmux_2_2` | `tmux 2.2` | x |  |
| `tmux_2_3` | `tmux 2.3` | x | Debian Stretch |
| `tmux_2_4` | `tmux 2.4` | x |  |
| `tmux_2_5` | `tmux 2.5` | x |  |
| `tmux_2_6` | `tmux 2.6` | x | Ubuntu 18.04 LTS Bionic Beaver |
| `tmux_2_7` | `tmux 2.7` | x | CentOS 8 |
| `tmux_2_8` | `tmux 2.8` | x | Debian Buster |
| `tmux_2_9` | `tmux 2.9` | x |  |
| `tmux_2_9a` | `tmux 2.9a` | x |  |
| `tmux_3_0` | `tmux 3.0` | x |  |
| `tmux_3_0a` | `tmux 3.0a` | x | Ubuntu 20.04 LTS Focal Fossa |
| `tmux_3_1` | `tmux 3.1` | x |  |
| `tmux_3_1a` | `tmux 3.1a` | x |  |
| `tmux_3_1b` | `tmux 3.1b` | x |  |
| `tmux_3_1c` | `tmux 3.1c` | x | Debian Bullseye |
| `tmux_3_2` | `tmux 3.2` | x |  |
| `tmux_3_2a` | `tmux 3.2a` | x | Ubuntu 22.04 LTS Jammy Jellyfish, CentOS 9 |
| `tmux_3_3` | `tmux 3.3` | x |  |
| `tmux_3_3a` | `tmux 3.3a` | x | Debian Bookworm, Ubuntu 23.04 LTS Lunar Lobster |
| `tmux_3_4` | `tmux 3.4` | x | Debian experimental, Ubuntu 24.04 LTS Noble Numbat |
| `tmux_X_X` |  | x | tmux: `main` branch; library: `dev` branch |
|  |  |  |  |
| `tmux_stable` | `tmux 3.3` |  |  |
| `tmux_latest` | `tmux 3.3a` |  |  |

```
[dependencies]
tmux_interface = {
 version = "^0.1.0",
 features = ["tmux_2_6"]
}
```

By default `tmux_stable` is used. It can be removed with `--no-default-features` cargo command line option or with `default-features = false` option in `Cargo.toml` You can also add `features` to your dependencies entry in `Cargo.toml`, if you want to specify the version of tmux you want to use.

```
[dependencies]
tmux_interface = {
 version = "^0.1.0",
 default-features = false,
 features = ["tmux_2_6"]
}
```

### [§](http://docs.rs/tmux_interface/latest/tmux_interface#32-tmux-command-alias)3.2. Tmux Command Alias

`cmd_alias` use alias instead of full tmux command name (e.g. `list-sessions` ->`ls`). Enabled by default.

### [§](http://docs.rs/tmux_interface/latest/tmux_interface#33-repository)3.3. Repository

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#331-using-crates-repository)3.3.1. Using Crates Repository

```
[dependencies]
tmux_interface = {
 version = "0.0.7",
}
```

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#332-using-local-repository)3.3.2. Using Local Repository

```
[dependencies]
tmux_interface = {
 version = "0.0.7",
 path = "../tmux-interface"
}
```

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#333-using-remote-repository)3.3.3. Using Remote Repository

```
tmux_interface = {
 git = "https://github.com/AntonGepting/tmux-interface-rs.git",
 branch = "dev"
}
```

[§](http://docs.rs/tmux_interface/latest/tmux_interface#4-modules-overview)4. Modules Overview
----------------------------------------------------------------------------------------------

*   Commands ([`commands`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/index.html "mod tmux_interface::commands"))

    *   Clients and Sessions ([`clients_and_sessions`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/clients_and_sessions/index.html "mod tmux_interface::commands::clients_and_sessions"))
    *   Windows and Panes ([`windows_and_panes`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/windows_and_panes/index.html "mod tmux_interface::commands::windows_and_panes"))
    *   Key Bindings ([`key_bindings`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/key_bindings/index.html "mod tmux_interface::commands::key_bindings"))
    *   Options ([`options`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/options/index.html "mod tmux_interface::commands::options"))
    *   Hooks ([`hooks`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/hooks/index.html "mod tmux_interface::commands::hooks"))
    *   Global and Session Environment ([`global_and_session_environment`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/global_and_session_environment/index.html "mod tmux_interface::commands::global_and_session_environment"))
    *   Status Line ([`status_line`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/status_line/index.html "mod tmux_interface::commands::status_line"))
    *   Buffers ([`buffers`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/buffers/index.html "mod tmux_interface::commands::buffers"))
    *   Miscellaneous ([`miscellaneous`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/miscellaneous/index.html "mod tmux_interface::commands::miscellaneous"))
    *   …
    *   Common ([`common`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/common/index.html "mod tmux_interface::commands::common"))
    *   TmuxCommand ([`TmuxCommand`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux_command/struct.TmuxCommand.html "struct tmux_interface::commands::tmux_command::TmuxCommand")), TmuxCommands ([`TmuxCommands`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux_commands/struct.TmuxCommands.html "struct tmux_interface::commands::tmux_commands::TmuxCommands"))
    *   Tmux ([`Tmux`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux/struct.Tmux.html "struct tmux_interface::commands::tmux::Tmux"))
    *   …

*   Modes

    *   Default Mode
    *   Control Mode ([`control_mode`](https://docs.rs/tmux_interface/latest/tmux_interface/control_mode/index.html "mod tmux_interface::control_mode")) 
        *   (unimplemented, draft)

    *   Copy Mode 
        *   (unimplemented, draft)

    *   Command Mode 
        *   (unimplemented)

    *   Clock Mode 
        *   (unimplemented)

*   Formats ([`formats`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/index.html "mod tmux_interface::formats"))

    *   [`Formats`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/formats/struct.Formats.html "struct tmux_interface::formats::formats::Formats")
    *   [`FormatsOutput`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/formats_output/struct.FormatsOutput.html "struct tmux_interface::formats::formats_output::FormatsOutput")
    *   [`Variable`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/variable/enum.Variable.html "enum tmux_interface::formats::variable::Variable")
    *   [`VariableOutput`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/variable_output/enum.VariableOutput.html "enum tmux_interface::formats::variable_output::VariableOutput")
    *   …

*   Options ([`options`](https://docs.rs/tmux_interface/latest/tmux_interface/options/index.html "mod tmux_interface::options"))

*   Styles ([`styles`](https://docs.rs/tmux_interface/latest/tmux_interface/styles/index.html "mod tmux_interface::styles"))

    *   [`StyleList`](https://docs.rs/tmux_interface/latest/tmux_interface/styles/style_list/struct.StyleList.html "struct tmux_interface::styles::style_list::StyleList")
    *   [`Style`](https://docs.rs/tmux_interface/latest/tmux_interface/styles/style/enum.Style.html "enum tmux_interface::styles::style::Style")
    *   …

*   Target ([`target`](https://docs.rs/tmux_interface/latest/tmux_interface/target/index.html "mod tmux_interface::target"))

    *   [`TargetSession`](https://docs.rs/tmux_interface/latest/tmux_interface/target/target_session/enum.TargetSession.html "enum tmux_interface::target::target_session::TargetSession")
    *   [`TargetWindow`](https://docs.rs/tmux_interface/latest/tmux_interface/target/target_window/enum.TargetWindow.html "enum tmux_interface::target::target_window::TargetWindow")
    *   [`TargetPane`](https://docs.rs/tmux_interface/latest/tmux_interface/target/target_pane/enum.TargetPane.html "enum tmux_interface::target::target_pane::TargetPane")
    *   …

*   Variables ([`variables`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/index.html "mod tmux_interface::variables"))

    *   [`Sessions`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/session/sessions/struct.Sessions.html "struct tmux_interface::variables::session::sessions::Sessions")
    *   [`Session`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/session/session/struct.Session.html "struct tmux_interface::variables::session::session::Session")
    *   [`Windows`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/window/windows/struct.Windows.html "struct tmux_interface::variables::window::windows::Windows")
    *   [`Window`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/window/window/struct.Window.html "struct tmux_interface::variables::window::window::Window")
    *   [`Panes`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/pane/panes/struct.Panes.html "struct tmux_interface::variables::pane::panes::Panes")
    *   [`Pane`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/pane/pane/struct.Pane.html "struct tmux_interface::variables::pane::pane::Pane")
    *   …

*   Error ([`Error`](https://docs.rs/tmux_interface/latest/tmux_interface/error/enum.Error.html "enum tmux_interface::error::Error"))

*   …

[§](http://docs.rs/tmux_interface/latest/tmux_interface#5-modules-and-levels-hierarchy)5. Modules and Levels Hierarchy
----------------------------------------------------------------------------------------------------------------------

```
5. Tmux Objects Controller
 +---------+     +-----------+                             +-----+
 | Options |     | Variables |                             | ... |
 +---------+     +-----------+                             +-----+
 ...

4. Tmux Objects Getter/Setter
 +-----------------+                                       +-----+
 | GetServerOption |                                       | ... |
 +-----------------+                                       +-----+
 ...

3. Command Builder
 +------+     +------------+      +---------------+        +-----+
 | Tmux |     | NewSession |      | AttachSession |        | ... |
 +------+     +------------+      +---------------+        +-----+

2. Tmux Command
 +-------------+                  +------------+
 | TmuxCommand |                  | TmuxOutput |
 +-------------+                  +------------+
 +-----------------+
 | TmuxCommands    |
 +-----------------+

1. Standard Library
 +---------------------------------------+
 |        std::process::Command          |
 +---------------------------------------+
 +-----------+ +-----------+ +-----------+
 | .output() | | .spawn()  | | .status() |
 +-----------+ +-----------+ +-----------+

 Platform specific (Windows, UNIX, ...)
 +---------------------------------------+
 |             sys::process              |
 +---------------------------------------+
 +-----------+ +-------------------------+
 | .output() | |        .spawn()         |
 +-----------+ +-------------------------+

0. OS
 +--------+                      +-----------------+       +-----+
 | fork() |                      | CreateProcess() |       | ... |
 +--------+                      +-----------------+       +-----+
```

**Figure 5:** Schematic Levels and Modules Hierarchy

and thereby:

*   Each level allows to build practically the same command, but with more or less effort and advantages
*   Each level has some abstraction and some limitations
*   Each level is based on top of the previous one (uses APIs of the previous one)

### [§](http://docs.rs/tmux_interface/latest/tmux_interface#51-level-explanations-and-examples)5.1. Level Explanations and Examples

Tmux command invocation can be described and accessed on different levels:

*       1.   syscall `fork(...)`, `CreateProcess(...)` - Operating System level abstraction

*       1.   [`std::process::Command`](https://doc.rust-lang.org/nightly/std/process/struct.Command.html "struct std::process::Command") - Rust standard library level abstraction

    *   OS independence
    *   comfortable working low level
    *   manually build commands using literals
    *   hard coded literals

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#examples)Examples

```
use std::process::Command;

// tmux -2 -uv new-session -ADEd -s example_5_1_1
let output = Command::new("tmux")
    .args(["-2", "-uv", "new-session", "-ADEd", "-s", "example_5_1_1"])
    .output()
    .unwrap();

assert!(output.status.success());

// tmux -2 -uv kill-session -t example_5_1_1
let output = Command::new("tmux")
    .args(["-2", "-uv", "kill-session", "-t", "example_5_1_1"])
    .output()
    .unwrap();

assert!(output.status.success());
```

**Listing 5.1.1:** build tmux commands using `std::process::Command`

*       1.   [`TmuxCommand`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux_command/struct.TmuxCommand.html "struct tmux_interface::commands::tmux_command::TmuxCommand"), [`TmuxCommands`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux_commands/struct.TmuxCommands.html "struct tmux_interface::commands::tmux_commands::TmuxCommands") - custom command abstraction

    *   additional functionality for [`std::process::Command`](https://doc.rust-lang.org/nightly/std/process/struct.Command.html "struct std::process::Command")
    *   allows to store additional information about commands such as: 
        *   command alias (`new`), beside command name (`new-session`)
        *   short flag name (`-l`) and long flag name (`--long-flag`)
        *   custom separator, hyphen, etc… (``, `-`, `--`, `=`, ``)

    *   runtime mechanisms for deciding and building short or long commands

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#examples-1)Examples

```
use tmux_interface::TmuxCommand;

// new-session -ADEd -s example_5_1_2
let mut new_session = TmuxCommand::new();
new_session
    .name("new-session")
    .push_flag_short('A')
    .push_flag_short('D')
    .push_flag_short('E')
    .push_flag_short('d')
    .arg("-s", "example_5_1_2");

// tmux -2uv new-session -ADEd -s example_5_1_2
let mut tmux = TmuxCommand::new();
tmux.name("tmux")
    .push_flag_short('2')
    .push_flag_short('u')
    .push_flag_short('v')
    .push_cmd(new_session)
    .combine_short_flags();

let output = tmux.to_command().output().unwrap();

assert!(output.status.success());

// kill-session -t example_5_1_2
let mut kill_session = TmuxCommand::new();
kill_session.name("kill-session").arg("-t", "example_5_1_2");

// tmux -2uv kill-session -t example_5_1_2
let mut tmux = TmuxCommand::new();
tmux.name("tmux")
    .push_flag_short('2')
    .push_flag_short('u')
    .push_flag_short('v')
    .push_cmd(kill_session)
    .combine_short_flags();

let output = tmux.to_command().output().unwrap();

assert!(output.status.success());
```

**Listing 5.1.2:** build tmux commands using `tmux_interface::TmuxCommand`

*       1.   [`Tmux`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/tmux/struct.Tmux.html "struct tmux_interface::commands::tmux::Tmux"), [`NewSession`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/clients_and_sessions/new_session/struct.NewSession.html "struct tmux_interface::commands::clients_and_sessions::new_session::NewSession"), [`AttachSession`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/clients_and_sessions/attach_session/struct.AttachSession.html "struct tmux_interface::commands::clients_and_sessions::attach_session::AttachSession") … - tmux commands builder

    *   structures, traits, implementations and methods as abstraction from literals
    *   near to tmux naming as possible
    *   build tmux commands
    *   tmux commands can include binary name and arguments or nor for control mode
    *   order of arguments doesn’t matter
    *   using macros

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#examples-2)Examples

```
use tmux_interface::{KillSession, NewSession, Tmux};

let session_name = "example_5_1_3";

// tmux -2uv new-session -ADEd -s example_5_1_3
let tmux = Tmux::with_command(
    NewSession::new()
        .attach()
        .detach_other()
        .not_update_env()
        .detached()
        .session_name(session_name),
)
.colours256()
.force_utf8()
.verbose_logging();

let output = tmux.output().unwrap();

assert!(output.success());

// tmux -2uv kill-session -t example_5_1_3
let tmux = Tmux::with_command(KillSession::new().target_session(session_name))
    .colours256()
    .force_utf8()
    .verbose_logging();

let output = tmux.output().unwrap();

assert!(output.success());
```

**Listing 5.1.3:** build tmux commands using `tmux_interface::{Tmux, NewSession, KillSession} structures`

```
use tmux_interface::{kill_session, new_session, tmux};

let session_name = "example_5_1_4";

// tmux -2uv new-session -ADEd -s example_5_1_4
let tmux = tmux!(-2, -u, -v, new_session!(-A, -D, -E, -d, -s session_name));

let output = tmux.output().unwrap();

assert!(output.success());

// tmux -2uv kill-session -t example_5_1_4
let tmux = tmux!(-2, -u, -v, kill_session!(-t session_name));

let output = tmux.output().unwrap();

assert!(output.success());
```

**Listing 5.1.4:** build tmux commands using `tmux_interface::{tmux, new_session, kill_session} macros`

*   [`Options`](https://docs.rs/tmux_interface/latest/tmux_interface/options/index.html "mod tmux_interface::options"), [`Variables`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/index.html "mod tmux_interface::variables"), [`Formats`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/formats/struct.Formats.html "struct tmux_interface::formats::formats::Formats") - tmux objects control

    *   accessing and using internal tmux instances 
        *   formats
        *   options
        *   variables
        *   …

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#example)Example

`unimplemented!();`

*   `TmuxInterface` - tmux control

    *   setting/getting methods abstraction, just an object with it’s attributes
    *   offline/online working (default/control mode)
    *   mapping of whole tmux with it’s all internal instances as an object in Rust

#### [§](http://docs.rs/tmux_interface/latest/tmux_interface#example-1)Example

`unimplemented!();`

`pub use error::Error;``pub use commands::*;``pub use control_mode::*;``pub use formats::*;``pub use options::*;``pub use styles::*;``pub use target::*;``pub use variables::*;`[commands](https://docs.rs/tmux_interface/latest/tmux_interface/commands/index.html "mod tmux_interface::commands")The [`commands`](https://docs.rs/tmux_interface/latest/tmux_interface/commands/index.html "mod tmux_interface::commands") module contains functions for building and executing tmux commands[control_ mode](https://docs.rs/tmux_interface/latest/tmux_interface/control_mode/index.html "mod tmux_interface::control_mode")The [`control_mode`](https://docs.rs/tmux_interface/latest/tmux_interface/control_mode/index.html "mod tmux_interface::control_mode") module contains functions for working in control mode of tmux[copy_ mode](https://docs.rs/tmux_interface/latest/tmux_interface/copy_mode/index.html "mod tmux_interface::copy_mode")[error](https://docs.rs/tmux_interface/latest/tmux_interface/error/index.html "mod tmux_interface::error")[formats](https://docs.rs/tmux_interface/latest/tmux_interface/formats/index.html "mod tmux_interface::formats")The [`formats`](https://docs.rs/tmux_interface/latest/tmux_interface/formats/formats/index.html "mod tmux_interface::formats::formats") module contains functions for working with tmux formats[options](https://docs.rs/tmux_interface/latest/tmux_interface/options/index.html "mod tmux_interface::options")Command builders and output parsers[styles](https://docs.rs/tmux_interface/latest/tmux_interface/styles/index.html "mod tmux_interface::styles")[target](https://docs.rs/tmux_interface/latest/tmux_interface/target/index.html "mod tmux_interface::target")The [`target`](https://docs.rs/tmux_interface/latest/tmux_interface/target/index.html "mod tmux_interface::target") module contains functions for building targets for tmux commands[variables](https://docs.rs/tmux_interface/latest/tmux_interface/variables/index.html "mod tmux_interface::variables")The [`variables`](https://docs.rs/tmux_interface/latest/tmux_interface/variables/index.html "mod tmux_interface::variables") module contains functions for getting variables from tmux[attach_ session](https://docs.rs/tmux_interface/latest/tmux_interface/macro.attach_session.html "macro tmux_interface::attach_session")Generate command using flags from TMUX manual[break_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.break_pane.html "macro tmux_interface::break_pane")Break `src-pane` off from its containing window to make it the only pane in `dst-window`[capture_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.capture_pane.html "macro tmux_interface::capture_pane")Manual[choose_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.choose_buffer.html "macro tmux_interface::choose_buffer")Stucture for putting a pane into buffer mode[choose_ client](https://docs.rs/tmux_interface/latest/tmux_interface/macro.choose_client.html "macro tmux_interface::choose_client")Put a pane into client mode, allowing a client to be selected interactively from a list[choose_ tree](https://docs.rs/tmux_interface/latest/tmux_interface/macro.choose_tree.html "macro tmux_interface::choose_tree")Put a pane into tree mode, where a session, window or pane may be chosen interactively from a list[clear_ history](https://docs.rs/tmux_interface/latest/tmux_interface/macro.clear_history.html "macro tmux_interface::clear_history")Remove and free the history for the specified pane.[clear_ prompt_ history](https://docs.rs/tmux_interface/latest/tmux_interface/macro.clear_prompt_history.html "macro tmux_interface::clear_prompt_history")Manual[clock_ mode](https://docs.rs/tmux_interface/latest/tmux_interface/macro.clock_mode.html "macro tmux_interface::clock_mode")Manual[command_ prompt](https://docs.rs/tmux_interface/latest/tmux_interface/macro.command_prompt.html "macro tmux_interface::command_prompt")Structure for open the command prompt in a client[confirm_ before](https://docs.rs/tmux_interface/latest/tmux_interface/macro.confirm_before.html "macro tmux_interface::confirm_before")Manual[copy_ mode](https://docs.rs/tmux_interface/latest/tmux_interface/macro.copy_mode.html "macro tmux_interface::copy_mode")Enter copy mode[delete_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.delete_buffer.html "macro tmux_interface::delete_buffer")Delete the buffer named buffer-name, or the most recently added automatically named buffer if not specified.[detach_ client](https://docs.rs/tmux_interface/latest/tmux_interface/macro.detach_client.html "macro tmux_interface::detach_client")Manual[display_ menu](https://docs.rs/tmux_interface/latest/tmux_interface/macro.display_menu.html "macro tmux_interface::display_menu")Structure for displaying a menu on target-client[display_ message](https://docs.rs/tmux_interface/latest/tmux_interface/macro.display_message.html "macro tmux_interface::display_message")Structure for displaying a message[display_ panes](https://docs.rs/tmux_interface/latest/tmux_interface/macro.display_panes.html "macro tmux_interface::display_panes")Display a visible indicator of each pane shown by `target-client`[display_ popup](https://docs.rs/tmux_interface/latest/tmux_interface/macro.display_popup.html "macro tmux_interface::display_popup")Structure for displaying a menu on target-client[find_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.find_window.html "macro tmux_interface::find_window")Search for the fnmatch(3) pattern `match-string` in window names, titles, and visible content (but not history)[has_ session](https://docs.rs/tmux_interface/latest/tmux_interface/macro.has_session.html "macro tmux_interface::has_session")Report if the specified session exist[join_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.join_pane.html "macro tmux_interface::join_pane")Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it and move `src-pane` into the space[kill_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.kill_pane.html "macro tmux_interface::kill_pane")Destroy the given pane[kill_ server](https://docs.rs/tmux_interface/latest/tmux_interface/macro.kill_server.html "macro tmux_interface::kill_server")Kill the tmux server and clients and destroy all sessions[kill_ session](https://docs.rs/tmux_interface/latest/tmux_interface/macro.kill_session.html "macro tmux_interface::kill_session")Destroy the given session[kill_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.kill_window.html "macro tmux_interface::kill_window")Kill the current window or the window at target-window, removing it from any sessions to which it is linked[last_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.last_pane.html "macro tmux_interface::last_pane")Select the last (previously selected) pane[last_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.last_window.html "macro tmux_interface::last_window")Select the last (previously selected) window[link_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.link_window.html "macro tmux_interface::link_window")Link the window at src-window to the specified dst-window[list_ buffers](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_buffers.html "macro tmux_interface::list_buffers")List the global buffers.[list_ clients](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_clients.html "macro tmux_interface::list_clients")List all clients attached to the server[list_ commands](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_commands.html "macro tmux_interface::list_commands")List the syntax of all commands supported by tmux[list_ keys](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_keys.html "macro tmux_interface::list_keys")Manual[list_ panes](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_panes.html "macro tmux_interface::list_panes")List panes on the server[list_ sessions](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_sessions.html "macro tmux_interface::list_sessions")List all sessions managed by the server[list_ windows](https://docs.rs/tmux_interface/latest/tmux_interface/macro.list_windows.html "macro tmux_interface::list_windows")List windows on the server[load_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.load_buffer.html "macro tmux_interface::load_buffer")Load the contents of the specified paste buffer from path.[lock_ server](https://docs.rs/tmux_interface/latest/tmux_interface/macro.lock_server.html "macro tmux_interface::lock_server")Manual[move_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.move_pane.html "macro tmux_interface::move_pane")Like join-pane, but `src-pane` and `dst-pane` may belong to the same window[move_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.move_window.html "macro tmux_interface::move_window")This is similar to link-window, except the window at `src-window` is moved to `dst-window`[new_ session](https://docs.rs/tmux_interface/latest/tmux_interface/macro.new_session.html "macro tmux_interface::new_session")Structure for creating a new session[new_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.new_window.html "macro tmux_interface::new_window")Structure for creating new window, using `tmux new-window` command[next_ layout](https://docs.rs/tmux_interface/latest/tmux_interface/macro.next_layout.html "macro tmux_interface::next_layout")Move a window to the next layout and rearrange the panes to fit[next_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.next_window.html "macro tmux_interface::next_window")Move to the next window in the session[paste_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.paste_buffer.html "macro tmux_interface::paste_buffer")Structure for inserting the contents of a paste buffer into the specified pane[pipe_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.pipe_pane.html "macro tmux_interface::pipe_pane")Pipe output sent by the program in target-pane to a shell command or vice versa[previous_ layout](https://docs.rs/tmux_interface/latest/tmux_interface/macro.previous_layout.html "macro tmux_interface::previous_layout")Move to the previous layout in the session[previous_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.previous_window.html "macro tmux_interface::previous_window")Move to the previous window in the session[rename_ session](https://docs.rs/tmux_interface/latest/tmux_interface/macro.rename_session.html "macro tmux_interface::rename_session")Rename the session to `new-name`[rename_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.rename_window.html "macro tmux_interface::rename_window")Rename the current window, or the window at target-window if specified, to new-name[resize_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.resize_pane.html "macro tmux_interface::resize_pane")Resize a pane, up, down, left or right[resize_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.resize_window.html "macro tmux_interface::resize_window")Resize a window, up, down, left or right[respawn_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.respawn_pane.html "macro tmux_interface::respawn_pane")Reactivate a pane in which the command has exited[respawn_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.respawn_window.html "macro tmux_interface::respawn_window")Reactivate a window in which the command has exited[rotate_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.rotate_window.html "macro tmux_interface::rotate_window")Rotate the positions of the panes within a window[run_ shell](https://docs.rs/tmux_interface/latest/tmux_interface/macro.run_shell.html "macro tmux_interface::run_shell")Manual[save_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.save_buffer.html "macro tmux_interface::save_buffer")Save the contents of the specified paste buffer to path.[select_ layout](https://docs.rs/tmux_interface/latest/tmux_interface/macro.select_layout.html "macro tmux_interface::select_layout")Choose a specific layout for a window[select_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.select_pane.html "macro tmux_interface::select_pane")Make pane `target-pane` the active pane in window `target-window`[select_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.select_window.html "macro tmux_interface::select_window")Select the window at target-window.[send_ keys](https://docs.rs/tmux_interface/latest/tmux_interface/macro.send_keys.html "macro tmux_interface::send_keys")Structure[send_ prefix](https://docs.rs/tmux_interface/latest/tmux_interface/macro.send_prefix.html "macro tmux_interface::send_prefix")Manual[server_ access](https://docs.rs/tmux_interface/latest/tmux_interface/macro.server_access.html "macro tmux_interface::server_access")Structure for creating a new session[set_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.set_buffer.html "macro tmux_interface::set_buffer")Set the contents of the specified buffer to data.[show_ buffer](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_buffer.html "macro tmux_interface::show_buffer")Display the contents of the specified buffer.[show_ environment](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_environment.html "macro tmux_interface::show_environment")Manual[show_ hooks](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_hooks.html "macro tmux_interface::show_hooks")Manual[show_ messages](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_messages.html "macro tmux_interface::show_messages")Show client messages or server information[show_ options](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_options.html "macro tmux_interface::show_options")Structure for showing options[show_ prompt_ history](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_prompt_history.html "macro tmux_interface::show_prompt_history")Manual[show_ window_ options](https://docs.rs/tmux_interface/latest/tmux_interface/macro.show_window_options.html "macro tmux_interface::show_window_options")Manual[source_ file](https://docs.rs/tmux_interface/latest/tmux_interface/macro.source_file.html "macro tmux_interface::source_file")Execute commands from path[split_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.split_window.html "macro tmux_interface::split_window")Create a new pane by splitting target-pane[start_ server](https://docs.rs/tmux_interface/latest/tmux_interface/macro.start_server.html "macro tmux_interface::start_server")Start the tmux server, if not already running, without creating any sessions[suspend_ client](https://docs.rs/tmux_interface/latest/tmux_interface/macro.suspend_client.html "macro tmux_interface::suspend_client")Suspend a client by sending SIGTSTP (tty stop)[swap_ pane](https://docs.rs/tmux_interface/latest/tmux_interface/macro.swap_pane.html "macro tmux_interface::swap_pane")Swap two panes[swap_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.swap_window.html "macro tmux_interface::swap_window")This is similar to link-window, except the source and destination windows are swapped[switch_ client](https://docs.rs/tmux_interface/latest/tmux_interface/macro.switch_client.html "macro tmux_interface::switch_client")Structure to switch the current session for client `target-client` to `target-session`[tmux](https://docs.rs/tmux_interface/latest/tmux_interface/macro.tmux.html "macro tmux_interface::tmux")[man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)[unbind_ key](https://docs.rs/tmux_interface/latest/tmux_interface/macro.unbind_key.html "macro tmux_interface::unbind_key")Manual[unlink_ window](https://docs.rs/tmux_interface/latest/tmux_interface/macro.unlink_window.html "macro tmux_interface::unlink_window")Unlink `target-window`[wait_ for](https://docs.rs/tmux_interface/latest/tmux_interface/macro.wait_for.html "macro tmux_interface::wait_for")Manual

----
## Notes / Comments / Lessons

- Collection method: docs.rs crate sitemap discovery with in-page link expansion.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
