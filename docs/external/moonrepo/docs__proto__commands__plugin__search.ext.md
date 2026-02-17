----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/plugin/search
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, plugin, search
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/plugin/search

- [Home](/)
- Commands
- [plugin](/docs/proto/commands/plugin)
- [search](/docs/proto/commands/plugin/search)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# plugin search

v0.36.0

The `proto plugin search ` command will search for plugins provided by the community, based
on the provided query string. Built-in plugins are not searchable.

```
$ proto plugin search moonSearch results for: moonLearn more about plugins: https://moonrepo.dev/docs/proto/plugins╭──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Plugin      Author    Format Description             Locator                                                             ││──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││ moon        moonrepo  TOML   moon is a multi-        https://raw.githubusercontent.com/moonrepo/moon/master/proto-       ││                              language build system   plugin.toml                                                         ││                              and codebase management                                                                     ││                              tool.                                                                                       │╰──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

### Arguments[​](#arguments)

- `` - Query string to match against.

### Options[​](#options)

- `--json` - Print the results in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/plugin/search.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
