----
## External Docs Snapshot // opencode

- Captured: 2026-02-17T20:19:48.515Z
- Source root: https://opencode.ai/docs
- Source page: /docs/models
- Keywords: opencode, docs, ai coding assistant, cli, models
- Summary: [Skip to content](http://opencode.ai/docs/models#_top)
----

Source: https://opencode.ai/docs/models

Models | OpenCode
===============
[Skip to content](http://opencode.ai/docs/models#_top)

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

*   [Overview](http://opencode.ai/docs/models#_top)
*   [Providers](http://opencode.ai/docs/models#providers)
*   [Select a model](http://opencode.ai/docs/models#select-a-model)
*   [Recommended models](http://opencode.ai/docs/models#recommended-models)
*   [Set a default](http://opencode.ai/docs/models#set-a-default)
*   [Configure models](http://opencode.ai/docs/models#configure-models)
*   [Variants](http://opencode.ai/docs/models#variants)
    *   [Built-in variants](http://opencode.ai/docs/models#built-in-variants)
    *   [Custom variants](http://opencode.ai/docs/models#custom-variants)
    *   [Cycle variants](http://opencode.ai/docs/models#cycle-variants)

*   [Loading models](http://opencode.ai/docs/models#loading-models)

On this page
------------

*   [Overview](http://opencode.ai/docs/models#_top)
*   [Providers](http://opencode.ai/docs/models#providers)
*   [Select a model](http://opencode.ai/docs/models#select-a-model)
*   [Recommended models](http://opencode.ai/docs/models#recommended-models)
*   [Set a default](http://opencode.ai/docs/models#set-a-default)
*   [Configure models](http://opencode.ai/docs/models#configure-models)
*   [Variants](http://opencode.ai/docs/models#variants)
    *   [Built-in variants](http://opencode.ai/docs/models#built-in-variants)
    *   [Custom variants](http://opencode.ai/docs/models#custom-variants)
    *   [Cycle variants](http://opencode.ai/docs/models#cycle-variants)

*   [Loading models](http://opencode.ai/docs/models#loading-models)

Models
======

Configuring an LLM provider and model.

OpenCode uses the [AI SDK](https://ai-sdk.dev/) and [Models.dev](https://models.dev/) to support **75+ LLM providers** and it supports running local models.

* * *

[Providers](http://opencode.ai/docs/models#providers)
-----------------------------------------------------

Most popular providers are preloaded by default. If you’ve added the credentials for a provider through the `/connect` command, they’ll be available when you start OpenCode.

Learn more about [providers](http://opencode.ai/docs/providers).

* * *

[Select a model](http://opencode.ai/docs/models#select-a-model)
---------------------------------------------------------------

Once you’ve configured your provider you can select the model you want by typing in:

`/models`

* * *

[Recommended models](http://opencode.ai/docs/models#recommended-models)
-----------------------------------------------------------------------

There are a lot of models out there, with new models coming out every week.

Tip

Consider using one of the models we recommend.

However, there are only a few of them that are good at both generating code and tool calling.

Here are several models that work well with OpenCode, in no particular order. (This is not an exhaustive list nor is it necessarily up to date):

*   GPT 5.2
*   GPT 5.1 Codex
*   Claude Opus 4.5
*   Claude Sonnet 4.5
*   Minimax M2.1
*   Gemini 3 Pro

* * *

[Set a default](http://opencode.ai/docs/models#set-a-default)
-------------------------------------------------------------

To set one of these as the default model, you can set the `model` key in your OpenCode config.

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "model": "lmstudio/google/gemma-3n-e4b"}`

Here the full ID is `provider_id/model_id`. For example, if you’re using [OpenCode Zen](http://opencode.ai/docs/zen), you would use `opencode/gpt-5.1-codex` for GPT 5.1 Codex.

If you’ve configured a [custom provider](http://opencode.ai/docs/providers#custom), the `provider_id` is key from the `provider` part of your config, and the `model_id` is the key from `provider.models`.

* * *

[Configure models](http://opencode.ai/docs/models#configure-models)
-------------------------------------------------------------------

You can globally configure a model’s options through the config.

opencode.jsonc

`{  "$schema": "https://opencode.ai/config.json",  "provider": {    "openai": {      "models": {        "gpt-5": {          "options": {            "reasoningEffort": "high",            "textVerbosity": "low",            "reasoningSummary": "auto",            "include": ["reasoning.encrypted_content"],          },        },      },    },    "anthropic": {      "models": {        "claude-sonnet-4-5-20250929": {          "options": {            "thinking": {              "type": "enabled",              "budgetTokens": 16000,            },          },        },      },    },  },}`

Here we’re configuring global settings for two built-in models: `gpt-5` when accessed via the `openai` provider, and `claude-sonnet-4-20250514` when accessed via the `anthropic` provider. The built-in provider and model names can be found on [Models.dev](https://models.dev/).

You can also configure these options for any agents that you are using. The agent config overrides any global options here. [Learn more](http://opencode.ai/docs/agents/#additional).

You can also define custom variants that extend built-in ones. Variants let you configure different settings for the same model without creating duplicate entries:

opencode.jsonc

`{  "$schema": "https://opencode.ai/config.json",  "provider": {    "opencode": {      "models": {        "gpt-5": {          "variants": {            "high": {              "reasoningEffort": "high",              "textVerbosity": "low",              "reasoningSummary": "auto",            },            "low": {              "reasoningEffort": "low",              "textVerbosity": "low",              "reasoningSummary": "auto",            },          },        },      },    },  },}`

* * *

[Variants](http://opencode.ai/docs/models#variants)
---------------------------------------------------

Many models support multiple variants with different configurations. OpenCode ships with built-in default variants for popular providers.

### [Built-in variants](http://opencode.ai/docs/models#built-in-variants)

OpenCode ships with default variants for many providers:

**Anthropic**:

*   `high` - High thinking budget (default)
*   `max` - Maximum thinking budget

**OpenAI**:

Varies by model but roughly:

*   `none` - No reasoning
*   `minimal` - Minimal reasoning effort
*   `low` - Low reasoning effort
*   `medium` - Medium reasoning effort
*   `high` - High reasoning effort
*   `xhigh` - Extra high reasoning effort

**Google**:

*   `low` - Lower effort/token budget
*   `high` - Higher effort/token budget

Tip

This list is not comprehensive. Many other providers have built-in defaults too.

### [Custom variants](http://opencode.ai/docs/models#custom-variants)

You can override existing variants or add your own:

opencode.jsonc

`{  "$schema": "https://opencode.ai/config.json",  "provider": {    "openai": {      "models": {        "gpt-5": {          "variants": {            "thinking": {              "reasoningEffort": "high",              "textVerbosity": "low",            },            "fast": {              "disabled": true,            },          },        },      },    },  },}`

### [Cycle variants](http://opencode.ai/docs/models#cycle-variants)

Use the keybind `variant_cycle` to quickly switch between variants. [Learn more](http://opencode.ai/docs/keybinds).

* * *

[Loading models](http://opencode.ai/docs/models#loading-models)
---------------------------------------------------------------

When OpenCode starts up, it checks for models in the following priority order:

1.   The `--model` or `-m` command line flag. The format is the same as in the config file: `provider_id/model_id`.

2.   The model list in the OpenCode config.

opencode.json

`{  "$schema": "https://opencode.ai/config.json",  "model": "anthropic/claude-sonnet-4-20250514"}`    
The format here is `provider/model`.

3.   The last used model.

4.   The first model using an internal priority.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/models.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 17, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
