----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/react
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, examples, react
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/examples/react

- [Home](/)
- JavaScript
- [Examples](/docs/guides/node/examples)
- [React](/docs/guides/examples/react)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# React example

React is an application or library concern, and not a build system one, since the bundling of React
is abstracted away through another tool like webpack. Because of this, moon has no guidelines around
utilizing React directly. You can use React however you wish!

However, with that being said, we do suggest the following:

- Add `react` and related dependencies to each project, not the root. This includes `@types/react` as well. This will ensure accurate [hashing](/docs/concepts/cache#hashing).

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add react
```

```
yarn workspace
 add react
```

```
npm install --workspace
 react
```

```
pnpm add --filter
 react
```

```
bun install react
```

- Configure Babel with the `@babel/preset-react` preset.

- Configure [TypeScript](/docs/guides/examples/typescript) compiler options with `"jsx": "react-jsx"`.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/examples/react.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
