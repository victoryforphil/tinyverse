----
## External Docs Snapshot // opencode

- Captured: 2026-02-17T20:19:48.515Z
- Source root: https://opencode.ai/docs
- Source page: /docs/lsp
- Keywords: opencode, docs, ai coding assistant, cli, lsp
- Summary: [Skip to content](http://opencode.ai/docs/lsp#_top)
----

Source: https://opencode.ai/docs/lsp

LSP Servers | OpenCode
===============
[Skip to content](http://opencode.ai/docs/lsp#_top)

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

*   [Overview](http://opencode.ai/docs/lsp#_top)
*   [Built-in](http://opencode.ai/docs/lsp#built-in)
*   [How It Works](http://opencode.ai/docs/lsp#how-it-works)
*   [Configure](http://opencode.ai/docs/lsp#configure)
    *   [Environment variables](http://opencode.ai/docs/lsp#environment-variables)
    *   [Initialization options](http://opencode.ai/docs/lsp#initialization-options)
    *   [Disabling LSP servers](http://opencode.ai/docs/lsp#disabling-lsp-servers)
    *   [Custom LSP servers](http://opencode.ai/docs/lsp#custom-lsp-servers)

*   [Additional Information](http://opencode.ai/docs/lsp#additional-information)
    *   [PHP Intelephense](http://opencode.ai/docs/lsp#php-intelephense)

On this page
------------

*   [Overview](http://opencode.ai/docs/lsp#_top)
*   [Built-in](http://opencode.ai/docs/lsp#built-in)
*   [How It Works](http://opencode.ai/docs/lsp#how-it-works)
*   [Configure](http://opencode.ai/docs/lsp#configure)
    *   [Environment variables](http://opencode.ai/docs/lsp#environment-variables)
    *   [Initialization options](http://opencode.ai/docs/lsp#initialization-options)
    *   [Disabling LSP servers](http://opencode.ai/docs/lsp#disabling-lsp-servers)
    *   [Custom LSP servers](http://opencode.ai/docs/lsp#custom-lsp-servers)

*   [Additional Information](http://opencode.ai/docs/lsp#additional-information)
    *   [PHP Intelephense](http://opencode.ai/docs/lsp#php-intelephense)

LSP Servers
===========

OpenCode integrates with your LSP servers.

OpenCode integrates with your Language Server Protocol (LSP) to help the LLM interact with your codebase. It uses diagnostics to provide feedback to the LLM.

* * *

[Built-in](http://opencode.ai/docs/lsp#built-in)
------------------------------------------------

OpenCode comes with several built-in LSP servers for popular languages:

| LSP Server | Extensions | Requirements |
| --- | --- | --- |
| astro | .astro | Auto-installs for Astro projects |
| bash | .sh, .bash, .zsh, .ksh | Auto-installs bash-language-server |
| clangd | .c, .cpp, .cc, .cxx, .c++, .h, .hpp, .hh, .hxx, .h++ | Auto-installs for C/C++ projects |
| csharp | .cs | `.NET SDK` installed |
| clojure-lsp | .clj, .cljs, .cljc, .edn | `clojure-lsp` command available |
| dart | .dart | `dart` command available |
| deno | .ts, .tsx, .js, .jsx, .mjs | `deno` command available (auto-detects deno.json/deno.jsonc) |
| elixir-ls | .ex, .exs | `elixir` command available |
| eslint | .ts, .tsx, .js, .jsx, .mjs, .cjs, .mts, .cts, .vue | `eslint` dependency in project |
| fsharp | .fs, .fsi, .fsx, .fsscript | `.NET SDK` installed |
| gleam | .gleam | `gleam` command available |
| gopls | .go | `go` command available |
| hls | .hs, .lhs | `haskell-language-server-wrapper` command available |
| jdtls | .java | `Java SDK (version 21+)` installed |
| kotlin-ls | .kt, .kts | Auto-installs for Kotlin projects |
| lua-ls | .lua | Auto-installs for Lua projects |
| nixd | .nix | `nixd` command available |
| ocaml-lsp | .ml, .mli | `ocamllsp` command available |
| oxlint | .ts, .tsx, .js, .jsx, .mjs, .cjs, .mts, .cts, .vue, .astro, .svelte | `oxlint` dependency in project |
| php intelephense | .php | Auto-installs for PHP projects |
| prisma | .prisma | `prisma` command available |
| pyright | .py, .pyi | `pyright` dependency installed |
| ruby-lsp (rubocop) | .rb, .rake, .gemspec, .ru | `ruby` and `gem` commands available |
| rust | .rs | `rust-analyzer` command available |
| sourcekit-lsp | .swift, .objc, .objcpp | `swift` installed (`xcode` on macOS) |
| svelte | .svelte | Auto-installs for Svelte projects |
| terraform | .tf, .tfvars | Auto-installs from GitHub releases |
| tinymist | .typ, .typc | Auto-installs from GitHub releases |
| typescript | .ts, .tsx, .js, .jsx, .mjs, .cjs, .mts, .cts | `typescript` dependency in project |
| vue | .vue | Auto-installs for Vue projects |
| yaml-ls | .yaml, .yml | Auto-installs Red Hat yaml-language-server |
| zls | .zig, .zon | `zig` command available |

LSP servers are automatically enabled when one of the above file extensions are detected and the requirements are met.

Note

You can disable automatic LSP server downloads by setting the `OPENCODE_DISABLE_LSP_DOWNLOAD` environment variable to `true`.

* * *

[How It Works](http://opencode.ai/docs/lsp#how-it-works)
--------------------------------------------------------

When opencode opens a file, it:

1.   Checks the file extension against all enabled LSP servers.
2.   Starts the appropriate LSP server if not already running.

* * *

[Configure](http://opencode.ai/docs/lsp#configure)
--------------------------------------------------

You can customize LSP servers through the `lsp` section in your opencode config.

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": {}}`

Each LSP server supports the following:

| Property | Type | Description |
| --- | --- | --- |
| `disabled` | boolean | Set this to `true` to disable the LSP server |
| `command` | string[] | The command to start the LSP server |
| `extensions` | string[] | File extensions this LSP server should handle |
| `env` | object | Environment variables to set when starting server |
| `initialization` | object | Initialization options to send to the LSP server |

Let’s look at some examples.

* * *

### [Environment variables](http://opencode.ai/docs/lsp#environment-variables)

Use the `env` property to set environment variables when starting the LSP server:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": {    "rust": {      "env": {        "RUST_LOG": "debug"      }    }  }}`

* * *

### [Initialization options](http://opencode.ai/docs/lsp#initialization-options)

Use the `initialization` property to pass initialization options to the LSP server. These are server-specific settings sent during the LSP `initialize` request:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": {    "typescript": {      "initialization": {        "preferences": {          "importModuleSpecifierPreference": "relative"        }      }    }  }}`

Note

Initialization options vary by LSP server. Check your LSP server’s documentation for available options.

* * *

### [Disabling LSP servers](http://opencode.ai/docs/lsp#disabling-lsp-servers)

To disable **all** LSP servers globally, set `lsp` to `false`:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": false}`

To disable a **specific** LSP server, set `disabled` to `true`:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": {    "typescript": {      "disabled": true    }  }}`

* * *

### [Custom LSP servers](http://opencode.ai/docs/lsp#custom-lsp-servers)

You can add custom LSP servers by specifying the command and file extensions:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "lsp": {    "custom-lsp": {      "command": ["custom-lsp-server", "--stdio"],      "extensions": [".custom"]    }  }}`

* * *

[Additional Information](http://opencode.ai/docs/lsp#additional-information)
----------------------------------------------------------------------------

### [PHP Intelephense](http://opencode.ai/docs/lsp#php-intelephense)

PHP Intelephense offers premium features through a license key. You can provide a license key by placing (only) the key in a text file at:

*   On macOS/Linux: `$HOME/intelephense/license.txt`
*   On Windows: `%USERPROFILE%/intelephense/license.txt`

The file should contain only the license key with no additional content.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/lsp.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 17, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
