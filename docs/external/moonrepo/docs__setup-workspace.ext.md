----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/setup-workspace
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, setup workspace
- Summary: BunDenoGoNode.jsPHPPythonRubyRust
----

Source: https://moonrepo.dev/docs/setup-workspace

BunDenoGoNode.jsPHPPythonRubyRust
- [Home](/)
- Getting started
- [Setup workspace](/docs/setup-workspace)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Setup workspace

2 min

Once moon has been [installed](/docs/install), we must setup the [workspace](/docs/concepts/workspace),
which is denoted by the `.moon` folder — this is known as the workspace root. The workspace is in
charge of:

- Integrating with a version control system.

- Defining configuration that applies to its entire tree.

- Housing [projects](/docs/concepts/project) to build a [project graph](/docs/how-it-works/project-graph).

- Running tasks with the [action graph](/docs/how-it-works/action-graph).

## Initializing the repository[​](#initializing-the-repository)

Let's scaffold and initialize moon in a repository with the [`moon init`](/docs/commands/init) command.
This should typically be ran at the root, but can be nested within a directory.

```
$ moon init
```

When executed, the following operations will be applied.

- Creates a `.moon` folder with a [`.moon/workspace.yml`](/docs/config/workspace) configuration file.

- Appends necessary ignore patterns to the relative `.gitignore`.

- Infers the version control system from the environment.

info

If you're investigating moon, or merely want to prototype, you can use `moon init --minimal` to
quickly initialize and create minimal configuration files.

## Migrate from an existing build system[​](#migrate-from-an-existing-build-system)

Looking to migrate from Nx or Turborepo to moon? Use our
[`moon ext migrate-nx`](/docs/guides/extensions#migrate-nx) or
[`moon ext migrate-turborepo`](/docs/guides/extensions#migrate-turborepo) commands for a (somewhat)
seamless migration!

These extensions will convert your existing configuration files to moon's format as best as
possible, but is not a requirement.

## Configuring a version control system[​](#configuring-a-version-control-system)

moon requires a version control system (VCS) to be present for functionality like file diffing,
hashing, and revision comparison. The VCS and its default branch can be configured through the
[`vcs`](/docs/config/workspace#vcs) setting.

.moon/workspace.yml

```
vcs:  manager: 'git'  defaultBranch: 'master'
```

moon defaults to `git` and the settings above, so feel free to skip this.

## Next steps[​](#next-steps)

[Create a project](/docs/create-project)[Configure `.moon/workspace.yml` further](/docs/config/workspace)[Learn about the workspace](/docs/concepts/workspace)

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/setup-workspace.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
