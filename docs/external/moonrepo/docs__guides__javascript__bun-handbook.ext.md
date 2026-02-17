----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/javascript/bun-handbook
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, javascript, bun handbook
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/javascript/bun-handbook

- [Home](/)
- JavaScript
- [Bun handbook](/docs/guides/javascript/bun-handbook)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Bun handbook

Utilizing JavaScript (and TypeScript) in a monorepo can be a daunting task, especially when using
Bun (or Node.js), as there are many ways to structure your code and to configure your tools. With
this handbook, we'll help guide you through this process.

info

This guide is a living document and will continue to be updated over time!

## moon setup[​](#moon-setup)

For this part of the handbook, we'll be focusing on [moon](/moon), our task runner. To start,
languages in moon act like plugins, where their functionality and support is not enabled unless
explicitly configured. We follow this approach to avoid unnecessary overhead.

### Enabling the language[​](#enabling-the-language)

To enable JavaScript support via Bun, define the [`bun`](/docs/config/toolchain#bun) setting in
[`.moon/toolchains.yml`](/docs/config/toolchain), even if an empty object.

.moon/toolchains.yml

```
# Enable Bunbun: {}
```

info

In moon v1.40+, use `javascript` and `bun` instead of `bun` to enable the new WASM powered Bun
toolchain, which is far more accurate and efficient. The non-WASM toolchain will be deprecated in
the future.

Or by pinning a `bun` version in [`.prototools`](/docs/proto/config) in the workspace root.

.prototools

```
bun = "1.0.0"
```

This will enable the Bun toolchain and provide the following automations around its ecosystem:

- Node modules will automatically be installed if dependencies in `package.json` have changed, or the lockfile has changed, since the last time a task has ran. We'll also take `package.json` workspaces into account and install modules in the correct location; either the workspace root, in a project, or both.

- Relationships between projects will automatically be discovered based on `dependencies`, `devDependencies`, and `peerDependencies` in `package.json`.

### Utilizing the toolchain[​](#utilizing-the-toolchain)

When a language is enabled, moon by default will assume that the language's binary is available
within the current environment (typically on `PATH`). This has the downside of requiring all
developers and machines to manually install the correct version of the language, and to stay in
sync.

Instead, you can utilize [moon's toolchain](/docs/concepts/toolchain), which will download and
install the language in the background, and ensure every task is executed using the exact version
across all machines.

Enabling the toolchain is as simple as defining the [`bun.version`](/docs/config/toolchain#version)
setting.

.moon/toolchains.yml

```
# Enable Bun toolchain with an explicit versionbun:  version: '1.0.0'
```

Versions can also be defined with [`.prototools`](/docs/proto/config).

### Configuring the toolchain[​](#configuring-the-toolchain)

Since the JavaScript ecosystem supports multiple runtimes, moon is unable to automatically detect
the correct runtime for all scenarios. Does the existence of a `package.json` mean Node.js or Bun?
We don't know, and default to Node.js because of its popularity.

To work around this, you can set `toolchain` to "bun" at the task-level or project-level.

moon.yml

```
# For all tasks in the projecttoolchain:  default: 'bun'tasks:  build:    command: 'webpack'    # For this specific task    toolchain: 'bun'
```

The task-level `toolchain.default` only needs to be set if executing a `node_modules` binary! The
`bun` binary automatically sets the toolchain to Bun.

### Using `package.json` scripts[​](#using-packagejson-scripts)

If you're looking to prototype moon, or reduce the migration effort to moon tasks, you can configure
moon to inherit `package.json` scripts, and internally convert them to moon tasks. This can be
achieved with the [`bun.inferTasksFromScripts`](/docs/config/toolchain#infertasksfromscripts)
setting.

.moon/toolchains.yml

```
bun:  inferTasksFromScripts: true
```

Or you can run scripts through `bun run` calls.

moon.yml

```
tasks:  build:    command: 'bun run build'
```

## Handbook[​](#handbook)

info

Refer to the [Node.js handbook](/docs/guides/javascript/node-handbook) for more information on repository structure,
dependency management, and more. Since both runtimes are extremely similar, the information in that
handbook also applies to Bun!

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/javascript/bun-handbook.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
