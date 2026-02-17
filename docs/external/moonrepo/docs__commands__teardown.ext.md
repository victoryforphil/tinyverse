----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/teardown
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, teardown
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/teardown

- [Home](/)
- [Commands](/docs/commands)
- [teardown](/docs/commands/teardown)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# teardown

The `moon teardown` command, as its name infers, will teardown and clean the current environment,
opposite the [`setup`](/docs/commands/setup) command. It achieves this by doing the following:

- Uninstalling all configured tools in the toolchain.

- Removing any download or temporary files/folders.

```
$ moon teardown
```

### Configuration[​](#configuration)

- [`*`](/docs/config/toolchain) in `.moon/toolchains.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/teardown.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
