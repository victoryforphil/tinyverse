use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{SpawnSessionOptions, TmuxClient};
use tinyverse_lib::{CreateSessionInput, SessionStore, resolve_session_name};
use tinyverse_ui::{
    ActionLine, DetailSection, GuidanceLine, LabeledField, Panel, Tone, default_stdout_context,
};

use super::args::SpawnArgs;
use crate::commands::config::store;
use crate::commands::output::display_session_name;
use crate::prompts::{resolve_launch_prompt, resolve_user_prompt};
use crate::providers::{LaunchContext, find_by_key};

pub fn execute(args: SpawnArgs) -> Result<()> {
    let config = store::load()?;
    let mut store = SessionStore::open_default()?;
    let session_name = resolve_session_name(args.key.as_deref(), &mut store)?;
    let user_prompt = resolve_user_prompt(args.prompt.as_deref())?;
    let agent_key = resolve_agent_key(&args, &config);
    let provider = find_by_key(agent_key.as_str())
        .with_context(|| format!("unknown provider `{}`", agent_key))?;
    let model = args
        .model
        .as_deref()
        .or(config.spawn.default_model.as_deref());
    let launch_prompt = resolve_launch_prompt(agent_key.as_str(), user_prompt.as_deref());
    let agent_command = provider.build_launch_command(LaunchContext {
        prompt: launch_prompt.as_deref(),
        model,
        args: args.agent_args.as_deref(),
    });

    let clean_shell = resolve_clean_shell(&args, &config);
    let working_dir = resolve_working_dir(&config)?;

    let mut options = SpawnSessionOptions::new(session_name.clone());
    options.working_dir = Some(working_dir.clone());
    if clean_shell {
        options.pane_shell_command = Some("zsh -f".to_owned());
    }
    options.agent_command = Some(agent_command);

    let client = TmuxClient::new();
    let result = client
        .spawn_session(options)
        .with_context(|| format!("failed to spawn session `{session_name}`"))?;

    let stored = store.create_session(&CreateSessionInput {
        session_name: session_name.clone(),
        agent_type: agent_key.clone(),
        description: None,
        tmux_session_name: session_name.clone(),
        tmux_session_id: None,
        console_pane_id: Some(result.console_pane_id.clone()),
        agent_pane_id: Some(result.agent_pane_id.clone()),
    })?;

    info!(
        "Spawned session {} (key={}, agent={})",
        session_name,
        stored.session_key,
        provider.metadata().name
    );

    let display_name = display_session_name(&session_name);
    print_spawn_summary(
        &display_name,
        &stored.session_key,
        provider.metadata().name,
        &result.console_pane_id,
        &result.agent_pane_id,
        clean_shell,
        &working_dir.display().to_string(),
    );

    Ok(())
}

fn print_spawn_summary(
    display_name: &str,
    session_key: &str,
    agent_name: &str,
    console_pane: &str,
    agent_pane: &str,
    clean_shell: bool,
    working_dir: &str,
) {
    let context = default_stdout_context();

    let details = DetailSection::new("Session Details")
        .with_field(LabeledField::new("Name", display_name))
        .with_field(LabeledField::new("Key", session_key))
        .with_field(LabeledField::new("Agent", agent_name))
        .with_field(LabeledField::new("Working dir", working_dir))
        .with_field(LabeledField::new("Agent pane", agent_pane))
        .with_field(LabeledField::new("Console pane", console_pane))
        .with_field(LabeledField::new(
            "Shell",
            if clean_shell {
                "zsh -f (clean)"
            } else {
                "default"
            },
        ))
        .render(&context);

    let guidance = GuidanceLine::new(format!("Run `tinyverse attach {session_key}` to connect."))
        .render(&context);

    let body = Panel::new(format!("{details}\n\n{guidance}"))
        .with_title("TinyVerse: Spawn")
        .with_tone(Tone::Success)
        .render(&context);

    let header =
        ActionLine::new("OK", format!("Spawned {display_name}"), Tone::Success).render(&context);

    println!("{header}\n\n{body}");
}

fn resolve_clean_shell(args: &SpawnArgs, config: &store::TinyverseConfig) -> bool {
    if args.clean_shell {
        return true;
    }
    if args.no_clean_shell {
        return false;
    }

    config.shell.clean
}

fn resolve_agent_key(args: &SpawnArgs, config: &store::TinyverseConfig) -> String {
    args.agent
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| config.spawn.default_agent.clone())
}

fn resolve_working_dir(config: &store::TinyverseConfig) -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().context("failed to read current working directory")?;
    let Some(default_dir) = config.workspace.default_dir.as_deref() else {
        return Ok(cwd);
    };

    let candidate = expand_config_path(default_dir, &cwd);
    if !candidate.is_dir() {
        anyhow::bail!(
            "configured workspace.default_dir is not a directory: `{}`",
            candidate.display()
        );
    }

    candidate.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize workspace dir `{}`",
            candidate.display()
        )
    })
}

fn expand_config_path(value: &str, cwd: &std::path::Path) -> std::path::PathBuf {
    let expanded = if let Some(stripped) = value.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            std::path::PathBuf::from(home).join(stripped)
        } else {
            std::path::PathBuf::from(value)
        }
    } else {
        std::path::PathBuf::from(value)
    };

    if expanded.is_absolute() {
        expanded
    } else {
        cwd.join(expanded)
    }
}

#[cfg(test)]
mod tests {
    use super::SpawnArgs;
    use super::{resolve_agent_key, resolve_working_dir};
    use crate::commands::config::store::{
        GitConfig, ShellConfig, SpawnConfig, TinyverseConfig, WorkspaceConfig,
    };
    use crate::prompts::resolve_user_prompt;

    #[test]
    fn treats_empty_prompt_as_none() {
        let resolved = resolve_user_prompt(Some("   ")).expect("prompt resolution should succeed");
        assert!(resolved.is_none());
    }

    #[test]
    fn uses_config_default_agent_when_flag_is_missing() {
        let args = SpawnArgs {
            key: None,
            agent: None,
            prompt: None,
            model: None,
            agent_args: None,
            clean_shell: false,
            no_clean_shell: false,
        };
        let config = TinyverseConfig {
            shell: ShellConfig::default(),
            workspace: WorkspaceConfig::default(),
            git: GitConfig::default(),
            spawn: SpawnConfig {
                default_agent: "opencode".to_owned(),
                default_model: None,
            },
        };

        assert_eq!(resolve_agent_key(&args, &config), "opencode");
    }

    #[test]
    fn resolves_workspace_default_dir_when_present() {
        let config = TinyverseConfig {
            shell: ShellConfig::default(),
            workspace: WorkspaceConfig {
                default_dir: Some(".".to_owned()),
            },
            git: GitConfig::default(),
            spawn: SpawnConfig::default(),
        };

        let resolved = resolve_working_dir(&config).expect("workspace resolution should succeed");
        assert!(resolved.is_absolute());
        assert!(resolved.is_dir());
    }
}
