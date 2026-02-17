----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/sync/projects
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, sync, projects
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/sync/projects

- [Home](/)
- [Commands](/docs/commands)
- [sync](/docs/commands/sync)
- [projects](/docs/commands/sync/projects)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# sync projects

v1.8.0

The `moon sync projects` command will force sync all projects in the workspace to help achieve a
[healthy repository state](/docs/faq#what-should-be-considered-the-source-of-truth). This applies
the following:

- Ensures cross-project dependencies are linked based on [`dependsOn`](/docs/config/project#dependson).

- Ensures language specific configuration files are present and accurate (`package.json`, `tsconfig.json`, etc).

- Ensures root configuration and project configuration are in sync.

- Any additional language specific semantics that may be required.

```
$ moon sync projects
```

This command should rarely be ran, as [`moon run`](/docs/commands/run) will sync affected projects
automatically! However, when migrating or refactoring, manual syncing may be necessary.

### Configuration[​](#configuration)

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/sync/projects.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
