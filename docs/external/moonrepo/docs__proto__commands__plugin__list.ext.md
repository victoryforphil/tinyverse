----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/list
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, plugin, list
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/list

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)
- [list](/docs/proto/commands/plugin/list)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin list

v0.23.0

The `proto plugin list [...id]` command will list all available and configured plugins, for both
third-party and built-in tools. Will load all `./.prototools` traversing upwards, and the
`~/.proto/.prototools` file.

Furthermore, it can list tool information, along with their installed versions, relevant timestamps,
available aliases, and store location.

```
$ proto plugin list --versionsBun ────────────────────────────────────  ID: bun  Source URL: https://github.com/moonrepo/plugins/releases/download/bun_tool-v0.14.0/bun_tool.wasm  Store directory: /Users/miles/.proto/tools/bun  Versions:    1.1.42 - installed 12/25/24, fallback versionDeno ───────────────────────────────────  ID: deno  Source URL: https://github.com/moonrepo/plugins/releases/download/deno_tool-v0.13.0/deno_tool.wasm  Store directory: /Users/miles/.proto/tools/deno  Versions:    1.30.0 - installed 02/01/24, last used 11/28/24    1.40.0 - installed 02/01/24, last used 12/09/24    1.43.1 - installed 12/25/24, fallback versionGo ─────────────────────────────────────  ID: go  Source URL: https://github.com/moonrepo/plugins/releases/download/go_tool-v0.14.0/go_tool.wasm  Store directory: /Users/miles/.proto/tools/go  Versions:    1.18.0 - installed 12/25/24, fallback version    1.19.0 - installed 12/22/24    1.20.12 - installed 12/09/23    1.23.4 - installed 12/24/24
```

A list of tool IDs can be provided to filter the output list.

```
$ proto plugin list node npm
```

### Arguments[​](#arguments)

- `[id...]` - IDs of tools.

### Options[​](#options)

- `--aliases` - Print the list with resolved aliases.

- `--versions` - Print the list with installed versions.

- `--json` - Print the list in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/plugin/list.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
