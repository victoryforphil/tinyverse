----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/run
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, run
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/run

- [Home](/)
- [Commands](/docs/commands)
- [run](/docs/commands/run)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# run

The `moon run` (or `moon r`) command will run one or many [targets](/docs/concepts/target) and all of
its dependencies in topological order. Each run will incrementally cache each task, improving speed
and development times... over time. View the official [Run a task](/docs/run-task) and
[Cheat sheet](/docs/cheat-sheet#tasks) articles for more information!

```
# Run `lint` in project `app`$ moon run app:lint# Run `dev` in project `client` and `server`$ moon run client:dev server:dev# Run `test` in all projects$ moon run :test# Run `test` in all projects with tag `frontend`$ moon run '#frontend:test'# Run `format` in default project$ moon run format# Run `build` in projects matching the query$ moon run :build --query "language=javascript && projectLayer=library"
```

info

The default behavior for `moon run` is to "fail fast", meaning that any failed task will immediately
abort execution of the entire action graph. Use `moon exec --on-failure continue` for alternative
behavior.

### Arguments[​](#arguments)

- `...` - [Targets](/docs/concepts/target) or project relative tasks to run.

- `[-- ]` - Additional arguments to [pass to the underlying command](/docs/run-task#passing-arguments-to-the-underlying-command).

### Options[​](#options)

Inherits all options from [`moon exec`](/docs/commands/exec) except for `--on-failure`.

### Configuration[​](#configuration)

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`tasks`](/docs/config/tasks#tasks) in `.moon/tasks/all.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/run.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
