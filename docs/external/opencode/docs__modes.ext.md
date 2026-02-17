----
## External Docs Snapshot // opencode

- Captured: 2026-02-17T20:19:48.515Z
- Source root: https://opencode.ai/docs
- Source page: /docs/modes
- Keywords: opencode, docs, ai coding assistant, cli, modes
- Summary: [Skip to content](http://opencode.ai/docs/modes#_top)
----

Source: https://opencode.ai/docs/modes

Modes | OpenCode
===============
[Skip to content](http://opencode.ai/docs/modes#_top)

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

*   [Overview](http://opencode.ai/docs/modes#_top)
*   [Built-in](http://opencode.ai/docs/modes#built-in)
    *   [Build](http://opencode.ai/docs/modes#build)
    *   [Plan](http://opencode.ai/docs/modes#plan)

*   [Switching](http://opencode.ai/docs/modes#switching)
*   [Configure](http://opencode.ai/docs/modes#configure)
    *   [JSON Configuration](http://opencode.ai/docs/modes#json-configuration)
    *   [Markdown Configuration](http://opencode.ai/docs/modes#markdown-configuration)
    *   [Model](http://opencode.ai/docs/modes#model)
    *   [Temperature](http://opencode.ai/docs/modes#temperature)
    *   [Prompt](http://opencode.ai/docs/modes#prompt)
    *   [Tools](http://opencode.ai/docs/modes#tools)

*   [Custom modes](http://opencode.ai/docs/modes#custom-modes)
    *   [Using JSON configuration](http://opencode.ai/docs/modes#using-json-configuration)
    *   [Using markdown files](http://opencode.ai/docs/modes#using-markdown-files)
    *   [Use cases](http://opencode.ai/docs/modes#use-cases)

On this page
------------

*   [Overview](http://opencode.ai/docs/modes#_top)
*   [Built-in](http://opencode.ai/docs/modes#built-in)
    *   [Build](http://opencode.ai/docs/modes#build)
    *   [Plan](http://opencode.ai/docs/modes#plan)

*   [Switching](http://opencode.ai/docs/modes#switching)
*   [Configure](http://opencode.ai/docs/modes#configure)
    *   [JSON Configuration](http://opencode.ai/docs/modes#json-configuration)
    *   [Markdown Configuration](http://opencode.ai/docs/modes#markdown-configuration)
    *   [Model](http://opencode.ai/docs/modes#model)
    *   [Temperature](http://opencode.ai/docs/modes#temperature)
    *   [Prompt](http://opencode.ai/docs/modes#prompt)
    *   [Tools](http://opencode.ai/docs/modes#tools)

*   [Custom modes](http://opencode.ai/docs/modes#custom-modes)
    *   [Using JSON configuration](http://opencode.ai/docs/modes#using-json-configuration)
    *   [Using markdown files](http://opencode.ai/docs/modes#using-markdown-files)
    *   [Use cases](http://opencode.ai/docs/modes#use-cases)

Modes
=====

Different modes for different use cases.

Caution

Modes are now configured through the `agent` option in the opencode config. The `mode` option is now deprecated. [Learn more](http://opencode.ai/docs/agents).

Modes in opencode allow you to customize the behavior, tools, and prompts for different use cases.

It comes with two built-in modes: **build** and **plan**. You can customize these or configure your own through the opencode config.

You can switch between modes during a session or configure them in your config file.

* * *

[Built-in](http://opencode.ai/docs/modes#built-in)
--------------------------------------------------

opencode comes with two built-in modes.

* * *

### [Build](http://opencode.ai/docs/modes#build)

Build is the **default** mode with all tools enabled. This is the standard mode for development work where you need full access to file operations and system commands.

* * *

### [Plan](http://opencode.ai/docs/modes#plan)

A restricted mode designed for planning and analysis. In plan mode, the following tools are disabled by default:

*   `write` - Cannot create new files
*   `edit` - Cannot modify existing files, except for files located at `.opencode/plans/*.md` to detail the plan itself
*   `patch` - Cannot apply patches
*   `bash` - Cannot execute shell commands

This mode is useful when you want the AI to analyze code, suggest changes, or create plans without making any actual modifications to your codebase.

* * *

[Switching](http://opencode.ai/docs/modes#switching)
----------------------------------------------------

You can switch between modes during a session using the _Tab_ key. Or your configured `switch_mode` keybind.

See also: [Formatters](http://opencode.ai/docs/formatters) for information about code formatting configuration.

* * *

[Configure](http://opencode.ai/docs/modes#configure)
----------------------------------------------------

You can customize the built-in modes or create your own through configuration. Modes can be configured in two ways:

### [JSON Configuration](http://opencode.ai/docs/modes#json-configuration)

Configure modes in your `opencode.json` config file:

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "mode": {    "build": {      "model": "anthropic/claude-sonnet-4-20250514",      "prompt": "{file:./prompts/build.txt}",      "tools": {        "write": true,        "edit": true,        "bash": true      }    },    "plan": {      "model": "anthropic/claude-haiku-4-20250514",      "tools": {        "write": false,        "edit": false,        "bash": false      }    }  }}`

### [Markdown Configuration](http://opencode.ai/docs/modes#markdown-configuration)

You can also define modes using markdown files. Place them in:

*   Global: `~/.config/opencode/modes/`
*   Project: `.opencode/modes/`

~/.config/opencode/modes/review.md

```
---model: anthropic/claude-sonnet-4-20250514temperature: 0.1tools:  write: false  edit: false  bash: false---
You are in code review mode. Focus on:
- Code quality and best practices- Potential bugs and edge cases- Performance implications- Security considerations
Provide constructive feedback without making direct changes.
```

The markdown file name becomes the mode name (e.g., `review.md` creates a `review` mode).

Let’s look at these configuration options in detail.

* * *

### [Model](http://opencode.ai/docs/modes#model)

Use the `model` config to override the default model for this mode. Useful for using different models optimized for different tasks. For example, a faster model for planning, a more capable model for implementation.

opencode.json

`{  "mode": {    "plan": {      "model": "anthropic/claude-haiku-4-20250514"    }  }}`

* * *

### [Temperature](http://opencode.ai/docs/modes#temperature)

Control the randomness and creativity of the AI’s responses with the `temperature` config. Lower values make responses more focused and deterministic, while higher values increase creativity and variability.

opencode.json

`{  "mode": {    "plan": {      "temperature": 0.1    },    "creative": {      "temperature": 0.8    }  }}`

Temperature values typically range from 0.0 to 1.0:

*   **0.0-0.2**: Very focused and deterministic responses, ideal for code analysis and planning
*   **0.3-0.5**: Balanced responses with some creativity, good for general development tasks
*   **0.6-1.0**: More creative and varied responses, useful for brainstorming and exploration

opencode.json

`{  "mode": {    "analyze": {      "temperature": 0.1,      "prompt": "{file:./prompts/analysis.txt}"    },    "build": {      "temperature": 0.3    },    "brainstorm": {      "temperature": 0.7,      "prompt": "{file:./prompts/creative.txt}"    }  }}`

If no temperature is specified, opencode uses model-specific defaults (typically 0 for most models, 0.55 for Qwen models).

* * *

### [Prompt](http://opencode.ai/docs/modes#prompt)

Specify a custom system prompt file for this mode with the `prompt` config. The prompt file should contain instructions specific to the mode’s purpose.

opencode.json

`{  "mode": {    "review": {      "prompt": "{file:./prompts/code-review.txt}"    }  }}`

This path is relative to where the config file is located. So this works for both the global opencode config and the project specific config.

* * *

### [Tools](http://opencode.ai/docs/modes#tools)

Control which tools are available in this mode with the `tools` config. You can enable or disable specific tools by setting them to `true` or `false`.

`{  "mode": {    "readonly": {      "tools": {        "write": false,        "edit": false,        "bash": false,        "read": true,        "grep": true,        "glob": true      }    }  }}`

If no tools are specified, all tools are enabled by default.

* * *

#### [Available tools](http://opencode.ai/docs/modes#available-tools)

Here are all the tools can be controlled through the mode config.

| Tool | Description |
| --- | --- |
| `bash` | Execute shell commands |
| `edit` | Modify existing files |
| `write` | Create new files |
| `read` | Read file contents |
| `grep` | Search file contents |
| `glob` | Find files by pattern |
| `list` | List directory contents |
| `patch` | Apply patches to files |
| `todowrite` | Manage todo lists |
| `todoread` | Read todo lists |
| `webfetch` | Fetch web content |

* * *

[Custom modes](http://opencode.ai/docs/modes#custom-modes)
----------------------------------------------------------

You can create your own custom modes by adding them to the configuration. Here are examples using both approaches:

### [Using JSON configuration](http://opencode.ai/docs/modes#using-json-configuration)

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "mode": {    "docs": {      "prompt": "{file:./prompts/documentation.txt}",      "tools": {        "write": true,        "edit": true,        "bash": false,        "read": true,        "grep": true,        "glob": true      }    }  }}`

### [Using markdown files](http://opencode.ai/docs/modes#using-markdown-files)

Create mode files in `.opencode/modes/` for project-specific modes or `~/.config/opencode/modes/` for global modes:

.opencode/modes/debug.md

```
---temperature: 0.1tools:  bash: true  read: true  grep: true  write: false  edit: false---
You are in debug mode. Your primary goal is to help investigate and diagnose issues.
Focus on:
- Understanding the problem through careful analysis- Using bash commands to inspect system state- Reading relevant files and logs- Searching for patterns and anomalies- Providing clear explanations of findings
Do not make any changes to files. Only investigate and report.
```

~/.config/opencode/modes/refactor.md

```
---model: anthropic/claude-sonnet-4-20250514temperature: 0.2tools:  edit: true  read: true  grep: true  glob: true---
You are in refactoring mode. Focus on improving code quality without changing functionality.
Priorities:
- Improve code readability and maintainability- Apply consistent naming conventions- Reduce code duplication- Optimize performance where appropriate- Ensure all tests continue to pass
```

* * *

### [Use cases](http://opencode.ai/docs/modes#use-cases)

Here are some common use cases for different modes.

*   **Build mode**: Full development work with all tools enabled
*   **Plan mode**: Analysis and planning without making changes
*   **Review mode**: Code review with read-only access plus documentation tools
*   **Debug mode**: Focused on investigation with bash and read tools enabled
*   **Docs mode**: Documentation writing with file operations but no system commands

You might also find different models are good for different use cases.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/modes.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 17, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
