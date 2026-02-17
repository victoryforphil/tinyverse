----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/nuxt
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, examples, nuxt
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/examples/nuxt

- [Home](/)
- JavaScript
- [Examples](/docs/guides/node/examples)
- [Nuxt](/docs/guides/examples/nuxt)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Nuxt example

In this guide, you'll learn how to integrate [Nuxt v3](https://nuxt.com), a [Vue](/docs/guides/examples/vue) framework,
into moon.

Begin by creating a new Nuxt project at a specified folder path (this should not be created in the
workspace root, unless a polyrepo).

```
cd apps && npx nuxi init

```

View the [official Nuxt docs](https://nuxt.com/docs/getting-started/installation) for a more
in-depth guide to getting started!

## Setup[​](#setup)

Since Nuxt is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

/moon.yml

```
fileGroups:  nuxt:    - 'assets/**/*'    - 'components/**/*'    - 'composables/**/*'    - 'content/**/*'    - 'layouts/**/*'    - 'middleware/**/*'    - 'pages/**/*'    - 'plugins/**/*'    - 'public/**/*'    - 'server/**/*'    - 'utils/**/*'    - '.nuxtignore'    - 'app.config.*'    - 'app.vue'    - 'nuxt.config.*'tasks:  nuxt:    command: 'nuxt'    preset: 'server'  # Production build  build:    command: 'nuxt build'    inputs:      - '@group(nuxt)'    outputs:      - '.nuxt'      - '.output'  # Development server  dev:    command: 'nuxt dev'    preset: 'server'  # Preview production build locally  preview:    command: 'nuxt preview'    deps:      - '~:build'    preset: 'server'
```

Be sure to keep the `postinstall` script in your project's `package.json`.

/package.json

```
{  // ...  "scripts": {    "postinstall": "nuxt prepare"  }}
```

### ESLint integration[​](#eslint-integration)

Refer to our [Vue documentation](/docs/guides/examples/vue#eslint-integration) for more information on linting.

### TypeScript integration[​](#typescript-integration)

Nuxt requires `vue-tsc` for typechecking, so refer to our
[Vue documentation](/docs/guides/examples/vue#typescript-integration) for more information.

## Configuration[​](#configuration)

### Root-level[​](#root-level)

We suggest against root-level configuration, as Nuxt should be installed per-project, and the
`nuxt` command expects the configuration to live relative to the project root.

### Project-level[​](#project-level)

When creating a new Nuxt project, a
[`nuxt.config.ts`](https://v3.nuxtjs.org/api/configuration/nuxt-config) is created, and must exist
in the project root. This allows each project to configure Next.js for their needs.

/nuxt.config.ts

```
export default defineNuxtConfig({});
```

## Testing[​](#testing)

Nuxt supports testing through [Jest](https://jestjs.io/) or [Vitest](https://vitest.dev/). Refer to
our [Jest documentation](/docs/guides/examples/jest) or [Vitest documentation](/docs/guides/examples/vite) for more information on testing.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/examples/nuxt.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
