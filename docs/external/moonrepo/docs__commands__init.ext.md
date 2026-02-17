----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/init
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, init
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/init

- [Home](/)
- [Commands](/docs/commands)
- [init](/docs/commands/init)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# init

The `moon init` command will initialize moon into a repository and scaffold necessary config files
by creating a `.moon` folder.

```
$ moon init# In another directory$ moon init ./app
```

### Arguments[​](#arguments)

- `[dest]` - Destination to initialize and scaffold into. Defaults to `.` (current working directory).

### Options[​](#options)

- `--force` - Overwrite existing config files if they exist.

- `--minimal` - Generate minimal configurations and sane defaults.

- `--yes` - Skip all prompts and enables tools based on file detection.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/init.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
