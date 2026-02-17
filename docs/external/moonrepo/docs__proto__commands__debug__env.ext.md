----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/debug/env
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, debug, env
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/debug/env

- [Home](/)
- Commands
- [debug](/docs/proto/commands/debug)
- [env](/docs/proto/commands/debug/env)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# debug env

v0.26.0

The `proto debug env` command will print information about your current proto environment. Primarily
the store location, relevant file paths, and environment variables.

```
$ proto debug envStore ─────────────────────────────────────────────────────────────────────  Root: /Users/name/.proto  Bins: /Users/name/.proto/bin  Shims: /Users/name/.proto/shims  Plugins: /Users/name/.proto/plugins  Tools: /Users/name/.proto/tools  Temp: /Users/name/.proto/tempEnvironment ───────────────────────────────────────────────────────────────  Proto version: 0.44.0  Operating system: macos  Architecture: arm64  Config sources:    - /Users/name/Projects/example/.prototools    - /Users/name/.proto/.prototools  Virtual paths:    /userhome = /Users/name    /proto = /Users/name/.proto  Environment variables:    PROTO_APP_LOG = proto=info,schematic=info,starbase=info,warpgate=info,extism::pdk=info    PROTO_HOME = /Users/name/.proto    PROTO_OFFLINE_TIMEOUT = 750    PROTO_VERSION = 0.44.0
```

### Options[​](#options)

- `--json` - Print the list in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/debug/env.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
