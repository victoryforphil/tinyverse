----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/toolchain/info
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, toolchain, info
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/toolchain/info

- [Home](/)
- [Commands](/docs/commands)
- [toolchain](/docs/commands/toolchain)
- [info](/docs/commands/toolchain/info)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# toolchain info

v1.38.0

The `moon toolchain info  [plugin]` command will display detailed information about a toolchain,
like what files are scanned, what configuration settings are available, and what tier APIs are
supported. To do this, the command will download the WASM plugin, extract information, and call
specific functions.

For built-in toolchains, the [plugin locator][locator] argument is optional, and will be derived
from the identifier.

```
$ moon toolchain info typescript
```

For third-party toolchains, the [plugin locator][locator] argument is required, and must point to
the WASM plugin.

```
$ moon toolchain info custom https://example.com/path/to/plugin.wasm
```

### Arguments[​](#arguments)

- `` - ID of the toolchain to view.

- `[plugin]` - Optional [plugin locator][locator] for third-party toolchains.

## Example output[​](#example-output)

```
Toolchain ─────────────────────────────────────────────────────────────────  Provides sync operations that keep tsconfig.json's in a healthy state.  ID: typescript  Name: TypeScript  Version: 0.2.0Configuration ─────────────────────────────────────────────────────────────  createMissingConfig: bool  When `syncProjectReferences` is enabled, will create a `tsconfig.json`  in referenced projects if it does not exist.  includeProjectReferenceSources: bool  Appends sources of project reference to `include` in `tsconfig.json`,  for each project.  includeSharedTypes: bool  Appends shared types to `include` in `tsconfig.json`, for each project.  projectConfigFileName: string  Name of the `tsconfig.json` file within each project.  root: string  The relative root to the TypeScript root. Primarily used for  resolving project references.  rootConfigFileName: string  Name of the `tsconfig.json` file at the workspace root.  rootOptionsConfigFileName: string  Name of the shared compiler options `tsconfig.json` file  at the workspace root.  routeOutDirToCache: bool  Updates and routes `outDir` in `tsconfig.json` to moon's cache,  for each project.  syncProjectReferences: bool  Syncs all project dependencies as `references` in `tsconfig.json`,  for each project.  syncProjectReferencesToPaths: bool  Syncs all project dependencies as `paths` in `tsconfig.json`,  for each project.Tier 1 - Usage detection ──────────────────────────────────────────────────  Config files: tsconfig.json, tsconfig.*.json, *.tsconfig.json, .tsbuildinfo, *.tsbuildinfo  Executable names: tsc, tsserver  APIs:    🟢 register_toolchain (required)    🟢 define_toolchain_config    🟢 initialize_toolchain    ⚫️ detect_version_files    ⚫️ parse_version_file    🟢 define_docker_metadata    ⚫️ scaffold_docker    ⚫️ prune_docker    🟢 sync_project    ⚫️ sync_workspaceTier 2 - Ecosystem integration ─────────────────────────────────────────────  APIs:    ⚫️ extend_project_graph    ⚫️ extend_task_command    ⚫️ extend_task_script    ⚫️ locate_dependencies_root    ⚫️ install_dependencies    🟢 hash_task_contents    ⚫️ parse_lock    ⚫️ parse_manifest    ⚫️ setup_environmentTier 3 - Tool management ──────────────────────────────────────────────────  APIs:    ⚫️ register_tool (required)    ⚫️ load_versions    ⚫️ resolve_version    ⚫️ download_prebuilt (required)    ⚫️ unpack_archive    ⚫️ locate_executables (required)    ⚫️ setup_toolchain    ⚫️ teardown_toolchain
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/toolchain/info.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
