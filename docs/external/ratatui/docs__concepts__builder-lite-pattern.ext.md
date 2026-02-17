----
## External Docs Snapshot // ratatui

- Captured: 2026-02-17T02:49:21.122Z
- Source root: https://ratatui.rs/
- Source page: /concepts/builder-lite-pattern
- Keywords: ratatui, rust, tui, terminal ui, docs, concepts, builder lite pattern
- Summary: In Ratatui, most widgets (and some other objects) use the [Builder Lite](https://matklad.github.io/2022/05/29/builder-lite.html) pattern to set fields. This
----

Source: https://ratatui.rs/concepts/builder-lite-pattern

# Builder Lite Pattern

In Ratatui, most widgets (and some other objects) use the [Builder Lite](https://matklad.github.io/2022/05/29/builder-lite.html) pattern to set fields. This
allows the object to be created in a single shot with methods that setup how the widget will be
displayed, without having to store the widget in a variable and mutate it.

The builder lite pattern consumes the `self` parameter of each method and returns a value with the
updated field. An example of this from Paragraph (and any other widget that supports being
automatically wrapped in a block):

```
#[must_use]pub fn block(mut self, block: Block&#x3C;'a>) -> Self {    self.block = Some(block);    self}
```

Which you might call like:

```
let paragraph = Paragraph::new("foobar").block(Block::bordered())
```

If you’ve reached this page after seeing an error or warning in your app’s compilation, then it’s
likely that you are calling the setter methods against an object, but not storing or using the
result. This will have no effect on the actual display of the widget and is usually a mistake.

E.g. the following code:

```
let text = Text::raw("wrong");text.centered();
```

Should be replaced with:

```
let text = Text::raw("right").centered();
```

Or in situations where you want to reuse a widget’s setup more than once:

```
let text = Text::raw("right");let centered_text = text.clone().centered();let bold_text = text.bold();
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/concepts/builder-lite-pattern.md)

 [Previous Event Handling](/concepts/event-handling/) [Next Using no_std](/concepts/no-std/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
