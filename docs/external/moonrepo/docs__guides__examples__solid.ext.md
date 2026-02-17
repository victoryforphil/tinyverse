----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/solid
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, examples, solid
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/examples/solid

- [Home](/)
- JavaScript
- [Examples](/docs/guides/node/examples)
- [Solid](/docs/guides/examples/solid)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Solid example

[Solid](https://www.solidjs.com) (also known as SolidJS) is a JavaScript framework for building
interactive web applications. Because of this, Solid is an application or library concern, and not a
build system one, since the bundling of Solid is abstracted away through the application or a
bundler.

With that being said, we do have some suggestions on utilizing Solid effectively in a monorepo. To
begin, install Solid to a project.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add solid-js
```

```
yarn workspace
 add solid-js
```

```
npm install --workspace
 solid-js
```

```
pnpm add --filter
 solid-js
```

```
bun install solid-js
```

## Setup[​](#setup)

Solid utilizes JSX for rendering markup, which requires
[`babel-preset-solid`](https://www.npmjs.com/package/babel-preset-solid) for parsing and
transforming. To enable the preset for the entire monorepo, add the preset to a root
`babel.config.js`, otherwise add it to a `.babelrc.js` in each project that requires it.

```
module.exports = {  presets: ['solid'],};
```

### TypeScript integration[​](#typescript-integration)

For each project using Solid, add the following compiler options to the `tsconfig.json` found in the
project root.

/tsconfig.json

```
{  "compilerOptions": {    "jsx": "preserve",    "jsxImportSource": "solid-js"  }}
```

### Vite integration[​](#vite-integration)

If you're using a [Vite](/docs/guides/examples/vite) powered application (Solid Start or starter templates), you should
enable [`vite-plugin-solid`](https://www.npmjs.com/package/vite-plugin-solid) instead of configuring
Babel. Be sure to read our [guide on Vite](/docs/guides/examples/vite) as well!

/vite.config.js

```
import { defineConfig } from 'vite';import solidPlugin from 'vite-plugin-solid';export default defineConfig({  // ...  plugins: [solidPlugin()],});
```

Tags:
- [solid](/docs/tags/solid)
- [solidjs](/docs/tags/solidjs)

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/examples/solid.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
