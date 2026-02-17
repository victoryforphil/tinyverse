----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/target
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, concepts, target
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/concepts/target

- [Home](/)
- [Concepts](/docs/concepts)
- [Targets](/docs/concepts/target)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Targets

A target is a compound identifier that pairs a [scope](#common-scopes) to a [task](/docs/concepts/task),
separated by a `:`, in the format of `scope:task`.

Targets are used by terminal commands...

```
$ moon run designSystem:build
```

And configurations for declaring cross-project or cross-task dependencies.

```
tasks:  build:    command: 'webpack'    deps:      - 'designSystem:build'
```

## Common scopes[​](#common-scopes)

These scopes are available for both running targets and configuring them.

### By project[​](#by-project)

The most common scope is the project scope, which requires the name of a project, as defined in
[`.moon/workspace.yml`](/docs/config/workspace). When paired with a task name, it will run a specific
task from that project.

```
# Run `lint` in project `app`$ moon run app:lint
```

### By tagv1.4.0[​](#by-tag)

Another way to target projects is with the tag scope, which requires the name of a tag prefixed with
`#`, and will run a specific task in all projects with that tag.

```
# Run `lint` in projects with the tag `frontend`$ moon run '#frontend:lint'
```

caution

Because `#` is a special character in the terminal (is considered a comment), you'll need to wrap
the target in quotes, or escape it like so `\#`.

## Run scopes[​](#run-scopes)

These scopes are only available on the command line when running tasks with `moon run` or `moon ci`.

### All projects[​](#all-projects)

For situations where you want to run a specific target in all projects, for example `lint`ing, you
can utilize the all projects scope by omitting the project name from the target: `:lint`.

```
# Run `lint` in all projects$ moon run :lint
```

### Closest project `~`v1.33.0[​](#closest-project-)

If you are within a project folder, or an arbitrarily nested folder, and want to run a task in the
closest project (traversing upwards), the `~` scope can be used.

```
# Run `lint` in the closest project$ moon run ~:lint
```

## Config scopes[​](#config-scopes)

These scopes are only available when configuring a task.

### Dependencies `^`[​](#dependencies-)

When you want to include a reference for each project [that's depended on](/docs/concepts/project#dependencies),
you can utilize the `^` scope. This will be expanded to all depended on projects. If you do not
want all projects, then you'll need to explicitly define them.

moon.yml

```
dependsOn:  - 'apiClients'  - 'designSystem'# Configured astasks:  build:    command: 'webpack'    deps:      - '^:build'# Resolves totasks:  build:    command: 'webpack'    deps:      - 'apiClients:build'      - 'designSystem:build'
```

### Self `~`[​](#self-)

When referring to another task within the current project, you can utilize the `~` scope, or omit
the `~:` prefix altogether, which will be expanded to the current project's name. This is useful for
situations where the name is unknown, for example, when configuring
[`.moon/tasks/all.yml`](/docs/config/tasks), or if you just want a shortcut!

.moon/tasks/all.yml

```
# Configured astasks:  lint:    command: 'eslint'    deps:      - '~:typecheck'      # OR      - 'typecheck'  typecheck:    command: 'tsc'# Resolves to (assuming project is "foo")tasks:  lint:    command: 'eslint'    deps:      - 'foo:typecheck'  typecheck:    command: 'tsc'
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/concepts/target.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
