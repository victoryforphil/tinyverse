# TUI Components Unification Plan

> Merge `tinyverse_ui` (CLI text output library) and `tinyverse_tui_components`
> (imported dark-factory ratatui component library) into a single canonical
> component surface, eliminating duplicate helpers scattered across `tinyverse_tui`.

**Decision**: dark-factory components (`tinyverse_tui_components`) are the
canonical defaults. Tinyverse-original pieces are adopted only when they add
clear additive value (e.g., `Panel` rendering for CLI text output, `StyledTable`
for non-TUI reports, `Theme` trait for CLI contexts).

## Progress

- Completed: imported and wired `tinyverse_tui_components` into workspace + Moon.
- Completed: crate rename cleanup (`dark_tui_components` -> `tinyverse_tui_components`) in tests/examples/docs.
- Completed: dark-first foundation (`Tone`, `StatusPill::custom`, `StatusPill::for_tone`, `KeyBind::spans`, rect utils).
- Completed: `tinyverse_tui` runtime helper removal and replacement with shared components/utilities.
- Completed: popup migration to shared `PopupOverlay` and detail modal extraction to shared `ModalOverlay`.
- Completed: `tinyverse_tui` theme unification to shared `ComponentTheme` + shared loader usage.
- Completed: module split to `components/primitives` and `components/composites/chat`.
- Verified: `cargo check --workspace` and `cargo test --workspace` passing.

---

## Inventory: Tinyverse Original UI Library (`tinyverse_ui`)

Crate: `tinyverse_ui` — pure-text CLI rendering library (no ratatui dependency).
Depends on `nu-ansi-term`. Consumed by `tinyverse_cli` and `tinyverse_lib`.

| Component / Item | File | Description |
|---|---|---|
| `Tone` enum | `src/theme.rs` | Semantic tones: Neutral, Info, Success, Warning, Error, Muted |
| `Theme` trait | `src/theme.rs` | 10 style methods (section_header, label, tone badge, panel border, etc.) |
| `DefaultTheme` struct | `src/theme.rs` | Colorful ANSI terminal theme impl |
| `MinimalTheme` struct | `src/theme.rs` | Monochrome/plain theme impl |
| `RenderMode` enum | `src/render.rs` | Plain / ANSI output mode |
| `RenderContext` struct | `src/render.rs` | Holds mode + width + theme reference |
| `truncate_with_ellipsis` fn | `src/render.rs` | Text truncation |
| `pad_right` fn | `src/render.rs` | Right-pad to width |
| `visible_width` fn | `src/render.rs` | ANSI-aware character width |
| `terminal_width` fn | `src/render.rs` | Detect terminal columns |
| `Panel` struct | `src/components/panel.rs` | Bordered box with title/tone/padding → `String` |
| `PanelPadding` struct | `src/components/panel.rs` | Configurable padding for Panel |
| `StyledTable` struct | `src/components/styled_table.rs` | Aligned table with headers, stripes, numeric cols → `String` |
| `ColumnAlignment` enum | `src/components/styled_table.rs` | Left/Right alignment |
| `StripeMode` enum | `src/components/styled_table.rs` | None/DimEvenRows |
| `ActionLine` struct | `src/components/action_line.rs` | Toned label + message line |
| `ErrorBlock` struct | `src/components/error_block.rs` | ERROR title + detail + guidance |
| `SectionHeader` struct | `src/components/section_header.rs` | `== TITLE ==` underlined header |
| `LabeledField` struct | `src/components/labeled_field.rs` | `Label: value` bold label |
| `GuidanceLine` struct | `src/components/guidance_line.rs` | Dim guidance hint |
| `StatusBadge` struct | `src/components/status_badge.rs` | Tone-colored badge |
| `DetailSection` struct | `src/components/detail_section.rs` | Grouped detail block |
| `SummaryFooter` struct | `src/components/summary_footer.rs` | Footer summary line |
| `format_display_name` fn | `src/naming.rs` | Session name → display title |
| `example_outputs` module | `src/example_outputs.rs` | Gallery/demo renderers for tests |

**Key characteristics**:
- All renderers return `String` (no ratatui Frame/Rect)
- ANSI via `nu-ansi-term` (not ratatui `Style`)
- Exclusively used by CLI commands (list, spawn, kill, config, view, etc.)
- `format_display_name` also used by `tinyverse_lib::session_select`

---

## Inventory: Generic Helpers in `tinyverse_tui` to Extract

Crate: `tinyverse_tui` — the TUI application. Contains generic helpers in
`src/runtime/helpers.rs` and `src/theme.rs` that should live in the shared lib.

| Item | File | Description | Migration Target |
|---|---|---|---|
| `styled_panel(title, focused, theme) → Block` | `src/runtime/helpers.rs` | Themed bordered panel (rounds, bold title) | Replace with `PaneBlockComponent::build` + title_style |
| `status_pill(status, theme) → Span` | `src/runtime/helpers.rs` | Semantic status pill (active/stale/dead) | Already delegates to `StatusPill::new` |
| `tag_pill(label, theme) → Span` | `src/runtime/helpers.rs` | Accent pill badge | Replace with `StatusPill::accent` |
| `pill_badge(label, fg, bg, bold) → Span` | `src/runtime/helpers.rs` | Generic pill with custom colors | Merge into `StatusPill::custom` |
| `centered_rect(w, h, area) → Rect` | `src/runtime/helpers.rs` | Center a fixed-size rect in area | Already in dark `utils/rect.rs` via `inner_rect` / use `PopupAnchor::Center` |
| `anchored_rect(w, h, x, y, bounds) → Rect` | `src/runtime/helpers.rs` | Anchor a rect with bounds clamping | Already in dark `PopupOverlay::area` logic |
| `inset_rect(area, h, v) → Rect` | `src/runtime/helpers.rs` | Shrink rect by margins | Already in dark `utils/rect.rs::inner_rect` |
| `key_hint(key, action, theme) → Vec<Span>` | `src/runtime/helpers.rs` | Styled key+action hint spans | Replace with `KeyHintBar` or add `KeyBind::spans(theme)` |
| `truncate_to(value, max) → String` | `src/runtime/helpers.rs` | Truncate with `…` | Already in dark `utils/compact.rs::compact_label` |
| `rect_contains(rect, x, y) → bool` | `src/runtime/helpers.rs` | Point-in-rect test | Already in dark `utils/rect.rs::rect_contains` |
| `UiTheme` struct (44 fields) | `src/theme.rs` | Full ratatui Color palette (superset of `ComponentTheme`) | Merge into `ComponentTheme` as extended theme |
| `load_theme()` fn | `src/theme.rs` | TOML file loading + override cascade | Move to `tinyverse_tui_components` or keep in TUI app |
| `parse_color` / `unquote` fns | `src/theme.rs` | Color string parsing | Move alongside `load_theme` |

**Duplicated popup rendering**: `tinyverse_tui/src/runtime/chat_render/popups.rs`
reimplements popup list layout, viewport, query/hint rows — nearly identical to
`tinyverse_tui_components::PopupOverlay` but with `App`-coupled layout caching.
The `detail_modal.rs` could be generalized as a `ModalOverlay` component.

---

## Inventory: Imported Dark Components (`tinyverse_tui_components`)

Crate: `tinyverse_tui_components` — ratatui-native shared components.
Depends on `ratatui 0.29`, `crossterm 0.29`, `pulldown-cmark`, `tokio[sync]`.

### Core Framework

| Item | File | Type |
|---|---|---|
| `Component` trait | `src/component.rs` | Lifecycle trait: init, events, draw, focus |
| `ComponentResult<T>` | `src/component.rs` | Error type alias |
| `DynComponent` | `src/component.rs` | `Box<dyn Component>` alias |
| `Action` enum | `src/action.rs` | Tick, Render, Resize, Quit, Focus, Scroll, Select, Confirm, Cancel, Input, Cursor, Custom, etc. |
| `Event` enum | `src/event.rs` | Key, Mouse, Tick, Resize wrappers |
| `ComponentTheme` struct | `src/theme.rs` | 20-field Color palette (pills, keys, panes, text) |
| `ComponentThemeLike` trait | `src/theme.rs` | Getter contract for theme fields |

### Primitive Components

| Component | File | Description |
|---|---|---|
| `StatusPill` | `src/components/status_pill.rs` | Colored badge `" label "` with tone constructors (ok/warn/error/info/muted/accent) + `span()`/`span_compact()` |
| `LabeledField` | `src/components/labeled_field.rs` | `label: value` line with padding variants |
| `SectionHeader` | `src/components/section_header.rs` | `TITLE ───────` with accent + rule fill |
| `LoadingSpinner` | `src/components/loading_spinner.rs` | Time-based rotating `- \ | /` glyph |
| `KeyBind` / `KeyHintBar` | `src/components/key_hint_bar.rs` | Styled key hint bar with separator and line wrapping |
| `FooterBar` / `FooterBarProps` | `src/components/footer_bar.rs` | Segment-joined footer bar with separators |
| `PaneBlockComponent` | `src/components/pane_block_component.rs` | `Block::build(title, focused, theme)` bordered pane |

### Composite Components

| Component | File | Description |
|---|---|---|
| `CardGridComponent` | `src/components/card_grid_component.rs` | Responsive 1-3 col grid of bordered cards with selection + paging |
| `PopupOverlay` / `PopupOverlayProps` / `PopupItem` / `PopupAnchor` / `PopupHit` | `src/components/popup_overlay.rs` | Generic popup list with query row, hint row, hit-testing, viewport |
| `ChatComposerComponent` / `ChatComposerProps` | `src/components/chat_composer.rs` | Composer hint/draft renderer with cursor |
| `ChatConversationHeaderComponent` / `ChatConversationHeaderProps` / `ChatStatusTone` | `src/components/chat_conversation_header.rs` | Title + subtitle + status pill header |
| `ChatMessageListComponent` / `ChatMessageListProps` / `ChatPalette` | `src/components/chat_message_list.rs` | Full markdown-aware message transcript with scroll viewport |
| `ChatMessageEntry` / `ChatMessageRole` | `src/components/chat_types.rs` | Message data model (role, text, timestamp) |

### Layout Utilities

| Utility | File | Description |
|---|---|---|
| `rect_contains(Rect, col, row)` | `src/utils/rect.rs` | Point-in-rect test |
| `inner_rect(Rect)` | `src/utils/rect.rs` | Shrink by 1 on each side |
| `with_cursor_tail(&str)` | `src/utils/rect.rs` | Append `▐` cursor |
| `ThreePanePercents` | `src/utils/resizable.rs` | Three-way percentage split with min constraints |
| `percent_from_left/right_edge` | `src/utils/resizable.rs` | Pointer-to-percent conversion |
| `HorizontalSplit` | `src/utils/split_layout.rs` | N-way resizable horizontal layout with divider hit-test and pointer resize |
| `ListViewport` | `src/utils/viewport.rs` | Scrollable list viewport with anchored selection |

### Text Utilities

| Utility | File | Description |
|---|---|---|
| `compact_text/id/locator/timestamp/session_id/label/tail/normalized` | `src/utils/compact.rs` | Various truncation/shortening functions |
| `previous_index` / `next_index` | `src/utils/index.rs` | Wraparound index navigation |

---

## Canonical Direction: Dark-First

### Principle

> When dark-factory and tinyverse have equivalent components, **dark wins** as
> canonical. Tinyverse additions are merged into the dark component when they
> provide clear additive value without increasing API surface unnecessarily.

### Specific Decisions

| Area | Winner | Rationale |
|---|---|---|
| **Theme system (ratatui)** | Dark `ComponentThemeLike` trait | Clean trait contract; extend with tinyverse's extra fields |
| **Theme loading (TOML)** | Tinyverse `load_theme` | Dark has no file loader; this is additive |
| **StatusPill** | Dark | Already canonical; tinyverse helpers already delegate |
| **LabeledField (ratatui)** | Dark | Cleaner API with compact variant |
| **SectionHeader (ratatui)** | Dark | Width-aware rule fill vs tinyverse's static `==` |
| **KeyHintBar** | Dark | Wrapping, separator config, batch rendering |
| **key_hint (single)** | Merge | Add `KeyBind::spans()` for single-hint use case |
| **PaneBlockComponent** | Dark → extend | Add tinyverse's `title_style(text_primary, BOLD)` as default |
| **styled_panel** | Remove | Thin wrapper, inline into `PaneBlockComponent::build` |
| **Rect utilities** | Dark | Already equivalent; remove tinyverse_tui duplicates |
| **compact/truncate** | Dark | Richer truncation set |
| **PopupOverlay** | Dark | Generic + hit-testing; tinyverse_tui popup code is app-coupled duplicate |
| **CardGrid** | Dark | Only impl; already used |
| **Chat components** | Dark | Shared chat primitives (composer, header, message list) |
| **HorizontalSplit / resizable** | Dark | Only impl of resizable layout with pointer drag |
| **ListViewport** | Dark | Anchored viewport; tinyverse_tui has inline reimpl |
| **Panel (CLI text output)** | Tinyverse (keep) | CLI-only text renderer, no ratatui equivalent needed |
| **StyledTable (CLI text)** | Tinyverse (keep) | CLI-only text table, no ratatui equivalent needed |
| **ActionLine / ErrorBlock** | Tinyverse (keep) | CLI-only error/action rendering |
| **format_display_name** | Tinyverse (keep) | Session name formatting, not UI-specific |
| **Theme (CLI/ANSI)** | Tinyverse (keep) | `nu-ansi-term` theme for CLI text output |
| **Detail modal** | New shared component | Extract from tinyverse_tui into `ModalOverlay` |
| **Spinner** | Dark `LoadingSpinner` | Only impl |
| **pill_badge (custom colors)** | Merge | Add `StatusPill::custom(label, fg, bg, bold)` |
| **Component trait** | Dark (keep as-is) | App-level lifecycle, not needed by all consumers |

---

## Proposed Unified Module Architecture

### `tinyverse_tui_components` (the canonical shared ratatui component library)

```
tinyverse_tui_components/src/
├── lib.rs                          # Re-exports
├── action.rs                       # Action enum (unchanged)
├── component.rs                    # Component trait (unchanged)
├── event.rs                        # Event enum (unchanged)
├── theme/
│   ├── mod.rs                      # ComponentTheme, ComponentThemeLike, Tone enum
│   ├── palette.rs                  # Extended palette fields (chat colors, card bg, etc.)
│   └── loader.rs                   # TOML theme loading (from tinyverse_tui::theme)
├── primitives/
│   ├── mod.rs
│   ├── status_pill.rs              # StatusPill (+ custom constructor)
│   ├── labeled_field.rs            # LabeledField
│   ├── section_header.rs           # SectionHeader
│   ├── key_hint_bar.rs             # KeyBind + KeyHintBar (+ KeyBind::spans())
│   ├── footer_bar.rs               # FooterBar
│   ├── pane_block.rs               # PaneBlockComponent (with title_style default)
│   ├── loading_spinner.rs          # LoadingSpinner
│   └── modal_overlay.rs            # NEW: ModalOverlay (from detail_modal.rs)
├── composites/
│   ├── mod.rs
│   ├── card_grid.rs                # CardGridComponent
│   ├── popup_overlay.rs            # PopupOverlay + types
│   └── chat/
│       ├── mod.rs
│       ├── types.rs                # ChatMessageEntry, ChatMessageRole
│       ├── composer.rs             # ChatComposerComponent
│       ├── conversation_header.rs  # ChatConversationHeaderComponent
│       └── message_list.rs         # ChatMessageListComponent
├── utils/
│   ├── mod.rs
│   ├── compact.rs                  # Text truncation fns
│   ├── index.rs                    # Wraparound index fns
│   ├── rect.rs                     # rect_contains, inner_rect, inset_rect, centered_rect, anchored_rect, with_cursor_tail
│   ├── resizable.rs                # ThreePanePercents, percent_from_*_edge
│   ├── split_layout.rs             # HorizontalSplit
│   └── viewport.rs                 # ListViewport
```

### `tinyverse_ui` (CLI text output library — unchanged role)

Stays as-is. It serves CLI commands and `tinyverse_lib` with plain/ANSI text
rendering. No ratatui dependency. No merge into `tinyverse_tui_components`.

```
tinyverse_ui/src/
├── lib.rs
├── theme.rs                        # Tone, Theme trait, DefaultTheme, MinimalTheme
├── render.rs                       # RenderMode, RenderContext, text utils
├── naming.rs                       # format_display_name
├── components/                     # CLI text components (Panel, StyledTable, etc.)
└── example_outputs.rs
```

### `tinyverse_tui` (application — consuming both)

After migration, `tinyverse_tui/src/runtime/helpers.rs` becomes a thin shim or
is deleted. Popup/detail rendering delegates to shared components.

---

## Migration Strategy / Phases

### Phase 0: Foundation (theme + utils)

1. Extend `ComponentThemeLike` trait with tinyverse_tui's extra fields
   (text_primary, selected_card_bg, chat_* colors, path_pill_*).
2. Add `Tone` enum to dark theme module (mirrors tinyverse_ui's Tone for
   semantic pill construction: `StatusPill::for_tone(tone, theme)`).
3. Move `centered_rect`, `anchored_rect`, `inset_rect` into dark `utils/rect.rs`.
4. Move TOML theme loader into dark crate as `theme::loader`.
5. Add `StatusPill::custom(label, fg, bg, bold)` constructor.
6. Add `KeyBind::spans(theme) → Vec<Span>` for single key-hint rendering.
7. Restructure dark `src/components/` → `src/primitives/` + `src/composites/`.
8. Restructure dark `src/theme.rs` → `src/theme/mod.rs` + `palette.rs`.

### Phase 1: Consumer Migration (`tinyverse_tui`)

9. Make `UiTheme` implement `ComponentThemeLike` (bridge impl).
10. Replace `tinyverse_tui::helpers::styled_panel` calls with `PaneBlockComponent::build`.
11. Replace `helpers::status_pill/tag_pill/pill_badge` with `StatusPill` methods.
12. Replace `helpers::key_hint` with `KeyBind::spans`.
13. Replace `helpers::centered_rect/anchored_rect/inset_rect/rect_contains` with dark utils.
14. Replace `helpers::truncate_to` with `compact_label`.
15. Replace inline popup rendering (`chat_render/popups.rs`) with `PopupOverlay`.
16. Extract detail modal into shared `ModalOverlay` component.
17. Delete `tinyverse_tui/src/runtime/helpers.rs` (or reduce to app-specific).

### Phase 2: Polish + Validation

18. Verify all snapshot tests pass (insta review).
19. Run full TUI manually to confirm visual parity.
20. Remove any remaining dead code.
21. Update Cargo.toml dependencies (remove unused).
22. Final `@tui_designer` review pass.

---

## Parallelizable TODO Checklist

Each task is independent and can be assigned to a `developer_jr` worker.
Dependencies are noted where they exist; truly parallel tasks have no deps.

---

### T01: Add `Tone` enum to dark theme module

**Scope**: Add a semantic `Tone` enum to `tinyverse_tui_components::theme`
that maps to pill constructors.

**Files**:
- `tinyverse_tui_components/src/theme.rs` (or new `src/theme/mod.rs`)

**Work**:
- Add `pub enum Tone { Ok, Warn, Error, Info, Muted, Accent }`
- Add `StatusPill::for_tone(tone: Tone, label, theme) → Self` factory
- Add tests

**Acceptance**: `cargo test -p tinyverse_tui_components` passes. `Tone` is
re-exported from `lib.rs`.

**Deps**: None

---

### T02: Add `StatusPill::custom` constructor

**Scope**: Add a custom-color pill constructor to `StatusPill`.

**Files**:
- `tinyverse_tui_components/src/components/status_pill.rs`

**Work**:
- Add `pub fn custom(label: impl Into<String>, fg: Color, bg: Color, bold: bool) → Self`
- Adjust `span()` to apply bold modifier when flagged
- Add bold field to struct (default false for existing constructors)

**Acceptance**: Tests pass. Replaces need for `pill_badge` in tinyverse_tui.

**Deps**: None

---

### T03: Add `KeyBind::spans` method

**Scope**: Add a method to `KeyBind` that returns individual spans for inline
key-hint rendering (single hint, not full bar).

**Files**:
- `tinyverse_tui_components/src/components/key_hint_bar.rs`

**Work**:
- Add `pub fn spans(&self, theme: &impl ComponentThemeLike) → Vec<Span<'static>>`
- Returns `[key_span, space, action_span]` matching existing bar style
- Add test

**Acceptance**: Tests pass. `KeyBind::new("esc", "close").spans(&theme)` returns
styled spans equivalent to tinyverse_tui's `key_hint()`.

**Deps**: None

---

### T04: Move `centered_rect` to dark utils

**Scope**: Add `centered_rect(width, height, area) → Rect` to dark `utils/rect.rs`.

**Files**:
- `tinyverse_tui_components/src/utils/rect.rs`

**Work**:
- Copy `centered_rect` from `tinyverse_tui/src/runtime/helpers.rs`
- Re-export from `lib.rs`
- Add unit tests

**Acceptance**: `cargo test -p tinyverse_tui_components` passes. Function
re-exported at crate root.

**Deps**: None

---

### T05: Move `anchored_rect` to dark utils

**Scope**: Add `anchored_rect(width, height, x, y, bounds) → Rect` to dark utils.

**Files**:
- `tinyverse_tui_components/src/utils/rect.rs`

**Work**:
- Copy from `tinyverse_tui/src/runtime/helpers.rs`
- Re-export from `lib.rs`
- Add unit tests

**Acceptance**: Tests pass.

**Deps**: None

---

### T06: Move `inset_rect` to dark utils

**Scope**: Add `inset_rect(area, horizontal, vertical) → Rect` to dark utils.

**Files**:
- `tinyverse_tui_components/src/utils/rect.rs`

**Work**:
- Copy from `tinyverse_tui/src/runtime/helpers.rs`
- Re-export from `lib.rs`
- Add unit tests (including zero-width/height edge cases)

**Acceptance**: Tests pass.

**Deps**: None

---

### T07: Extend `ComponentThemeLike` with `text_primary`

**Scope**: Add `text_primary` field to the dark theme contract.

**Files**:
- `tinyverse_tui_components/src/theme.rs`

**Work**:
- Add `text_primary: Color` to `ComponentTheme` (default: `Color::Rgb(232, 228, 220)`)
- Add `fn text_primary(&self) → Color` to `ComponentThemeLike` trait
- Implement in `ComponentTheme`

**Acceptance**: All existing tests pass. `PaneBlockComponent::build` can use
`text_primary` for title style.

**Deps**: None

---

### T08: Extend `ComponentThemeLike` with `selected_card_bg`

**Scope**: Add selection background color to theme.

**Files**:
- `tinyverse_tui_components/src/theme.rs`

**Work**:
- Add `selected_card_bg: Color` field + trait getter
- Default: `Color::Rgb(28, 30, 30)`
- Implement

**Acceptance**: Tests pass. `CardGridComponent` can use for selected card bg.

**Deps**: None

---

### T09: Extend `ComponentThemeLike` with chat palette fields

**Scope**: Add chat-specific color fields to the dark theme.

**Files**:
- `tinyverse_tui_components/src/theme.rs`

**Work**:
- Add fields: `chat_separator_fg`, `chat_header_user_bg`, `chat_header_agent_bg`,
  `chat_header_system_bg`, `chat_collapsible_bg`, `chat_collapsible_focused_bg`,
  `chat_collapsible_tag_bg`, `chat_code_bg`, `path_pill_fg`, `path_pill_bg`
- Add trait getters + struct defaults (from `tinyverse_tui::UiTheme::default()`)
- Implement

**Acceptance**: Tests pass. `ChatMessageListComponent` can reference chat colors
via theme trait.

**Deps**: None

---

### T10: Enhance `PaneBlockComponent::build` with title style

**Scope**: Add `text_primary` + bold title styling to pane blocks (matching
tinyverse_tui's `styled_panel`).

**Files**:
- `tinyverse_tui_components/src/components/pane_block_component.rs`

**Work**:
- Use `theme.text_primary()` + `Modifier::BOLD` for `.title_style(…)` when
  available
- Add rounded border type (matching tinyverse `BorderType::Rounded`)
- Keep existing `focused/unfocused` border color logic

**Acceptance**: Visual output matches tinyverse_tui's `styled_panel`. Tests pass.

**Deps**: T07 (needs `text_primary`)

---

### T11: Implement `UiTheme → ComponentThemeLike` bridge

**Scope**: Make tinyverse_tui's `UiTheme` implement the dark `ComponentThemeLike`
trait so all shared components can accept it.

**Files**:
- `tinyverse_tui/src/theme.rs`

**Work**:
- Add `impl ComponentThemeLike for UiTheme { … }` mapping the 20 base fields
  (pill_*, key_hint_*, pane_*, text_secondary, text_muted)
- Extend with new fields from T07-T09 if those are done; use defaults otherwise

**Acceptance**: `cargo check -p tinyverse_tui` passes. Shared components can
accept `&app.theme` directly.

**Deps**: None (base trait); T07/T08/T09 for extended fields

---

### T12: Replace `helpers::status_pill` with `StatusPill`

**Scope**: Remove `tinyverse_tui::helpers::status_pill` and use dark `StatusPill`
directly at call sites.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs` (remove fn)
- `tinyverse_tui/src/runtime/render.rs` (update call sites)
- Any other callers in `tinyverse_tui/src/runtime/`

**Work**:
- Find all `status_pill(…)` calls
- Replace with `StatusPill::ok/warn/error/muted(…, &theme).span()` or
  `StatusPill::custom(…).span()` for status-to-tone mapping
- Remove the function from helpers.rs

**Acceptance**: `cargo check -p tinyverse_tui`. Visual parity in TUI.

**Deps**: T11 (UiTheme must impl ComponentThemeLike)

---

### T13: Replace `helpers::tag_pill` with `StatusPill::accent`

**Scope**: Remove `tag_pill` wrapper.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- All callers in `tinyverse_tui/src/runtime/`

**Work**:
- Replace `tag_pill(label, &theme)` with `StatusPill::accent(label, &theme).span()`
- Remove function

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: T11

---

### T14: Replace `helpers::pill_badge` with `StatusPill::custom`

**Scope**: Remove `pill_badge` wrapper.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- All callers

**Work**:
- Replace with `StatusPill::custom(label, fg, bg, bold).span()`
- Remove function

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: T02, T11

---

### T15: Replace `helpers::styled_panel` with `PaneBlockComponent::build`

**Scope**: Remove `styled_panel` wrapper.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- `tinyverse_tui/src/runtime/render.rs`
- `tinyverse_tui/src/runtime/chat_render/messages.rs`

**Work**:
- Replace `styled_panel(title, focused, &theme)` with
  `PaneBlockComponent::build(title, focused, &theme)`
- Verify rounded borders + bold title match

**Acceptance**: `cargo check -p tinyverse_tui`. Visual parity.

**Deps**: T10, T11

---

### T16: Replace `helpers::key_hint` with `KeyBind::spans`

**Scope**: Remove `key_hint` wrapper.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- All callers in render.rs, detail_modal.rs, etc.

**Work**:
- Replace `key_hint("esc", "close", &theme)` with
  `KeyBind::new("esc", "close").spans(&theme)`
- Remove function

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: T03, T11

---

### T17: Replace rect helpers with dark utils

**Scope**: Remove duplicate rect functions from tinyverse_tui helpers.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- All callers

**Work**:
- Replace `centered_rect` → `tinyverse_tui_components::centered_rect`
- Replace `anchored_rect` → `tinyverse_tui_components::anchored_rect`
- Replace `inset_rect` → `tinyverse_tui_components::inset_rect`
- Replace `rect_contains` → `tinyverse_tui_components::rect_contains`
- Remove functions from helpers.rs

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: T04, T05, T06

---

### T18: Replace `helpers::truncate_to` with `compact_label`

**Scope**: Remove duplicate truncation.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs`
- All callers

**Work**:
- Replace `truncate_to(val, max)` with `compact_label(val, max)`
- Note: verify ellipsis character matches (`…` vs `...`)
- Remove function

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: None

---

### T19: Migrate popup rendering to `PopupOverlay`

**Scope**: Replace tinyverse_tui's hand-rolled popup rendering with the shared
`PopupOverlay` component.

**Files**:
- `tinyverse_tui/src/runtime/chat_render/popups.rs` (rewrite)

**Work**:
- Replace `PopupConfig` / `render_popup` / `popup_rect` / `list_viewport` with
  `PopupOverlayProps` + `PopupOverlay::render`
- Map model/agent/autocomplete configs to `PopupOverlayProps` structs
- Wire layout cache rects from `PopupOverlay::area()` return
- Wire hit-test from `PopupOverlay::hit_test()` in events.rs
- Remove `PopupLayoutTarget`, local `PopupConfig`, `popup_rect`, `list_viewport`

**Acceptance**: `cargo test -p tinyverse_tui`. Popup visuals unchanged. Mouse
interactions work (hit-test).

**Deps**: T11 (theme bridge)

---

### T20: Extract `ModalOverlay` component from `detail_modal.rs`

**Scope**: Create a generic `ModalOverlay` component in dark crate, then use it
in tinyverse_tui.

**Files**:
- NEW: `tinyverse_tui_components/src/components/modal_overlay.rs`
- `tinyverse_tui_components/src/components/mod.rs`
- `tinyverse_tui_components/src/lib.rs`

**Work**:
- Create `ModalOverlayProps { title, meta_lines, body_lines, footer_hints, scroll_offset }`
- Create `ModalOverlay::render(frame, parent, props, theme)` with:
  - Centered rect (nearly full area)
  - Clear + bordered block with title
  - Meta header section
  - Divider rule
  - Scrollable body
  - Key hint footer
- Re-export from lib.rs

**Acceptance**: `cargo test -p tinyverse_tui_components`. Component renders
correctly in isolation test.

**Deps**: T04 (centered_rect), T07 (text_primary)

---

### T21: Use `ModalOverlay` in tinyverse_tui detail modal

**Scope**: Replace inline detail_modal rendering with shared `ModalOverlay`.

**Files**:
- `tinyverse_tui/src/runtime/chat_render/detail_modal.rs`

**Work**:
- Convert `render_chat_detail_modal` to build `ModalOverlayProps` from app state
- Call `ModalOverlay::render(frame, parent, props, &app.theme)`
- Keep `detail_lines_for_part_key` and `render_detail_body` as app-specific
  helpers that produce `Vec<Line>`

**Acceptance**: `cargo check -p tinyverse_tui`. Detail modal visuals unchanged.

**Deps**: T20, T11

---

### T22: Restructure dark components into primitives/composites dirs

**Scope**: Move component files into logical subdirectories.

**Files**:
- `tinyverse_tui_components/src/components/` → split into `primitives/` + `composites/`
- Update `mod.rs` files and `lib.rs` re-exports

**Work**:
- Create `src/primitives/` dir: move status_pill, labeled_field, section_header,
  key_hint_bar, footer_bar, pane_block, loading_spinner, modal_overlay
- Create `src/composites/` dir: move card_grid, popup_overlay
- Create `src/composites/chat/` dir: move chat_types, chat_composer,
  chat_conversation_header, chat_message_list
- Update all `mod.rs`, `use` paths, and `lib.rs` re-exports
- Ensure public API unchanged (same items re-exported from crate root)

**Acceptance**: `cargo test -p tinyverse_tui_components`. `cargo check -p tinyverse_tui`.
No public API changes visible to consumers.

**Deps**: T20 (modal_overlay should exist first)

---

### T23: Restructure dark theme into submodule

**Scope**: Split `theme.rs` into `theme/mod.rs` + `theme/palette.rs`.

**Files**:
- `tinyverse_tui_components/src/theme.rs` → `src/theme/mod.rs`
- NEW: `tinyverse_tui_components/src/theme/palette.rs`

**Work**:
- Move `ComponentTheme` struct + `Default` impl to `palette.rs`
- Keep `ComponentThemeLike` trait in `mod.rs`
- Keep re-exports identical

**Acceptance**: `cargo test -p tinyverse_tui_components`. No public API changes.

**Deps**: None (can happen early or late)

---

### T24: Move theme TOML loader to dark crate

**Scope**: Move `load_theme`, `parse_color`, `unquote`, `candidate_paths` from
tinyverse_tui into dark crate's `theme/loader.rs`.

**Files**:
- NEW: `tinyverse_tui_components/src/theme/loader.rs`
- `tinyverse_tui/src/theme.rs` (simplify to re-export)

**Work**:
- Move loader functions to dark `theme/loader.rs`
- Generalize: accept `theme_dirs: &[PathBuf]` instead of hardcoding tinyverse paths
- tinyverse_tui calls `load_theme_from_dirs(candidate_paths())` where
  `candidate_paths()` stays in tinyverse_tui (uses tinyverse_lib)
- Re-export `load_theme_from_dirs`, `parse_color` from dark lib

**Acceptance**: `cargo test -p tinyverse_tui_components`. `cargo check -p tinyverse_tui`.
Theme loading behavior unchanged.

**Deps**: T23 (theme submodule), T07/T08/T09 (extended fields)

---

### T25: Delete `tinyverse_tui/src/runtime/helpers.rs`

**Scope**: Remove helpers.rs after all functions have been migrated or inlined.

**Files**:
- `tinyverse_tui/src/runtime/helpers.rs` (delete)
- `tinyverse_tui/src/runtime/mod.rs` or wherever it's declared (remove `mod helpers`)
- All remaining `use crate::runtime::helpers::*` imports (update)

**Work**:
- Verify no functions remain
- Delete file
- Clean up imports

**Acceptance**: `cargo check -p tinyverse_tui`. No references to `helpers::`.

**Deps**: T12, T13, T14, T15, T16, T17, T18 (all helper replacements done)

---

### T26: Remove inline `list_viewport` from chat popups

**Scope**: Delete the inline viewport function that duplicates `ListViewport`.

**Files**:
- `tinyverse_tui/src/runtime/chat_render/popups.rs`

**Work**:
- Replace `list_viewport(total, visible, selected)` calls with
  `ListViewport::new(total, visible, selected)` and use `.start`/`.end`
- Delete local `list_viewport` function

**Acceptance**: `cargo check -p tinyverse_tui`.

**Deps**: Part of T19 (popup migration) or independent if popups not fully migrated

---

### T27: Add component examples for new components

**Scope**: Update `examples/components_preview.rs` with new ModalOverlay and
extended StatusPill demos.

**Files**:
- `tinyverse_tui_components/examples/components_preview.rs`

**Work**:
- Add `StatusPill::custom` + `StatusPill::for_tone` demos
- Add `ModalOverlay` demo
- Add `KeyBind::spans` demo

**Acceptance**: `cargo run -p tinyverse_tui_components --example components_preview` runs.

**Deps**: T01, T02, T03, T20

---

### T28: Audit and update snapshot tests

**Scope**: Run insta snapshot review for both crates after migration.

**Files**:
- `tinyverse_tui_components/tests/`
- `tinyverse_tui/src/runtime/snapshots/`

**Work**:
- Run `scripts/insta_review.sh.ts -p tinyverse_tui_components`
- Run `scripts/insta_review.sh.ts -p tinyverse_tui`
- Review diffs, accept intentional changes
- Fix any regressions

**Acceptance**: All snapshot tests pass. No unintended visual changes.

**Deps**: All prior tasks complete

---

### T29: Verify full TUI manual smoke test

**Scope**: Manual verification that the TUI works end-to-end after migration.

**Files**: None (manual testing)

**Work**:
- Run `bun scripts/dev.sh.ts` or `cargo run -p tinyverse_cli -- tui`
- Verify: pane borders, pills, key hints, popups, chat messages, detail modal,
  card grid, resizable splits, mouse interactions, theme colors
- Note any regressions

**Acceptance**: No visual or functional regressions.

**Deps**: All prior tasks complete

---

### T30: Clean up Cargo.toml dependencies

**Scope**: Remove unused dependencies after migration.

**Files**:
- `tinyverse_tui/Cargo.toml`
- `tinyverse_tui_components/Cargo.toml`

**Work**:
- Check if `tinyverse_tui_components` needs `tinyverse_lib` dep (for theme loader)
- Verify no unused deps remain
- Run `cargo check --workspace`

**Acceptance**: Clean build, no warnings about unused deps.

**Deps**: T24

---

## Task Dependency Graph (Parallelization Guide)

```
Independent (can all run in parallel):
  T01, T02, T03, T04, T05, T06, T07, T08, T09, T11, T18, T23

After T07:
  T10 → T15
  T20 → T21

After T04 + T05 + T06:
  T17

After T03:
  T16

After T02:
  T14

After T11:
  T12, T13, T15, T16, T19, T21

After T12 + T13 + T14 + T15 + T16 + T17 + T18:
  T25

After T23 + T07 + T08 + T09:
  T24

After T20:
  T22

After T01 + T02 + T03 + T20:
  T27

After all:
  T28, T29, T30
```

**Maximum parallelism**: 12 tasks (T01–T09, T11, T18, T23) can start immediately.

---

## Risks and Guardrails

| Risk | Mitigation |
|---|---|
| Breaking `ComponentThemeLike` trait (new required methods) | Add methods with default impls returning sensible fallbacks so existing impls don't break |
| Visual regressions in TUI | Snapshot tests + manual smoke test (T28, T29) |
| PopupOverlay API mismatch with tinyverse_tui's layout caching | PopupOverlay already returns computed `Rect` from `area()` — cache that |
| Chat message rendering differences | tinyverse_tui uses richer part-based rendering (collapsible, tool calls); dark `ChatMessageListComponent` uses markdown — these serve different apps. Don't force-merge chat rendering; keep tinyverse_tui's `chat_render/` modules for now |
| Theme TOML loader depends on `tinyverse_lib` paths | Keep path resolution in tinyverse_tui; only move parser/applicator to dark |
| `nu-ansi-term` vs `ratatui::Style` confusion | These are separate worlds: `tinyverse_ui` = CLI text output (nu-ansi-term), `tinyverse_tui_components` = ratatui. Never mix. |
| Circular deps | `tinyverse_tui_components` must NOT depend on `tinyverse_lib` or `tinyverse_tui`. Theme loader must accept generic path args. |
| Module restructure (T22) breaks imports | Do restructure last within phase, use `pub use` re-exports to keep crate-root API stable |
| `ComponentThemeLike` trait grows too large | Consider splitting into `ComponentThemeLike` (base) + `ChatThemeLike` extension trait if it exceeds ~30 methods |

---

## Validation Commands

```bash
# Build check (both crates)
cargo check -p tinyverse_tui_components
cargo check -p tinyverse_tui
cargo check -p tinyverse_ui

# Full workspace check
bun scripts/check.sh.ts

# Tests
bun scripts/test.sh.ts
# or individually:
cargo nextest run -p tinyverse_tui_components
cargo nextest run -p tinyverse_tui
cargo nextest run -p tinyverse_ui

# Snapshot review
scripts/insta_review.sh.ts -p tinyverse_tui_components
scripts/insta_review.sh.ts -p tinyverse_tui

# Example preview
cargo run -p tinyverse_tui_components --example components_preview

# Manual TUI smoke test
cargo run -p tinyverse_cli -- tui

# Full CI
bun scripts/ci.sh.ts
```
