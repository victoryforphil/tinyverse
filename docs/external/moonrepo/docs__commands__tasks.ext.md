----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/tasks
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, tasks
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/tasks

- [Home](/)
- [Commands](/docs/commands)
- [tasks](/docs/commands/tasks)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# tasks

v2.0.0

The `moon tasks` command will list all tasks available in the workspace as a table of information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Task                          Command          Type        Preset      Toolchains                                Description                                                           ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││website:build                 docusaurus       build                   typescript, javascript, node, yarn        Builds the Docusaurus app.                                            ││website:format                prettier         test                    javascript, node, yarn                                                                                          ││website:format-write          prettier         test                    javascript, node, yarn                                                                                          ││website:lint                  eslint           test                    javascript, node, yarn                                                                                          ││website:lint-fix              eslint           test                    javascript, node, yarn                                                                                          ││website:start                 docusaurus       run         server      typescript, javascript, node, yarn                                                                              ││website:test                  jest             test                    javascript, node, yarn                                                                                          ││website:typecheck             tsc              test                    typescript, javascript, node, yarn                                                                              │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query tasks`](/docs/commands/query/tasks) for advanced querying and filtering of tasks.

### Arguments[​](#arguments)

- `[id]` - Filter tasks to a specific project ID.

### Options[​](#options)

- `--json` - Print the projects as JSON.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/tasks.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
