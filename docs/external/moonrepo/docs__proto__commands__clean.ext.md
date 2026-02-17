----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/clean
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, clean
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/clean

- [Home](/)
- Commands
- [clean](/docs/proto/commands/clean)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# clean

The `proto clean` command can be used to uninstall stale and unused tools, plugins, and more. By
default, it will remove items that haven't been used in the last 30 days.

```
$ proto clean
```

Furthermore, the command can be used to target a specific artifact type.

```
$ proto clean plugins
```

### Arguments[​](#arguments)

- `[target]` - Type of target. Accepts `cache`, `plugins`, `temp`, or `tools`. v0.44.0

### Options[​](#options)

- `--days` - Number of days before a tool is considered stale.

- `--json` - Print the clean result in JSON format. v0.44.0

- `--yes` - Avoid and confirm all prompts.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/clean.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
