----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/how-it-works/task-graph
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, how it works, task graph
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/how-it-works/task-graph

- [Home](/)
- [How it works](/docs/how-it-works)
- [Task graph](/docs/how-it-works/task-graph)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Task graph

The task graph is a representation of all configured
[tasks in the workspace](/docs/config/workspace#projects) and their relationships between each other,
and is represented internally as a directed acyclic graph (DAG). This graph is derived from
information in the [project graph](/docs/how-it-works/project-graph). Below is a visual representation of a task
graph.

info

The [`moon task-graph`](/docs/commands/task-graph) command can be used to view the structure of your
workspace.

## Relationships[​](#relationships)

A relationship is between a dependent (downstream task) and a dependency/requirement (upstream
task). Relationships are derived explicitly with the task [`deps`](/docs/config/project#deps) setting,
and fall into 1 of 2 categories:

### Required[​](#required)

These are dependencies that are required to run and complete with a success, before the owning task
can run. If a required dependency fails, then the owning task will abort.

### Optional[​](#optional)

The opposite of [required](#required), these are dependencies that can either a) not exist during
task inheritance, or b) run and fail without aborting the owning task.

## What is the graph used for?[​](#what-is-the-graph-used-for)

Great question, the task graph is extremely important for running tasks (duh), and it also:

- Is fed into the [action graph](/docs/how-it-works/action-graph) that can be executed in topological order.

- Determines affected tasks in [continuous integration](/docs/guides/ci) workflows.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/how-it-works/task-graph.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
