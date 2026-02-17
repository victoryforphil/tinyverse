----
## External Docs Snapshot // opencode

- Captured: 2026-02-17T20:19:48.515Z
- Source root: https://opencode.ai/docs
- Source page: /docs/gitlab
- Keywords: opencode, docs, ai coding assistant, cli, gitlab
- Summary: [Skip to content](http://opencode.ai/docs/gitlab#_top)
----

Source: https://opencode.ai/docs/gitlab

GitLab | OpenCode
===============
[Skip to content](http://opencode.ai/docs/gitlab#_top)

[![Image 1](http://opencode.ai/docs/_astro/logo-dark.DOStV66V.svg)![Image 2](http://opencode.ai/docs/_astro/logo-light.B0yzR0O5.svg) OpenCode](http://opencode.ai/docs/)

[app.header.home](http://opencode.ai/)[app.header.docs](http://opencode.ai/docs/)

[](https://github.com/anomalyco/opencode)[](https://opencode.ai/discord)

Search Ctrl K

 Cancel 

*   [Intro](http://opencode.ai/docs/)
*   [Config](http://opencode.ai/docs/config/)
*   [Providers](http://opencode.ai/docs/providers/)
*   [Network](http://opencode.ai/docs/network/)
*   [Enterprise](http://opencode.ai/docs/enterprise/)
*   [Troubleshooting](http://opencode.ai/docs/troubleshooting/)
*   [Windows (WSL)](http://opencode.ai/docs/windows-wsl/)
*   
Usage 
    *   [TUI](http://opencode.ai/docs/tui/)
    *   [CLI](http://opencode.ai/docs/cli/)
    *   [Web](http://opencode.ai/docs/web/)
    *   [IDE](http://opencode.ai/docs/ide/)
    *   [Zen](http://opencode.ai/docs/zen/)
    *   [Share](http://opencode.ai/docs/share/)
    *   [GitHub](http://opencode.ai/docs/github/)
    *   [GitLab](http://opencode.ai/docs/gitlab/)

*   
Configure 
    *   [Tools](http://opencode.ai/docs/tools/)
    *   [Rules](http://opencode.ai/docs/rules/)
    *   [Agents](http://opencode.ai/docs/agents/)
    *   [Models](http://opencode.ai/docs/models/)
    *   [Themes](http://opencode.ai/docs/themes/)
    *   [Keybinds](http://opencode.ai/docs/keybinds/)
    *   [Commands](http://opencode.ai/docs/commands/)
    *   [Formatters](http://opencode.ai/docs/formatters/)
    *   [Permissions](http://opencode.ai/docs/permissions/)
    *   [LSP Servers](http://opencode.ai/docs/lsp/)
    *   [MCP servers](http://opencode.ai/docs/mcp-servers/)
    *   [ACP Support](http://opencode.ai/docs/acp/)
    *   [Agent Skills](http://opencode.ai/docs/skills/)
    *   [Custom Tools](http://opencode.ai/docs/custom-tools/)

*   
Develop 
    *   [SDK](http://opencode.ai/docs/sdk/)
    *   [Server](http://opencode.ai/docs/server/)
    *   [Plugins](http://opencode.ai/docs/plugins/)
    *   [Ecosystem](http://opencode.ai/docs/ecosystem/)

[GitHub](https://github.com/anomalyco/opencode)[Discord](https://opencode.ai/discord)

Select theme Select language 

On this page

*   [Overview](http://opencode.ai/docs/gitlab#_top)
*   [GitLab CI](http://opencode.ai/docs/gitlab#gitlab-ci)
    *   [Features](http://opencode.ai/docs/gitlab#features)
    *   [Setup](http://opencode.ai/docs/gitlab#setup)

*   [GitLab Duo](http://opencode.ai/docs/gitlab#gitlab-duo)
    *   [Features](http://opencode.ai/docs/gitlab#features-1)
    *   [Setup](http://opencode.ai/docs/gitlab#setup-1)
    *   [Examples](http://opencode.ai/docs/gitlab#examples)

On this page
------------

*   [Overview](http://opencode.ai/docs/gitlab#_top)
*   [GitLab CI](http://opencode.ai/docs/gitlab#gitlab-ci)
    *   [Features](http://opencode.ai/docs/gitlab#features)
    *   [Setup](http://opencode.ai/docs/gitlab#setup)

*   [GitLab Duo](http://opencode.ai/docs/gitlab#gitlab-duo)
    *   [Features](http://opencode.ai/docs/gitlab#features-1)
    *   [Setup](http://opencode.ai/docs/gitlab#setup-1)
    *   [Examples](http://opencode.ai/docs/gitlab#examples)

GitLab
======

Use OpenCode in GitLab issues and merge requests.

OpenCode integrates with your GitLab workflow through your GitLab CI/CD pipeline or with GitLab Duo.

In both cases, OpenCode will run on your GitLab runners.

* * *

[GitLab CI](http://opencode.ai/docs/gitlab#gitlab-ci)
-----------------------------------------------------

OpenCode works in a regular GitLab pipeline. You can build it into a pipeline as a [CI component](https://docs.gitlab.com/ee/ci/components/)

Here we are using a community-created CI/CD component for OpenCode — [nagyv/gitlab-opencode](https://gitlab.com/nagyv/gitlab-opencode).

* * *

### [Features](http://opencode.ai/docs/gitlab#features)

*   **Use custom configuration per job**: Configure OpenCode with a custom configuration directory, for example `./config/#custom-directory` to enable or disable functionality per OpenCode invocation.
*   **Minimal setup**: The CI component sets up OpenCode in the background, you only need to create the OpenCode configuration and the initial prompt.
*   **Flexible**: The CI component supports several inputs for customizing its behavior

* * *

### [Setup](http://opencode.ai/docs/gitlab#setup)

1.   Store your OpenCode authentication JSON as a File type CI environment variables under **Settings**>**CI/CD**>**Variables**. Make sure to mark them as “Masked and hidden”.

2.   Add the following to your `.gitlab-ci.yml` file.

.gitlab-ci.yml

`include:  - component: $CI_SERVER_FQDN/nagyv/gitlab-opencode/opencode@2    inputs:      config_dir: ${CI_PROJECT_DIR}/opencode-config      auth_json: $OPENCODE_AUTH_JSON # The variable name for your OpenCode authentication JSON      command: optional-custom-command      message: "Your prompt here"`    

For more inputs and use cases [check out the docs](https://gitlab.com/explore/catalog/nagyv/gitlab-opencode) for this component.

* * *

[GitLab Duo](http://opencode.ai/docs/gitlab#gitlab-duo)
-------------------------------------------------------

OpenCode integrates with your GitLab workflow. Mention `@opencode` in a comment, and OpenCode will execute tasks within your GitLab CI pipeline.

* * *

### [Features](http://opencode.ai/docs/gitlab#features-1)

*   **Triage issues**: Ask OpenCode to look into an issue and explain it to you.
*   **Fix and implement**: Ask OpenCode to fix an issue or implement a feature. It will create a new branch and raise a merge request with the changes.
*   **Secure**: OpenCode runs on your GitLab runners.

* * *

### [Setup](http://opencode.ai/docs/gitlab#setup-1)

OpenCode runs in your GitLab CI/CD pipeline, here’s what you’ll need to set it up:

Tip

Check out the [**GitLab docs**](https://docs.gitlab.com/user/duo_agent_platform/agent_assistant/) for up to date instructions.

1.   Configure your GitLab environment

2.   Set up CI/CD

3.   Get an AI model provider API key

4.   Create a service account

5.   Configure CI/CD variables

6.   Create a flow config file, here’s an example:

Flow configuration```
image: node:22-slimcommands:  - echo "Installing opencode"  - npm install --global opencode-ai  - echo "Installing glab"  - export GITLAB_TOKEN=$GITLAB_TOKEN_OPENCODE  - apt-get update --quiet && apt-get install --yes curl wget gpg git && rm --recursive --force /var/lib/apt/lists/*  - curl --silent --show-error --location "https://raw.githubusercontent.com/upciti/wakemeops/main/assets/install_repository" | bash  - apt-get install --yes glab  - echo "Configuring glab"  - echo $GITLAB_HOST  - echo "Creating OpenCode auth configuration"  - mkdir --parents ~/.local/share/opencode  - |    cat > ~/.local/share/opencode/auth.json << EOF    {      "anthropic": {        "type": "api",        "key": "$ANTHROPIC_API_KEY"      }    }    EOF  - echo "Configuring git"  - git config --global user.email "opencode@gitlab.com"  - git config --global user.name "OpenCode"  - echo "Testing glab"  - glab issue list  - echo "Running OpenCode"  - |    opencode run "    You are an AI assistant helping with GitLab operations.
    Context: $AI_FLOW_CONTEXT    Task: $AI_FLOW_INPUT    Event: $AI_FLOW_EVENT
    Please execute the requested task using the available GitLab tools.    Be thorough in your analysis and provide clear explanations.
    <important>    Please use the glab CLI to access data from GitLab. The glab CLI has already been authenticated. You can run the corresponding commands.
    If you are asked to summarize an MR or issue or asked to provide more information then please post back a note to the MR/Issue so that the user can see it.    You don't need to commit or push up changes, those will be done automatically based on the file changes you make.    </important>    "  - git checkout --branch $CI_WORKLOAD_REF origin/$CI_WORKLOAD_REF  - echo "Checking for git changes and pushing if any exist"  - |    if ! git diff --quiet || ! git diff --cached --quiet || [ --not --zero "$(git ls-files --others --exclude-standard)" ]; then      echo "Git changes detected, adding and pushing..."      git add .      if git diff --cached --quiet; then        echo "No staged changes to commit"      else        echo "Committing changes to branch: $CI_WORKLOAD_REF"        git commit --message "Codex changes"        echo "Pushing changes up to $CI_WORKLOAD_REF"        git push https://gitlab-ci-token:$GITLAB_TOKEN@$GITLAB_HOST/gl-demo-ultimate-dev-ai-epic-17570/test-java-project.git $CI_WORKLOAD_REF        echo "Changes successfully pushed"      fi    else      echo "No git changes detected, skipping push"    fivariables:  - ANTHROPIC_API_KEY  - GITLAB_TOKEN_OPENCODE  - GITLAB_HOST
```
     

You can refer to the [GitLab CLI agents docs](https://docs.gitlab.com/user/duo_agent_platform/agent_assistant/) for detailed instructions.

* * *

### [Examples](http://opencode.ai/docs/gitlab#examples)

Here are some examples of how you can use OpenCode in GitLab.

Tip

You can configure to use a different trigger phrase than `@opencode`.

*   **Explain an issue**

Add this comment in a GitLab issue.

`@opencode explain this issue`    
OpenCode will read the issue and reply with a clear explanation.

*   **Fix an issue**

In a GitLab issue, say:

`@opencode fix this`    
OpenCode will create a new branch, implement the changes, and open a merge request with the changes.

*   **Review merge requests**

Leave the following comment on a GitLab merge request.

`@opencode review this merge request`    
OpenCode will review the merge request and provide feedback.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/gitlab.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord)Select language 

© [Anomaly](https://anoma.ly/)

Last updated: Feb 17, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery with direct HTML fallback support.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
