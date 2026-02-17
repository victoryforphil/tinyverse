----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/hash
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, hash
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/hash

- [Home](/)
- [Commands](/docs/commands)
- [hash](/docs/commands/hash)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# hash

v2.0.0

Use the `moon hash` command to inspect the contents and sources of a generated hash, also known as
the hash manifest. This is extremely useful in debugging task inputs.

```
$ moon hash 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde# Query hash using short form$ moon hash 0b55b234
```

By default, this will output the contents of the hash manifest (which is JSON), and the fully
qualified resolved hash.

```
Hash: 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde{  "command": "build",  "args": ["./build"]  // ...}
```

The command can also be output raw JSON by passing the `--json` flag.

### Comparing hashes[​](#comparing-hashes)

The command can also be used to compare two hashes by diffing their contents. Simply pass two hashes
as arguments.

```
# Diff between 2 hashes$ moon hash 0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171# Diff between 2 hashes using short form$ moon hash 0b55b234 2388552f
```

By default, this will output the contents of a hash file (which is JSON), highlighting the
differences between the left and right hashes. Lines that match will be printed in white, while the
left differences printed in green, and right differences printed in red. If you use `git diff`, this
will feel familiar to you.

```
Left:  0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fddeRight: 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171{	"command": "build",	"args": [+		"./dist"-		"./build"	],	...}
```

The differences can also be output in JSON by passing the `--json` flag. The output has the
following structure:

```
{	left: string,	left_hash: string,	left_diffs: string[],	right: string,	right_hash: string,	right_diffs: string[],}
```

### Options[​](#options)

- `--json` - Display the diff in JSON format.

### Configuration[​](#configuration)

- [`hasher`](/docs/config/workspace#hasher) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/hash.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
