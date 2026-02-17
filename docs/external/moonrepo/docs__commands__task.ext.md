----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/task
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, task
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/task

- [Home](/)
- [Commands](/docs/commands)
- [task](/docs/commands/task)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# task

v1.1.0

The `moon task [target]` (or `moon t`) command will display information about a task that has been
configured and exists within a project. If a task does not exist, the program will return with a 1
exit code.

```
$ moon task web:build
```

### Arguments[​](#arguments)

- `[target]` - Fully qualified project + task target.

### Options[​](#options)

- `--json` - Print the task and its configuration as JSON.

## Example output[​](#example-output)

The following output is an example of what this command prints, using our very own
`@moonrepo/runtime` package.

```
RUNTIME:BUILDTask: buildProject: runtimeToolchain: nodeType: buildPROCESSCommand: packemon build --addFiles --addExports --declarationEnvironment variables:  - NODE_ENV = productionWorking directory: ~/Projects/moon/packages/runtimeRuns dependencies: ConcurrentlyRuns in CI: YesDEPENDS ON  - types:buildINHERITS FROM  - .moon/tasks/node.ymlINPUTS  - .moon/*.yml  - .moon/tasks/node.yml  - packages/runtime/package.json  - packages/runtime/src/**/*  - packages/runtime/tsconfig.*.json  - packages/runtime/tsconfig.json  - packages/runtime/types/**/*  - tsconfig.options.jsonOUTPUTS  - packages/runtime/cjs
```

### Configuration[​](#configuration)

- [`tasks`](/docs/config/tasks#tasks) in `.moon/tasks/all.yml`

- [`tasks`](/docs/config/project#tasks) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/task.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
