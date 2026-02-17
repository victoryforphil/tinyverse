----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/info
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, plugin, info
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/info

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)
- [info](/docs/proto/commands/plugin/info)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin info

v0.23.0

The `proto plugin info ` command will display information about a tool and its plugin.

```
$ proto plugin info nodePlugin ─────────────────────────────────  ID: node  Name: Node.js  Type: Language  Version: 0.13.0  Source URL: https://github.com/moonrepo/plugins/releases/download/node_tool-v0.13.0/node_tool.wasmInventory ──────────────────────────────  Detected version: 23.5.0  Fallback version: 23.5.0  Store directory: /Users/name/.proto/tools/node  Executable file: /Users/name/.proto/tools/node/23.5.0/bin/node  Executables directory: /Users/name/.proto/tools/node/23.5.0/bin  Global packages directory: /Users/name/.proto/tools/node/globals/bin  Shims:    - /Users/name/.proto/shims/node  Binaries:    - /Users/name/.proto/bin/node    - /Users/name/.proto/bin/node-20    - /Users/name/.proto/bin/node-20.15    - /Users/name/.proto/bin/node-20.8    - /Users/name/.proto/bin/node-23    - /Users/name/.proto/bin/node-23.4    - /Users/name/.proto/bin/node-23.5  Installed versions:    20.8.0 - installed 12/19/24, last used 12/19/24    20.15.0 - installed 12/25/24, last used 12/25/24    23.4.0 - installed 12/19/24, last used 12/19/24    23.5.0 - installed 12/25/24, last used 12/25/24, fallback version  Remote aliases:    argon = 4.9.1    boron = 6.17.1    carbon = 8.17.0    dubnium = 10.24.1    erbium = 12.22.12    fermium = 14.21.3    gallium = 16.20.2    hydrogen = 18.20.5    iron = 20.18.1    jod = 22.12.0    latest = 23.5.0    stable = 22.12.0Configuration ──────────────────────────  Local aliases:    example = 19.0.0  Environment variables: —  Settings: —
```

### Arguments[​](#arguments)

- `` - ID of tool.

### Options[​](#options)

- `--json` - Print the info in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/plugin/info.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
