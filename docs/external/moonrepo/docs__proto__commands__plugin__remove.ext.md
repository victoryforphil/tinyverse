----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/remove
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, plugin, remove
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/remove

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)
- [remove](/docs/proto/commands/plugin/remove)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin remove

v0.23.0

The `proto plugin remove ` command will remove the provided tool ID from the `[plugins]` section
of the chosen (`.prototools`).

```
$ proto plugin remove node
```

Built-in plugins cannot be removed!

### Arguments[​](#arguments)

- `` - ID of the tool.

### Options[​](#options)

- `--from` - [Location of `.prototools`](/docs/proto/config#locations) to update. v0.41.0

- `--type` - Type of plugin to remove, either `tool` (default) or `backend`. v0.52.0

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/plugin/remove.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
