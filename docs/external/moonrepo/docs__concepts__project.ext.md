----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/concepts/project
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, concepts, project
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/concepts/project

- [Home](/)
- [Concepts](/docs/concepts)
- [Projects](/docs/concepts/project)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Projects

A project is a library, application, package, binary, tool, etc, that contains source files, test
files, assets, resources, and more. A project must exist and be configured within a
[workspace](/docs/concepts/workspace).

## IDs[​](#ids)

A project identifier (or name) is a unique resource for locating a project. The ID is explicitly
configured within [`.moon/workspace.yml`](/docs/config/workspace), as a key within the
[`projects`](/docs/config/workspace#projects) setting, and can be written in camel/kebab/snake case.
IDs support alphabetic unicode characters, `0-9`, `_`, `-`, `/`, `.`, and must start with a
character.

IDs are used heavily by configuration and the command line to link and reference everything. They're
also a much easier concept for remembering projects than file system paths, and they typically can
be written with less key strokes.

Lastly, a project ID can be paired with a task ID to create a [target](/docs/concepts/target).

## Aliases[​](#aliases)

Aliases are a secondary approach for naming projects, and can be used as a drop-in replacement for
standard names. What this means is that an alias can also be used when configuring dependencies, or
defining [targets](/docs/concepts/target).

However, the difference between aliases and names is that aliases can not be explicit configured
in moon. Instead, they are specific to a project's primary programming language, and are inferred
based on that context (when enabled in settings). For example, a JavaScript or TypeScript project
will use the `name` field from its `package.json` as the alias.

Because of this, a project can either be referenced by its name or alias, or both. Choose the
pattern that makes the most sense for your company or team!

## Dependencies[​](#dependencies)

Projects can depend on other projects within the [workspace](/docs/concepts/workspace) to build a
[project graph](/docs/how-it-works/action-graph), and in turn, an action graph for executing
[tasks](/docs/concepts/task). Project dependencies are divided into 2 categories:

- Explicit dependencies - These are dependencies that are explicitly defined in a project's [`moon.yml`](/docs/config/project) config file, using the [`dependsOn`](/docs/config/project#dependson) setting.

- Implicit dependencies - These are dependencies that are implicitly discovered by moon when scanning the repository. How an implicit dependency is discovered is based on the project's [`language`](/docs/config/project#language) setting, and how that language's ecosystem functions.

## Configuration[​](#configuration)

Projects can be configured with an optional [`moon.yml`](/docs/config/project) in the project root, or
through the optional workspace-level [`.moon/tasks/all.yml`](/docs/config/tasks).

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/concepts/project.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
