# Ratatui Editor + Highlight + Diff Research (Crossterm)

Date: 2026-02-17

## Executive recommendation

For the current TinyVerse stack (`ratatui = 0.29`, `crossterm = 0.29`), the best path is:

1. **Editor widget:** `tui-textarea`
2. **Syntax highlighting:** `syntect` + `syntect-tui` (or direct `syntect` mapping)
3. **Diff engine/UI:** `similar` for UI-friendly diffs, optionally `imara-diff` / `diffy-imara` for heavy diffs and patch flows

This gives fast time-to-value, minimal migration risk, and good terminal compatibility.

## Why this stack is best right now

- `tui-textarea` is mature for ratatui apps, has undo/redo, selection, regex search, line numbers, and backend-agnostic input handling.
- It supports `crossterm`, `termion`, and `termwiz`, which keeps options open while staying on Crossterm now.
- `syntect` is the most practical highlighter in the ratatui ecosystem today (large syntax/theme ecosystem, straightforward API).
- `similar` has a high-level `TextDiff` API with grouped operations and inline diffs that map cleanly to a custom ratatui diff widget.
- `imara-diff` adds a high-performance path when large-file diff speed becomes critical.

## Decision matrix

| Area | Recommended now | Why | Notes |
|---|---|---|---|
| Text editor | `tui-textarea` | Feature-complete text editing widget for ratatui apps | Good fit for current `ratatui 0.29` setup |
| Alternative editor | `edtui` | Vim/Emacs modes, built-in syntax highlighting, theming | Attractive, but integration/migration risk is higher in current stack |
| Alternative editor (large text focus) | `rat-text` | Rope backend, wide unicode-aware editing tools | Part of broader rat-salsa ecosystem; heavier integration surface |
| Syntax highlighting | `syntect` + `syntect-tui` | Proven syntax pipeline + ratatui style bridge | Easiest path without full parser infrastructure |
| Advanced highlighting | `tree-sitter-highlight` | Incremental/parser-based highlighting model | More setup complexity per language |
| Diff (UI friendly) | `similar` | High-level text diff API, grouped ops, inline changes | Best for rendering readable diffs in TUI |
| Diff (performance) | `imara-diff` | Fast histogram/myers implementations | Great for large files and tight latency budgets |
| Patch + merge workflows | `diffy-imara` | Unified diff/patch/apply/merge APIs on imara backend | Useful if you need patch semantics, not just display |

## Notes on key crates

### 1) `tui-textarea` (recommended editor baseline)

Strengths:
- Multi-line editing, undo/redo, selection, mouse scrolling, line numbers
- Configurable key handling plus default Emacs-style shortcuts
- Regex search support (feature-gated)
- Backend agnostic support (`crossterm`, `termion`, `termwiz`, custom backend)

Why it fits TinyVerse:
- TinyVerse TUI already uses `ratatui 0.29` + `crossterm 0.29`
- This is the least risky editor integration path

Important note:
- `ratatui-textarea` fork is explicitly marked unmaintained; use `tui-textarea`.

### 2) `edtui` (strong alternative if you want modal editing UX)

Strengths:
- Vim-first UX with Emacs mode option
- Built-in syntax highlighting support
- Built-in line wrapping, line numbers, mouse, theming, optional system editor support

Tradeoff:
- More opinionated and larger integration footprint than `tui-textarea`

### 3) `rat-text` (strong for large document ergonomics)

Strengths:
- Uses `ropey` backend
- Explicit unicode-aware editing utilities
- Built-in scrolling and style range management
- Docs mention very large text handling and advanced cursor/text mapping utilities

Tradeoff:
- Pulls in a broader widget ecosystem (`rat-*` crates), so adoption is less drop-in.

### 4) Syntax libraries

`syntect`:
- Mature syntax/theme ecosystem and straightforward architecture
- Clear split between parsing and highlighting modules

`syntect-tui`:
- Lightweight conversion layer from `syntect` styles to ratatui `Span`/`Style`
- Good for keeping your own rendering logic while avoiding repetitive conversion code

`tui-syntax-highlight`:
- Produces ratatui `Text` directly from syntect
- Nice high-level API, but check version compatibility against your current ratatui stack before adopting

`tree-sitter-highlight`:
- Event-based highlighting model, supports language injections
- Best when you need parser-aware incremental behavior, but setup and maintenance are higher

### 5) Diff libraries

`similar`:
- Excellent high-level API (`TextDiff`, grouped ops, inline changes)
- Supports deadlines/timeouts, useful to keep UI responsive
- Very suitable for building readable side-by-side or unified diff widgets

`imara-diff`:
- Performance-focused diff engine
- Implements Myers + Histogram; docs highlight strong runtime behavior and histogram speedups

`diffy-imara`:
- Patch-centric API (`create_patch`, `apply`, `merge`) with imara backend
- Useful when you need display + patch/merge operations

## Crossterm and compatibility guidance

- Ratatui backend docs recommend **Crossterm for most tasks**.
- Crossterm is cross-platform and supports Unix + Windows terminals (including older Windows support in docs).
- For editor correctness in terminal cells, include unicode helpers:
  - `unicode-segmentation` (grapheme boundaries)
  - `unicode-width` (display cell width)
  - `unicode-truncate` (safe width-based truncation)
- If you need an embedded shell/editor pane, `tui-term` is a viable PTY widget option.
- If you need larger custom scroll surfaces (code + line numbers + inline annotations), `tui-scrollview` can simplify viewport management.

## Suggested implementation plan for TinyVerse

### Phase 1 (fastest path)

1. Add `tui-textarea` for the editor core state.
2. Add `syntect` + `syntect-tui` for line rendering.
3. Build a unified diff view using `similar::TextDiff` and `grouped_ops`.
4. Keep document model line-based (`Vec<String>`) initially.

### Phase 2 (usability)

1. Add line numbers, search, and navigation shortcuts.
2. Add inline (intraline) highlighting from `similar` inline changes.
3. Add asynchronous diff recalculation + debounce to avoid blocking UI input.

### Phase 3 (scale)

1. Move document model to `ropey` when large-file performance requires it.
2. Consider `imara-diff` for heavy diffs.
3. Optionally evaluate `tree-sitter-highlight` for parser-aware highlighting and injections.

## Recommended dependency shortlist

Start with:

```toml
[dependencies]
tui-textarea = "0.7"
syntect = "5"
syntect-tui = "3"
similar = { version = "2", features = ["inline", "unicode"] }
unicode-segmentation = "1"
unicode-width = "0.2"
unicode-truncate = "2"
```

Add later if needed:

```toml
imara-diff = "0.2"
diffy-imara = "0.3"
ropey = "1"
tree-sitter-highlight = "0.26"
```

## Risk watchlist

- **Version compatibility:** verify any crate tied to ratatui core/widgets split against current `ratatui 0.29` usage.
- **Highlight performance:** avoid full-buffer re-highlight on every keystroke; highlight visible window + small margins.
- **Unicode cursor drift:** always perform cursor movement and diff offsets with grapheme-aware logic.
- **External editor handoff:** if spawning `$EDITOR`, cleanly leave alternate screen/raw mode and restore afterward.

## Sources used (Exa + primary docs)

- Ratatui backend comparison: <https://ratatui.rs/concepts/backends/comparison/>
- Ratatui third-party widgets: <https://ratatui.rs/showcase/third-party-widgets/>
- Ratatui spawn external editor recipe: <https://ratatui.rs/recipes/apps/spawn-vim/>
- `tui-textarea`: <https://docs.rs/tui-textarea/latest/tui_textarea/>
- `edtui`: <https://docs.rs/edtui/latest/edtui/>
- `rat-text`: <https://docs.rs/rat-text/latest/rat_text/>
- `syntect`: <https://docs.rs/syntect/latest/syntect/>
- `syntect-tui`: <https://docs.rs/syntect-tui/latest/syntect_tui/>
- `tui-syntax-highlight`: <https://docs.rs/tui-syntax-highlight/latest/tui_syntax_highlight/>
- `tree-sitter-highlight`: <https://docs.rs/tree-sitter-highlight/latest/tree_sitter_highlight/>
- `similar`: <https://docs.rs/similar/latest/similar/>
- `imara-diff`: <https://docs.rs/imara-diff/latest/imara_diff/>
- `diffy-imara`: <https://docs.rs/diffy-imara/latest/diffy_imara/>
- `crossterm`: <https://docs.rs/crossterm/latest/crossterm/>
- `ropey`: <https://docs.rs/ropey/latest/ropey/>
- `unicode-segmentation`: <https://docs.rs/unicode-segmentation/latest/unicode_segmentation/>
- `unicode-width`: <https://docs.rs/unicode-width/latest/unicode_width/>
- `unicode-truncate`: <https://docs.rs/unicode-truncate/latest/unicode_truncate/>
- `tui-term`: <https://docs.rs/tui-term/latest/tui_term/>
- `tui-scrollview`: <https://docs.rs/tui-scrollview/latest/tui_scrollview/>
