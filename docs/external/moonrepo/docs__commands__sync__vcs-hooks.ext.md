----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/vcs-hooks
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, sync, vcs hooks
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/sync/vcs-hooks

- [Home](/)
- [Commands](/docs/commands)
- [sync](/docs/commands/sync)
- [vcs-hooks](/docs/commands/sync/vcs-hooks)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# sync vcs-hooks

v1.9.0

The `moon sync vcs-hooks` command will manually sync hooks for the configured
[VCS](/docs/config/workspace#vcs), by generating and referencing hook scripts from the
[`vcs.hooks`](/docs/config/workspace#hooks) setting. Refer to the official
[VCS hooks](/docs/guides/vcs-hooks) guide for more information.

```
$ moon sync vcs-hooks
```

### Options[​](#options)

- `--clean` - Clean and remove previously generated hooks.

- `--force` - Bypass cache and force create hooks.

### Configuration[​](#configuration)

- [`vcs.hooks`](/docs/config/workspace#hooks) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/sync/vcs-hooks.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
