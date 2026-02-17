----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/projects
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, projects
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/projects

- [Home](/)
- [Commands](/docs/commands)
- [projects](/docs/commands/projects)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# projects

v2.0.0

The `moon projects` command will list all projects configured in the workspace as a table of
information.

```
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮│Project          Source                    Stack             Layer             Toolchains                                Description                                                   ││───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────││types            packages/types            frontend          library           javascript, node, typescript, yarn                                                                      ││website          website                   frontend          application       javascript, node, typescript, yarn        A static website powered by Docusaurus.                       │╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

info

Use [`moon query projects`](/docs/commands/query/projects) for advanced querying and filtering of projects.

### Options[​](#options)

- `--json` - Print the projects as JSON.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/projects.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
