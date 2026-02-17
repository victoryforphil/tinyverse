----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/how-it-works/project-graph
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, how it works, project graph
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/how-it-works/project-graph

- [Home](/)
- [How it works](/docs/how-it-works)
- [Project graph](/docs/how-it-works/project-graph)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Project graph

The project graph is a representation of all configured
[projects in the workspace](/docs/config/workspace#projects) and their relationships between each
other, and is represented internally as a directed acyclic graph (DAG). Below is a visual
representation of a project graph, composed of multiple applications and libraries, where both
project types depend on libraries.

info

The [`moon project-graph`](/docs/commands/project-graph) command can be used to view the structure of
your workspace.

## Relationships[​](#relationships)

A relationship is between a dependent (downstream project) and a dependency/requirement (upstream
project). Relationships are derived from source code and configuration files within the repository,
and fall into 1 of 2 categories:

### Explicit[​](#explicit)

These are dependencies that are explicitly defined in a project's [`moon.yml`](/docs/config/project)
config file, using the [`dependsOn`](/docs/config/project#dependson) setting.

moon.yml

```
dependsOn:  - 'components'  - id: 'utils'    scope: 'peer'
```

### Implicit[​](#implicit)

These are dependencies that are implicitly discovered by moon when scanning the repository. How an
implicit dependency is discovered is based on a
[language's platform integration](/docs/how-it-works/languages#tier-2--platform), and how that language's ecosystem
functions.

package.json

```
{  // ...  "dependencies": {    "@company/components": "workspace:*"  },  "peerDependencies": {    "@company/utils": "workspace:*"  }}
```

caution

If a language is not officially supported by moon, then implicit dependencies will not be
resolved. For unsupported languages, you must explicitly configure dependencies.

### Scopes[​](#scopes)

Every relationship is categorized into a scope that describes the type of relationship between the
parent and child. Scopes are currently used for [project syncing](/docs/commands/sync) and deep Docker
integration.

- Production - Dependency is required in production, will not be pruned in production environments, and will sync as a production dependency.

- Development - Dependency is required in development and production, will be pruned from production environments, and will sync as a development-only dependency.

- Build - Dependency is required for building only, and will sync as a build dependency.

- Peer - Dependency is a peer requirement, with language specific semantics. Will sync as a peer dependency when applicable.

## What is the graph used for?[​](#what-is-the-graph-used-for)

Great question, the project graph is used throughout the codebase to accomplish a variety of
functions, but mainly:

- Is fed into the [task graph](/docs/how-it-works/task-graph) to determine relationships of tasks between other tasks, and across projects.

- Powers our [Docker](/docs/guides/docker) layer caching and scaffolding implementations.

- Utilized for [project syncing](/docs/commands/sync) to ensure a healthy repository state.

- Determines affected projects in [continuous integration](/docs/guides/ci) workflows.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/how-it-works/project-graph.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
