----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/project
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, project
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/project

- [Home](/)
- [Commands](/docs/commands)
- [project](/docs/commands/project)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# project

The `moon project [id]` (or `moon p`) command will display all available information about a project
that has been configured and exists within the graph. If a project does not exist, the program will
return with a 1 exit code.

```
$ moon project web
```

### Arguments[​](#arguments)

- `[id]` - ID or alias of a project, as defined in [`projects`](/docs/config/workspace#projects).

### Options[​](#options)

- `--json` - Print the project and its configuration as JSON.

- `--no-tasks` - Do not list tasks for the project.

## Example output[​](#example-output)

The following output is an example of what this command prints, using our very own
`@moonrepo/runtime` package.

```
RUNTIMEProject: runtimeAlias: @moonrepo/runtimeSource: packages/runtimeRoot: ~/Projects/moon/packages/runtimeToolchain: nodeLanguage: typescriptStack: unknownType: libraryDEPENDS ON  - types (implicit, production)INHERITS FROM  - .moon/tasks/node.ymlTASKSbuild:  › packemon build --addFiles --addExports --declarationformat:  › prettier --check --config ../../prettier.config.js --ignore-path ../../.prettierignore --no-error-on-unmatched-pattern .lint:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives .lint-fix:  › eslint --cache --cache-location ./.eslintcache --color --ext .js,.ts,.tsx --ignore-path ../../.eslintignore --exit-on-fatal-error --no-error-on-unmatched-pattern --report-unused-disable-directives . --fixtest:  › jest --cache --color --preset jest-preset-moon --passWithNoTeststypecheck:  › tsc --buildFILE GROUPSconfigs:  - packages/runtime/*.{js,json}sources:  - packages/runtime/src/**/*  - packages/runtime/types/**/*tests:  - packages/runtime/tests/**/*
```

### Configuration[​](#configuration)

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

- [`project`](/docs/config/project#project) in `moon.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/project.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
