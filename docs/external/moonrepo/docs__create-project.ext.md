----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/create-project
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, create project
- Summary: BunDenoGoNode.jsPHPPythonRubyRust
----

Source: https://moonrepo.dev/docs/create-project

BunDenoGoNode.jsPHPPythonRubyRust
- [Home](/)
- Getting started
- [Create a project](/docs/create-project)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Create a project

3 min

With a [workspace](/docs/setup-workspace), we can now house one or many [projects](/docs/concepts/project),
with a project being an application, library, or tool. In the end, each project will have its own
build layer, personal tasks, and custom configuration.

## Declaring a project in the workspace[​](#declaring-a-project-in-the-workspace)

Although a project may exist in your repository, it's not accessible from moon until it's been
mapped in the [`projects`](/docs/config/workspace#projects) setting found in
[`.moon/workspace.yml`](/docs/config/workspace). When mapping a project, we require a unique name for
the project, and a project source location (path relative from the workspace root).

Let's say we have a frontend web application called "client", and a backend application called
"server", our `projects` setting would look like the following.

.moon/workspace.yml

```
projects:  client: 'apps/client'  server: 'apps/server'
```

We can now run [`moon project client`](/docs/commands/project) and
[`moon project server`](/docs/commands/project) to display information about each project. If these
projects were not mapped, or were pointing to an invalid source, the command would throw an error.

success

The [`projects`](/docs/config/workspace#projects) setting also supports a list of globs, if you'd prefer
to not manually curate the projects list!

## Configuring a project[​](#configuring-a-project)

A project can be configured in 1 of 2 ways:

- Through the [`.moon/tasks/all.yml`](/docs/config/tasks) config file, which defines file groups and tasks that are inherited by all projects within the workspace. Perfect for standardizing common tasks like linting, typechecking, and code formatting.

- Through the [`moon.yml`](/docs/config/project) config file, found at the root of each project, which defines files groups, tasks, dependencies, and more that are unique to that project.

Both config files are optional, and can be used separately or together, the choice is yours!

Now let's continue with our client and server example above. If we wanted to configure both
projects, and define config that's also shared between the 2, we could do something like the
following:

- Client
- Server
- Both (inherited)

apps/client/moon.yml

```
tasks:  build:    command: 'vite dev'    inputs:      - 'src/**/*'    outputs:      - 'dist'
```

apps/server/moon.yml

```
tasks:  build:    command: 'babel src --out-dir build'    inputs:      - 'src/**/*'    outputs:      - 'build'
```

.moon/tasks/all.yml

```
tasks:  format:    command: 'prettier --check .'  lint:    command: 'eslint --no-error-on-unmatched-pattern .'  test:    command: 'jest --passWithNoTests .'  typecheck:    command: 'tsc --build'
```

### Adding optional metadata[​](#adding-optional-metadata)

When utilizing moon in a large monorepo or organization, ownership becomes very important, but also
difficult to maintain. To combat this problem, moon supports the
[`project`](/docs/config/project#project) field within a project's [`moon.yml`](/docs/config/project)
config.

This field is optional by default, but when defined it provides metadata about the project,
specifically around team ownership, which developers maintain the project, where to discuss it, and
more!

Furthermore, we also support the [`layer`](/docs/config/project#layer) and
[`language`](/docs/config/project#language) settings for a more granular breakdown of what exists in the
repository.

/moon.yml

```
layer: 'tool'language: 'typescript'project:  name: 'moon'  description: 'A repo management tool.'  channel: '#moon'  owner: 'infra.platform'  maintainers: ['miles.johnson']
```

## Next steps[​](#next-steps)

[Setup toolchain](/docs/setup-toolchain)[Configure `.moon/workspace.yml` further](/docs/config/workspace)[Configure `.moon/tasks/all.yml` further](/docs/config/tasks)[Configure `moon.yml` further](/docs/config/project)[Learn about projects](/docs/concepts/project)

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/create-project.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
