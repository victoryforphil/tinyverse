----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/use
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, use
- Summary: This command has been deprecated and its functionality was merged into [`proto install`](/docs/proto/commands/install)
----

Source: https://moonrepo.dev/docs/proto/commands/use

# use

danger

This command has been deprecated and its functionality was merged into [`proto install`](/docs/proto/commands/install)
in v0.39. Use that command instead!

The `proto use` (or `proto u`) command will download and install all tools and plugins from all
parent [`.prototools`](/docs/proto/config) configuration files, and any [versions detected](/docs/proto/detection) in
the current working directory (if not defined in `.prototools`).

```
$ proto use
```

This command does not install tools for versions pinned in the global `~/.proto/.prototools`
file.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/use.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
