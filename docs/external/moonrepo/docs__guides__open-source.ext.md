----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/open-source
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, open source
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/open-source

- [Home](/)
- [Open source usage](/docs/guides/open-source)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Open source usage

Although moon was designed for large monorepos, it can also be used for open source projects,
especially when coupled with our [built-in continuous integration support](/docs/guides/ci).

However, a pain point with moon is that it has an explicitly configured version for each tool in the
[toolchain](/docs/concepts/toolchain), but open source projects typically need to run checks against
multiple versions! To mitigate this problem, you can set the matrix value as an environment
variable, in the format of `MOON__VERSION`.

.github/workflows/ci.yml

```
name: 'Pipeline'on:  push:    branches:      - 'master'  pull_request:jobs:  ci:    name: 'CI'    runs-on: ${{ matrix.os }}    strategy:      matrix:        os: ['ubuntu-latest', 'windows-latest']        node-version: [16, 18, 20]    steps:      # Checkout repository      - uses: 'actions/checkout@v4'        with:          fetch-depth: 0      # Install Node.js      - uses: 'actions/setup-node@v6'      # Install dependencies      - run: 'yarn install --immutable'      # Run moon and affected tasks      - run: 'yarn moon ci'        env:          MOON_NODE_VERSION: ${{ matrix.node-version }}
```

info

This example is only for GitHub actions, but the same mechanism can be applied to other CI
environments.

## Reporting run results[​](#reporting-run-results)

We also suggest using our
[`moonrepo/run-report-action`](https://github.com/marketplace/actions/moon-ci-run-reports) GitHub
action. This action will report the results of a [`moon ci`](/docs/commands/ci) run to a pull request
as a comment and workflow summary.

.github/workflows/ci.yml

```
# ...jobs:  ci:    name: 'CI'    runs-on: 'ubuntu-latest'    steps:      # ...      - run: 'yarn moon ci'      - uses: 'moonrepo/run-report-action@v1'        if: success() || failure()        with:          access-token: ${{ secrets.GITHUB_TOKEN }}
```

The report looks something like the following:

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/open-source.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
