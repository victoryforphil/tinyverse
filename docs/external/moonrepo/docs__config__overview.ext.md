----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/overview
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, config, overview
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/config/overview

- [Home](/)
- [Config files](/docs/config)
- [Overview](/docs/config/overview)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Overview

## Supported formatsv2.0.0[​](#supported-formats)

In moon, you can define configuration files in a variety of formats. We currently support the
following:

- JSON (`.json`)

- JSON with comments (`.jsonc`)

- [HCL](https://github.com/hashicorp/hcl) (`.hcl`)

- [Pkl](https://pkl-lang.org/) (`.pkl`)

- [TOML](https://toml.io/en/) (`.toml`)

- YAML (`.yml`, `.yaml`)

info

In moon v1, only YAML (`.yml`) and Pkl (`.pkl`) configuration files were supported.

## Schema validationv1.33.0[​](#schema-validation)

We support schema validation for all configuration files through
[JSON Schema](https://json-schema.org/), even for formats that are not JSON (depends on tool/editor
support). To reference the schema for a specific configuration file, configure the `$schema`
property at the top of the file with the appropriate schema found at `.moon/cache/schemas`.

- .moon/workspace
- .moon/extensions
- .moon/toolchains
- .moon/tasks
- moon
- template

.moon/workspace.yml

```
$schema: './cache/schemas/workspace.json'
```

.moon/extensions.yml

```
$schema: './cache/schemas/extensions.json'
```

.moon/toolchains.yml

```
$schema: './cache/schemas/toolchains.json'
```

.moon/tasks/all.yml

```
$schema: '../cache/schemas/tasks.json'
```

moon.yml

```
$schema: '../path/to/.moon/cache/schemas/project.json'
```

template.yml

```
$schema: '../path/to/.moon/cache/schemas/template.json'
```

info

The schemas are automatically created when running a task. If they do not exist yet, you can run
[`moon sync config-schemas`](/docs/commands/sync/config-schemas) to generate them manually.

danger

In older versions of moon, the schema files were located at `https://moonrepo.dev/schemas`. These
URLs are now deprecated, as they do not support dynamic settings. Please update your `$schema`
references to point to the local schema files in `.moon/cache/schemas`.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/config/overview.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
