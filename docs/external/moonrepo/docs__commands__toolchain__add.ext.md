----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/toolchain/add
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, toolchain, add
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/toolchain/add

- [Home](/)
- [Commands](/docs/commands)
- [toolchain](/docs/commands/toolchain)
- [add](/docs/commands/toolchain/add)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# toolchain add

v1.38.0

The `moon toolchain add  [plugin]` command will add a toolchain to the workspace by injecting a
configuration block into `.moon/toolchains.yml`. To do this, the command will download the WASM
plugin, extract information, and call initialize functions.

For built-in toolchains, the [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) argument is optional, and will be derived
from the identifier.

```
$ moon toolchain add typescript
```

For third-party toolchains, the [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) argument is required, and must point to
the WASM plugin.

```
$ moon toolchain add custom https://example.com/path/to/plugin.wasm
```

### Arguments[​](#arguments)

- `` - ID of the toolchain to use.

- `[plugin]` - Optional [plugin locator](/docs/guides/wasm-plugins#configuring-plugin-locations) for third-party toolchains.

### Options[​](#options)

- `--minimal` - Generate minimal configurations and sane defaults.

- `--yes` - Skip all prompts and enables tools based on file detection.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/toolchain/add.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
