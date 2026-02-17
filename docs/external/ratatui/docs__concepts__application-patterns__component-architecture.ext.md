----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/application-patterns/component-architecture
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, application patterns, component architecture
- Summary: If you are interested in a more object oriented approach to organizing TUIs, you can use a
----

Source: https://ratatui.rs/concepts/application-patterns/component-architecture

# Component Architecture

If you are interested in a more object oriented approach to organizing TUIs, you can use a
`Component` based approach.

A couple of projects in the wild use this approach

- [https://github.com/TaKO8Ki/gobang](https://github.com/TaKO8Ki/gobang)

- [https://github.com/nomadiz/edma](https://github.com/nomadiz/edma)

We also have a `component` template that has an example of this `Component` based approach:

- [https://github.com/ratatui/templates/tree/main/component](https://github.com/ratatui/templates/tree/main/component)

We already covered [TEA](../the-elm-architecture/) in the previous section. The `Component`
architecture takes a slightly more object oriented trait based approach.

Each component encapsulates its own state, event handlers, and rendering logic.

- Component Initialization (`init`) - This is where a component can set up any initial state or resources it needs. It’s a separate process from handling events or rendering.

- Event Handling (`handle_events`, `handle_key_events`, `handle_mouse_events`) - Each component has its own event handlers. This allows for a finer-grained approach to event handling, with each component only dealing with the events it’s interested in. This contrasts with Elm’s single update function that handles messages for the entire application.

- State Update (`update`) - Components can have their own local state and can update it in response to actions. This state is private to the component, which differs from Elm’s global model.

- Rendering (`render`) - Each component defines its own rendering logic. It knows how to draw itself, given a rendering context. This is similar to Elm’s view function but on a component-by-component basis.

Here’s an example of the `Component` trait implementation you might use:

```
use color_eyre::eyre::Result;use ratatui::crossterm::event::{KeyEvent, MouseEvent};use ratatui::layout::Rect;
use crate::{action::Action, event::Event, terminal::Frame};
pub trait Component {  fn init(&#x26;mut self) -> Result&#x3C;()> {    Ok(())  }  fn handle_events(&#x26;mut self, event: Option&#x3C;Event>) -> Action {    match event {      Some(Event::Quit) => Action::Quit,      Some(Event::Tick) => Action::Tick,      Some(Event::Key(key_event)) => self.handle_key_events(key_event),      Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),      Some(Event::Resize(x, y)) => Action::Resize(x, y),      Some(_) => Action::Noop,      None => Action::Noop,    }  }  fn handle_key_events(&#x26;mut self, key: KeyEvent) -> Action {    Action::Noop  }  fn handle_mouse_events(&#x26;mut self, mouse: MouseEvent) -> Action {    Action::Noop  }  fn update(&#x26;mut self, action: Action) -> Action {    Action::Noop  }  fn render(&#x26;mut self, f: &#x26;mut Frame, rect: Rect);}
```

One advantage of this approach is that it incentivizes co-locating the `handle_events`, `update` and
`render` functions on a component level.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/application-patterns/component-architecture.md)

 [Previous The Elm Architecture](/concepts/application-patterns/the-elm-architecture/) [Next Flux Architecture](/concepts/application-patterns/flux-architecture/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
