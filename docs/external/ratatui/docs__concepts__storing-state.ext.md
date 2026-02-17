----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/storing-state
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, storing state
- Summary: This page covers several ways that a programmer can store the state of the application.
----

Source: https://ratatui.rs/concepts/storing-state

# Storing Application State

This page covers several ways that a programmer can store the state of the application.

## Single Silo Method

[Section titled “Single Silo Method”](#single-silo-method)

This is perhaps the easiest method to understand, and works best for small applications that do no
require a large amount of state to be remembered. The idea behind this method is simple: “One struct
for all state”, and whenever a component requires knowledge about the state of the application, it
requests a reference to the `app` state.

This is the method used in the tutorial.

### Pros

[Section titled “Pros”](#pros)

This is conceptually very easy to understand. All of your states are stored in one place, and
passing it to sub-components is simple.

### Cons

[Section titled “Cons”](#cons)

However, you can tell when your application has outgrown the single silo application state when you
begin to write code like this:

```
let selected_item = &#x26;app.states.history.transacts_list.items[app.states.history.transacts_list.state.selected().unwrap()];
```

Another downside to this method, is the lack of multithreaded support. If you begin to use multiple
threads that need access to the application state, access to the `app` can become a bottleneck as
`Mutex` and locks get handed around.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/storing-state.md)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
