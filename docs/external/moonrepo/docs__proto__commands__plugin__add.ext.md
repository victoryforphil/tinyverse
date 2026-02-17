----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/add
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, plugin, add
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/add

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)
- [add](/docs/proto/commands/plugin/add)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin add

v0.23.0

The `proto plugin add
` command will add the provided ID and plugin locator string to
the `[plugins]` section of a chosen `.prototools`.

```
$ proto plugin add node "https://github.com/moonrepo/node-plugin/releases/latest/download/node_plugin.wasm"
```

Learn more about [plugin locator strings](/docs/proto/plugins#enabling-plugins).

### Arguments[​](#arguments)

- `` - ID of the tool.

- `` - How to locate the plugin.

### Options[​](#options)

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to add, either `tool` (default) or `backend`. v0.52.0

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/plugin/add.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
