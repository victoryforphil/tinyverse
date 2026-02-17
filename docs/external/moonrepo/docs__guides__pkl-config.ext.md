----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/pkl-config
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, pkl config
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/pkl-config

- [Home](/)
- [Pkl configuration](/docs/guides/pkl-config)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Pkl configuration

v1.32.0

While YAML is our official configuration format, we want to support dynamic formats, and as such,
have added support for Pkl. What is Pkl? If you haven't heard of Pkl yet,
[Pkl is a programmable configuration format by Apple](https://pkl-lang.org/). We like Pkl, as it
meets the following requirements:

- Is easy to read and write.

- Is dynamic and programmable (loops, variables, etc).

- Has type-safety / built-in schema support.

- Has Rust serde integration.

The primary requirement that we are hoping to achieve is supporting a configuration format that is
programmable. We want something that has native support for variables, loops, conditions, and
more, so that you could curate and compose your configuration very easily. Hacking this
functionality into YAML is a terrible user experience in our opinion!

## Installing Pkl[​](#installing-pkl)

Pkl utilizes a client-server architecture, which means that the `pkl` binary must exist in the
environment for parsing and evaluating `.pkl` files. Jump over to the
[official documentation for instructions on how to install Pkl](https://pkl-lang.org/main/current/pkl-cli/index.html#installation).

If you are using [proto](/proto), you can install Pkl with the following commands.

```
proto plugin add pkl https://raw.githubusercontent.com/milesj/proto-plugins/refs/heads/master/pkl.tomlproto install pkl --pin
```

## Using Pkl[​](#using-pkl)

To start using Pkl in moon, simply:

- Install [Pkl](#installing-pkl) and the [VS Code extension](https://pkl-lang.org/vscode/current/index.html)

- Create configs with the `.pkl` extension instead of `.yml`

info

We highly suggest reading the Pkl
[language reference](https://pkl-lang.org/main/current/language-reference/index.html), the
[standard library](https://pkl-lang.org/main/current/standard-library.html), or looking at our
[example configurations](#example-configs) when using Pkl.

### Caveats and restrictions[​](#caveats-and-restrictions)

Since this is an entirely new configuration format that is quite dynamic compared to YAML, there are
some key differences to be aware of!

- Only files are supported. Cannot use or extend from URLs.

- Each `.pkl` file is evaluated in isolation (loops are processed, variables assigned, etc). This means that task inheritance and file merging cannot extend or infer this native functionality.

- `default` is a [special feature](https://pkl-lang.org/main/current/language-reference/index.html#default-element) in Pkl and cannot be used as a setting name. This only applies to [`template.pkl`](/docs/config/template#default), but can be worked around by using `defaultValue` instead.

template.pkl

```
variables {  ["age"] {    type = "number"    prompt = "Age?"    defaultValue = 0}
```

- `local` is also a reserved word in Pkl. It can be worked around by escaping it with backticks, or you can simply use the [`preset` setting](/docs/config/project#preset) instead.

```
tasks {  ["example"] {    `local` = true    # Or    preset = "server"  }}
```

## Example configs[​](#example-configs)

### `.moon/workspace.pkl`[​](#moonworkspacepkl)

```
projects {  globs = List("apps/*", "packages/*")  sources {    ["root"] = "."  }}vcs {  defaultBranch = "master"}
```

### `.moon/toolchain.pkl`[​](#moontoolchainpkl)

```
node {  version = "20.15.0"  packageManager = "yarn"  yarn {    version = "4.3.1"  }  addEnginesConstraint = false  inferTasksFromScripts = false}
```

### `moon.pkl`[​](#moonpkl)

```
type = "application"language = "typescript"dependsOn = List("client", "ui")tasks {  ["build"] {    command = "docusaurus build"    deps = List("^:build")    outputs = List("build")    options {      interactive = true      retryCount = 3    }  }  ["typecheck"] {    command = "tsc --build"    inputs = new Listing {      "@globs(sources)"      "@globs(tests)"      "tsconfig.json"      "/tsconfig.options.json"    }  }}
```

## Example functionality[​](#example-functionality)

### Loops and conditionals[​](#loops-and-conditionals)

```
tasks {  for (_os in List("linux", "macos", "windows")) {    ["build-\(_os)"] {      command = "cargo"      args = List(        "--target",        if (_os == "linux") "x86_64-unknown-linux-gnu"          else if (_os == "macos") "x86_64-apple-darwin"          else "i686-pc-windows-msvc",        "--verbose"      )      options {        os = _os      }    }  }}
```

### Local variables[​](#local-variables)

```
local _sharedInputs = List("src/**/*")tasks {  ["test"] {    // ...    inputs = List("tests/**/*") + _sharedInputs  }  ["lint"] {    // ...    inputs = List("**/*.graphql") + _sharedInputs  }}
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/pkl-config.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
