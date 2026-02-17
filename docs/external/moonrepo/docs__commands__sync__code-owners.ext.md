----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/code-owners
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, sync, code owners
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/sync/code-owners

- [Home](/)
- [Commands](/docs/commands)
- [sync](/docs/commands/sync)
- [code-owners](/docs/commands/sync/code-owners)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# sync code-owners

v1.8.0

The `moon sync code-owners` command will manually sync code owners, by aggregating all owners from
projects, and generating a single `CODEOWNERS` file. Refer to the official
[code owners](/docs/guides/codeowners) guide for more information.

```
$ moon sync code-owners
```

### Options[​](#options)

- `--clean` - Clean and remove previously generated file.

- `--force` - Bypass cache and force create file.

### Configuration[​](#configuration)

- [`codeowners`](/docs/config/workspace#codeowners) in `.moon/workspace.yml`

- [`owners`](/docs/config/project#owners) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/sync/code-owners.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
