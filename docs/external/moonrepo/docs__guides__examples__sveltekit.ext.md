----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/sveltekit
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, examples, sveltekit
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/examples/sveltekit

- [Home](/)
- JavaScript
- [Examples](/docs/guides/node/examples)
- [SvelteKit](/docs/guides/examples/sveltekit)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# SvelteKit example

[SvelteKit](https://kit.svelte.dev) is built on [Svelte](https://svelte.dev), a UI framework that
uses a compiler to let you write breathtakingly concise components that do minimal work in the
browser, using languages you already know — HTML, CSS and JavaScript. It's a love letter to web
development.

```
cd apps && npm create svelte@latest

```

You will be prompted to choose between select templates, TypeScript, ESLint, Prettier, Playwright
and Vitest among other options. moon supports and has guides for many of these tools.

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite),
[using ESLint with moon](/docs/guides/examples/eslint) and [using Prettier with moon](/docs/guides/examples/prettier) for a more holistic
view.

## Setup[​](#setup)

Since SvelteKit is per-project, the associated moon tasks should be defined in each project's
[`moon.yml`](/docs/config/project) file.

tip

We suggest inheriting SvelteKit tasks from the
[official moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/sveltekit).

/moon.yml

```
# Inherit tasks from the `sveltekit` preset# https://github.com/moonrepo/moon-configstags: ['sveltekit']
```

### ESLint integration[​](#eslint-integration)

SvelteKit provides an option to setup ESLint along with your project, with moon you can use a
[global `lint` task](/docs/guides/examples/eslint). We encourage using the global `lint` task for consistency across all
projects within the repository. With this approach, the `eslint` command itself will be ran and the
`svelte3` rules will still be used.

/moon.yml

```
tasks:  # Extends the top-level lint  lint:    args:      - '--ext'      - '.ts,.svelte'
```

Be sure to enable the Svelte parser and plugin in a project local ESLint configuration file.

.eslintrc.cjs

```
module.exports = {  plugins: ['svelte3'],  ignorePatterns: ['*.cjs'],  settings: {    'svelte3/typescript': () => require('typescript'),  },  overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],};
```

### TypeScript integration[​](#typescript-integration)

SvelteKit also has built-in support for TypeScript, but has similar caveats to the
[ESLint integration](#eslint-integration). TypeScript itself is a bit involved, so we suggest
reading the official [SvelteKit documentation](https://kit.svelte.dev/docs/introduction) before
continuing.

At this point we'll assume that a `tsconfig.json` has been created in the application, and
typechecking works. From here we suggest utilizing a [global `typecheck` task](/docs/guides/examples/typescript) for
consistency across all projects within the repository. However, because Svelte isn't standard
JavaScript, it requires the use of the `svelte-check` command for type-checking.

info

The
[moon configuration preset](https://github.com/moonrepo/moon-configs/tree/master/javascript/sveltekit)
provides the `check` task below.

/moon.yml

```
workspace:  inheritedTasks:    exclude: ['typecheck']tasks:  check:    command: 'svelte-check --tsconfig ./tsconfig.json'    deps:      - 'typecheck-sync'    inputs:      - '@group(svelte)'      - 'tsconfig.json'
```

In case Svelte doesn't automatically create a `tsconfig.json`, you can use the following:

/tsconfig.json

```
{  "extends": "./.svelte-kit/tsconfig.json",  "compilerOptions": {    "allowJs": true,    "checkJs": true,    "esModuleInterop": true,    "forceConsistentCasingInFileNames": true,    "resolveJsonModule": true,    "skipLibCheck": true,    "sourceMap": true,    "strict": true  }}
```

## Configuration[​](#configuration)

### Root-level[​](#root-level)

We suggest against root-level configuration, as SvelteKit should be installed per-project, and the
`vite` command expects the configuration to live relative to the project root.

### Project-level[​](#project-level)

When creating a new SvelteKit project, a
[`svelte.config.js`](https://kit.svelte.dev/docs/configuration) is created, and must exist in the
project root. This allows each project to configure SvelteKit for their needs.

/svelte.config.js

```
import adapter from '@sveltejs/adapter-auto';import { vitePreprocess } from '@sveltejs/kit/vite';/** @type {import('@sveltejs/kit').Config} */const config = {  // Consult https://kit.svelte.dev/docs/integrations#preprocessors  // for more information about preprocessors  preprocess: vitePreprocess(),  kit: {    adapter: adapter(),  },};export default config;
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/examples/sveltekit.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
