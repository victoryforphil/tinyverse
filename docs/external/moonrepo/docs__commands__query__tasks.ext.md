----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/tasks
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, query, tasks
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/query/tasks

- [Home](/)
- [Commands](/docs/commands)
- [query](/docs/commands/query)
- [tasks](/docs/commands/query/tasks)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# query tasks

Use the `moon query tasks` sub-command to query task information for all projects in the project
graph. The tasks list can be filtered by passing a [query statement](/docs/concepts/query-lang) as
an argument, or by using [options](#options) arguments.

```
# Find all tasks grouped by project$ moon query tasks# Find all tasks from projects with an id that matches "react"$ moon query tasks --id react$ moon query tasks "task~react"
```

This will output a list of projects as JSON. The output has the following structure:

```
{	tasks: Record>,	options: QueryOptions,}
```

### Arguments[​](#arguments)

- `[query]` - An optional [query statement](/docs/concepts/query-lang) to filter projects with. When provided, all [filter options](#filters) are ignored. v1.4.0

### Options[​](#options)

#### Affected[​](#affected)

- `--affected` - Filter tasks that have been affected by touched files.

- `--downstream` - Include downstream dependents of queried tasks. Supports "none" (default), "direct", "deep". v1.30.0

- `--upstream` - Include upstream dependencies of queried tasks. Supports "none", "direct", "deep" (default). v1.30.0

#### Filtersv1.30.0[​](#filters)

All option values are case-insensitive regex patterns.

- `--command ` - Filter tasks that match this command.

- `--id ` - Filter tasks that match this ID.

- `--project ` - Filter tasks that belong to this project.

- `--script ` - Filter tasks that match this script.

- `--toolchain ` - Filter tasks of this toolchain. v1.31.0

- `--type ` - Filter tasks of this type.

### Configuration[​](#configuration)

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/query/tasks.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
