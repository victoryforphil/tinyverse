----
## External Docs Snapshot // opencode

- Captured: 2026-02-17T20:19:48.515Z
- Source root: https://opencode.ai/docs
- Source page: /docs/formatters
- Keywords: opencode, docs, ai coding assistant, cli, formatters
- Summary: [Skip to content](http://opencode.ai/docs/formatters#_top)
----

Source: https://opencode.ai/docs/formatters

Formatters | OpenCode
===============
[Skip to content](http://opencode.ai/docs/formatters#_top)

[![Image 1](http://opencode.ai/docs/_astro/logo-dark.DOStV66V.svg)![Image 2](http://opencode.ai/docs/_astro/logo-light.B0yzR0O5.svg) OpenCode](http://opencode.ai/docs/)

[app.header.home](http://opencode.ai/)[app.header.docs](http://opencode.ai/docs/)

[](https://github.com/anomalyco/opencode)[](https://opencode.ai/discord)

Search Ctrl K

 Cancel 

*   [Intro](http://opencode.ai/docs/)
*   [Config](http://opencode.ai/docs/config/)
*   [Providers](http://opencode.ai/docs/providers/)
*   [Network](http://opencode.ai/docs/network/)
*   [Enterprise](http://opencode.ai/docs/enterprise/)
*   [Troubleshooting](http://opencode.ai/docs/troubleshooting/)
*   [Windows (WSL)](http://opencode.ai/docs/windows-wsl/)
*   
Usage 
    *   [TUI](http://opencode.ai/docs/tui/)
    *   [CLI](http://opencode.ai/docs/cli/)
    *   [Web](http://opencode.ai/docs/web/)
    *   [IDE](http://opencode.ai/docs/ide/)
    *   [Zen](http://opencode.ai/docs/zen/)
    *   [Share](http://opencode.ai/docs/share/)
    *   [GitHub](http://opencode.ai/docs/github/)
    *   [GitLab](http://opencode.ai/docs/gitlab/)

*   
Configure 
    *   [Tools](http://opencode.ai/docs/tools/)
    *   [Rules](http://opencode.ai/docs/rules/)
    *   [Agents](http://opencode.ai/docs/agents/)
    *   [Models](http://opencode.ai/docs/models/)
    *   [Themes](http://opencode.ai/docs/themes/)
    *   [Keybinds](http://opencode.ai/docs/keybinds/)
    *   [Commands](http://opencode.ai/docs/commands/)
    *   [Formatters](http://opencode.ai/docs/formatters/)
    *   [Permissions](http://opencode.ai/docs/permissions/)
    *   [LSP Servers](http://opencode.ai/docs/lsp/)
    *   [MCP servers](http://opencode.ai/docs/mcp-servers/)
    *   [ACP Support](http://opencode.ai/docs/acp/)
    *   [Agent Skills](http://opencode.ai/docs/skills/)
    *   [Custom Tools](http://opencode.ai/docs/custom-tools/)

*   
Develop 
    *   [SDK](http://opencode.ai/docs/sdk/)
    *   [Server](http://opencode.ai/docs/server/)
    *   [Plugins](http://opencode.ai/docs/plugins/)
    *   [Ecosystem](http://opencode.ai/docs/ecosystem/)

[GitHub](https://github.com/anomalyco/opencode)[Discord](https://opencode.ai/discord)

Select theme Select language 

On this page

*   [Overview](http://opencode.ai/docs/formatters#_top)
*   [Built-in](http://opencode.ai/docs/formatters#built-in)
*   [How it works](http://opencode.ai/docs/formatters#how-it-works)
*   [Configure](http://opencode.ai/docs/formatters#configure)
    *   [Disabling formatters](http://opencode.ai/docs/formatters#disabling-formatters)
    *   [Custom formatters](http://opencode.ai/docs/formatters#custom-formatters)

On this page
------------

*   [Overview](http://opencode.ai/docs/formatters#_top)
*   [Built-in](http://opencode.ai/docs/formatters#built-in)
*   [How it works](http://opencode.ai/docs/formatters#how-it-works)
*   [Configure](http://opencode.ai/docs/formatters#configure)
    *   [Disabling formatters](http://opencode.ai/docs/formatters#disabling-formatters)
    *   [Custom formatters](http://opencode.ai/docs/formatters#custom-formatters)

Formatters
==========

OpenCode uses language specific formatters.

OpenCode automatically formats files after they are written or edited using language-specific formatters. This ensures that the code that is generated follows the code styles of your project.

* * *

[Built-in](http://opencode.ai/docs/formatters#built-in)
-------------------------------------------------------

OpenCode comes with several built-in formatters for popular languages and frameworks. Below is a list of the formatters, supported file extensions, and commands or config options it needs.

| Formatter | Extensions | Requirements |
| --- | --- | --- |
| air | .R | `air` command available |
| biome | .js, .jsx, .ts, .tsx, .html, .css, .md, .json, .yaml, and [more](https://biomejs.dev/) | `biome.json(c)` config file |
| cargofmt | .rs | `cargo fmt` command available |
| clang-format | .c, .cpp, .h, .hpp, .ino, and [more](https://clang.llvm.org/docs/ClangFormat.html) | `.clang-format` config file |
| cljfmt | .clj, .cljs, .cljc, .edn | `cljfmt` command available |
| dart | .dart | `dart` command available |
| dfmt | .d | `dfmt` command available |
| gleam | .gleam | `gleam` command available |
| gofmt | .go | `gofmt` command available |
| htmlbeautifier | .erb, .html.erb | `htmlbeautifier` command available |
| ktlint | .kt, .kts | `ktlint` command available |
| mix | .ex, .exs, .eex, .heex, .leex, .neex, .sface | `mix` command available |
| nixfmt | .nix | `nixfmt` command available |
| ocamlformat | .ml, .mli | `ocamlformat` command available and `.ocamlformat` config file |
| ormolu | .hs | `ormolu` command available |
| oxfmt (Experimental) | .js, .jsx, .ts, .tsx | `oxfmt` dependency in `package.json` and an [experimental env variable flag](http://opencode.ai/docs/cli/#experimental) |
| pint | .php | `laravel/pint` dependency in `composer.json` |
| prettier | .js, .jsx, .ts, .tsx, .html, .css, .md, .json, .yaml, and [more](https://prettier.io/docs/en/index.html) | `prettier` dependency in `package.json` |
| rubocop | .rb, .rake, .gemspec, .ru | `rubocop` command available |
| ruff | .py, .pyi | `ruff` command available with config |
| rustfmt | .rs | `rustfmt` command available |
| shfmt | .sh, .bash | `shfmt` command available |
| standardrb | .rb, .rake, .gemspec, .ru | `standardrb` command available |
| terraform | .tf, .tfvars | `terraform` command available |
| uv | .py, .pyi | `uv` command available |
| zig | .zig, .zon | `zig` command available |

So if your project has `prettier` in your `package.json`, OpenCode will automatically use it.

* * *

[How it works](http://opencode.ai/docs/formatters#how-it-works)
---------------------------------------------------------------

When OpenCode writes or edits a file, it:

1.   Checks the file extension against all enabled formatters.
2.   Runs the appropriate formatter command on the file.
3.   Applies the formatting changes automatically.

This process happens in the background, ensuring your code styles are maintained without any manual steps.

* * *

[Configure](http://opencode.ai/docs/formatters#configure)
---------------------------------------------------------

You can customize formatters through the `formatter` section in your OpenCode config.

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "formatter": {}}`

Each formatter configuration supports the following:

| Property | Type | Description |
| --- | --- | --- |
| `disabled` | boolean | Set this to `true` to disable the formatter |
| `command` | string[] | The command to run for formatting |
| `environment` | object | Environment variables to set when running the formatter |
| `extensions` | string[] | File extensions this formatter should handle |

Let’s look at some examples.

* * *

### [Disabling formatters](http://opencode.ai/docs/formatters#disabling-formatters)

To disable **all** formatters globally, set `formatter` to `false`:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "formatter": false}`

To disable a **specific** formatter, set `disabled` to `true`:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "formatter": {    "prettier": {      "disabled": true    }  }}`

* * *

### [Custom formatters](http://opencode.ai/docs/formatters#custom-formatters)

You can override the built-in formatters or add new ones by specifying the command, environment variables, and file extensions:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "formatter": {    "prettier": {      "command": ["npx", "prettier", "--write", "$FILE"],      "environment": {        "NODE_ENV": "development"      },      "extensions": [".js", ".ts", ".jsx", ".tsx"]    },    "custom-markdown-formatter": {      "command": ["deno", "fmt", "$FILE"],      "extensions": [".md"]    }  }}`

The **`$FILE` placeholder** in the command will be replaced with the path to the file being formatted.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/formatters.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 17, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
