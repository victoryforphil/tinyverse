----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/file-group
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, concepts, file group
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/concepts/file-group

- [Home](/)
- [Concepts](/docs/concepts)
- [File groups](/docs/concepts/file-group)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# File groups

File groups are a mechanism for grouping similar types of files and environment variables within a
project using [file glob patterns or literal file paths](/docs/concepts/file-pattern). These groups are then used
by [tasks](/docs/concepts/task) to calculate functionality like cache computation, affected files since last
change, deterministic builds, and more.

## Configuration[​](#configuration)

File groups can be configured per project through [`moon.yml`](/docs/config/project), or for many
projects through [`.moon/tasks/all.yml`](/docs/config/tasks).

### Token functions[​](#token-functions)

File groups can be referenced in [tasks](/docs/concepts/task) using [token functions](/docs/concepts/token). For example, the
`@group(name)` token will expand to all paths configured in the `sources` file group.

moon.yml

```
tasks:  build:    command: 'vite build'    inputs:      - '@group(sources)'
```

## Inheritance and merging[​](#inheritance-and-merging)

When a file group of the same name exists in both [configuration files](#configuration), the
project-level group will override the workspace-level group, and all other workspace-level groups
will be inherited as-is.

A primary scenario in which to define file groups at the project-level is when you want to
override file groups defined at the workspace-level. For example, say we want to override the
`sources` file group because our source folder is named "lib" and not "src", we would define our
file groups as followed.

.moon/tasks/all.yml

```
fileGroups:  sources:    - 'src/**/*'    - 'types/**/*'  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

moon.yml

```
fileGroups:  # Overrides global  sources:    - 'lib/**/*'    - 'types/**/*'  # Inherited as-is  tests:    - 'tests/**/*.test.*'    - '**/__tests__/**/*'
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/concepts/file-group.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
