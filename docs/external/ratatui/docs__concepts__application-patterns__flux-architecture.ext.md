----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/application-patterns/flux-architecture
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, application patterns, flux architecture
- Summary: [Flux](https://facebookarchive.github.io/flux/docs/in-depth-overview/) is a design pattern
----

Source: https://ratatui.rs/concepts/application-patterns/flux-architecture

# Flux Architecture

[Flux](https://facebookarchive.github.io/flux/docs/in-depth-overview/) is a design pattern
introduced by Facebook to address the challenges of building large scale web applications. Though
originally designed with web applications in mind, the Flux architecture can be applied to any
client-side project, including terminal applications. Here’s a real world example of using the
`Flux` architecture with `ratatui`: [https://github.com/Yengas/rust-chat-server/tree/main/tui](https://github.com/Yengas/rust-chat-server/tree/main/tui).

## Why `Flux` for `ratatui`?

[Section titled “Why Flux for ratatui?”](#why-flux-for-ratatui)

Terminal applications often have to deal with complex user interactions, multiple views, and dynamic
data sources. Keeping the application predictable and the logic decoupled is crucial. `Flux`, with
its unidirectional data flow, allows `ratatui` developers to have a structured way to handle user
input, process data, and update the views.

## `Flux` `ratatui` Overview

[Section titled “Flux ratatui Overview”](#flux-ratatui-overview)

### Dispatcher

[Section titled “Dispatcher”](#dispatcher)

The dispatcher remains the central hub that manages all data flow in your application. Every action
in the application, whether it’s a user input or a response from a server, will be channeled through
the dispatcher. This ensures a unified way of handling data, and since the dispatcher has no logic
of its own, it simply ensures that all registered callbacks receive the action data.

- ``` struct Dispatcher { store: Store,} impl Dispatcher { fn dispatch(&#x26;mut self, action: Action) { self.store.update(action); }} ``` ### Stores [Section titled “Stores”](#stores) Stores in Ratatui hold the application’s state and its logic. They could represent things like: A list of items in a menu.

- The content of a text editor or viewer.

- User configurations or preferences.

Stores listen for actions dispatched from the Dispatcher. When a relevant action is dispatched, the
store updates its state and notifies any listening components (or views) that a change has occurred.

```
struct Store {    counter: i32,}
impl Store {    fn new() -> Self {        Self { counter: 0 }    }
    fn update(&#x26;mut self, action: Action) {        match action {            Action::Increment => self.counter += 1,            Action::Decrement => self.counter -= 1,        }    }
    fn get_state(&#x26;self) -> i32 {        self.counter    }}
```

### Actions

[Section titled “Actions”](#actions)

Actions represent any change or event in your application. For instance, when a user presses a key,
selects a menu item, or inputs text, an action is created. This action is dispatched and processed
by the relevant stores, leading to potential changes in application state.

```
enum Action {    Increment,    Decrement,}
```

### Views / Widgets

[Section titled “Views / Widgets”](#views--widgets)

`ratatui`’s widgets display the application’s UI. They don’t hold or manage the application state,
but they display it. When a user interacts with a widget, it can create an action that gets
dispatched, which may lead to a change in a store, which in turn may lead to the widget being
updated.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/application-patterns/flux-architecture.md)

 [Previous Component Architecture](/concepts/application-patterns/component-architecture/) [Next Backends](/concepts/backends/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
