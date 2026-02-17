# TinyVerse TUI Themes

Starter themes for `tinyverse tui`.

Available:

- `vfp.theme.toml` — cyber-sleek, orange + cyan accents (zsh-style-guide aligned)
- `suchblue.theme.toml` — cool blue-forward
- `ember.theme.toml` — warm charcoal
- `rosepine.theme.toml` — muted rose/lavender variant

## Use a Theme

Priority order:

1. CLI flag: `tinyverse tui --theme <name-or-path>`
2. Env var: `TINYVERSE_THEME=<name-or-path>`
3. Config: `[tui] theme = "<name-or-path>"`
4. Fallback: `theme.toml`

Name lookup supports both:

- `<dir>/<name>.theme.toml`
- `<dir>/themes/<name>.theme.toml`

Where `<dir>` is searched in TinyVerse home first, then current working directory.

Examples:

```bash
tinyverse tui --theme vfp
TINYVERSE_THEME=suchblue tinyverse tui
tinyverse tui --theme ./themes/ember.theme.toml
tinyverse config set tui.theme ember
```
