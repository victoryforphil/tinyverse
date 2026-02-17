use anyhow::{Context, Result};
use log::{debug, info, warn};
use tinyverse_lib::tmux::{PanelRole, SpawnSessionOptions, SplitDirection, TmuxClient};
use tinyverse_lib::{CreateSessionInput, SessionStore, resolve_session_name};
use tinyverse_ui::{
    ActionLine, DetailSection, GuidanceLine, LabeledField, Panel, Tone, default_stdout_context,
};

use super::args::SpawnArgs;
use crate::commands::config::store;
use crate::commands::config::store::{TmuxLayoutDirection, TmuxLayoutPrimary};
use crate::commands::output::display_session_name;
use crate::opencode_service::lookup_managed_opencode_service;
use crate::prompts::{resolve_launch_prompt, resolve_user_prompt};
use crate::providers::{LaunchContext, find_by_key};

pub fn execute(args: SpawnArgs) -> Result<()> {
    let config = store::load()?;
    validate_tmux_config(&config)?;
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
    let managed_service = lookup_managed_opencode_service(&mut store, config.opencode.server.mode)?;
    let merged_agent_args = merge_agent_args_with_managed_service(
        agent_key.as_str(),
        args.agent_args.as_deref(),
        managed_service.as_ref(),
    );
    let agent_command = provider.build_launch_command(LaunchContext {
        prompt: launch_prompt.as_deref(),
        model,
        args: merged_agent_args.as_deref(),
    });
    let (agent_base_url, agent_session_id) =
        infer_agent_connection(agent_key.as_str(), agent_command.as_str());

    let clean_shell = resolve_clean_shell(&args, &config);
    let working_dir = resolve_working_dir(&config)?;
    debug!(
        "spawning session (name={}, agent={}, clean_shell={}, cwd={})",
        session_name,
        agent_key,
        clean_shell,
        working_dir.display()
    );

    let mut options = SpawnSessionOptions::new(session_name.clone());
    options.working_dir = Some(working_dir.clone());
    if clean_shell {
        options.pane_shell_command = Some("zsh -f".to_owned());
    }
    options.agent_command = Some(agent_command);
    options.initial_window_width = Some(config.tmux.initial_window_width);
    options.initial_window_height = Some(config.tmux.initial_window_height);
    options.split_direction = match config.tmux.layout.direction {
        TmuxLayoutDirection::Horizontal => SplitDirection::Horizontal,
        TmuxLayoutDirection::Vertical => SplitDirection::Vertical,
    };
    options.primary_role = match config.tmux.layout.primary {
        TmuxLayoutPrimary::Agent => PanelRole::Agent,
        TmuxLayoutPrimary::Console => PanelRole::Console,
    };
    options.secondary_size_percent = Some(config.tmux.layout.secondary_percent);

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
        agent_base_url,
        agent_session_id,
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

fn merge_agent_args_with_managed_service(
    agent_key: &str,
    agent_args: Option<&str>,
    managed_service: Option<&crate::opencode_service::ManagedOpencodeService>,
) -> Option<String> {
    let provided = agent_args.unwrap_or_default().trim();
    if !agent_key.eq_ignore_ascii_case("opencode") {
        return if provided.is_empty() {
            None
        } else {
            Some(provided.to_owned())
        };
    }

    let Some(service) = managed_service else {
        if !provided.is_empty() {
            warn!("spawning opencode session without managed server metadata; using provided args");
        }
        return if provided.is_empty() {
            None
        } else {
            Some(provided.to_owned())
        };
    };

    if provided.contains(" attach ")
        || provided.starts_with("attach ")
        || provided.contains("--hostname")
        || provided.contains("-h ")
        || provided.contains("--port")
        || provided.contains("-p ")
    {
        debug!("opencode spawn uses explicit args; managed server defaults skipped");
        return Some(provided.to_owned());
    }

    let connect_host = normalize_connect_host(&service.hostname);
    let managed_args = format!("--hostname {connect_host} --port {}", service.port);
    if provided.is_empty() {
        info!(
            "opencode spawn auto-configured from managed server (hostname={}, port={})",
            connect_host, service.port
        );
        return Some(managed_args);
    }

    info!(
        "opencode spawn appending managed server args (hostname={}, port={})",
        connect_host, service.port
    );
    Some(format!("{provided} {managed_args}"))
}

fn normalize_connect_host(hostname: &str) -> &str {
    match hostname {
        "0.0.0.0" => "127.0.0.1",
        "::" => "::1",
        _ => hostname,
    }
}

fn infer_agent_connection(agent_key: &str, command: &str) -> (Option<String>, Option<String>) {
    if !agent_key.eq_ignore_ascii_case("opencode") {
        return (None, None);
    }

    let tokens = command.split_whitespace().collect::<Vec<_>>();
    let is_opencode_command = tokens
        .iter()
        .any(|token| token.contains("opencode") && !token.starts_with("OPENCODE_"));
    if !is_opencode_command {
        return (None, None);
    }

    if let Some(base_url) = extract_attach_url(&tokens) {
        return (
            Some(base_url),
            extract_arg_value(&tokens, "--session", "-s"),
        );
    }

    let hostname =
        extract_arg_value(&tokens, "--hostname", "-h").unwrap_or_else(|| "127.0.0.1".to_owned());
    let port = extract_arg_value(&tokens, "--port", "-p")
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(4150);

    let base_url = format!("http://{hostname}:{port}");
    (Some(base_url), None)
}

fn extract_attach_url(tokens: &[&str]) -> Option<String> {
    for (index, token) in tokens.iter().enumerate() {
        if *token == "attach"
            && let Some(value) = tokens.get(index + 1)
            && !value.starts_with('-')
            && !value.trim().is_empty()
        {
            return Some(value.trim().trim_end_matches('/').to_owned());
        }
    }

    None
}

fn extract_arg_value(tokens: &[&str], long_flag: &str, short_flag: &str) -> Option<String> {
    for (index, token) in tokens.iter().enumerate() {
        if let Some(value) = token.strip_prefix(&format!("{long_flag}="))
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }
        if let Some(value) = token.strip_prefix(&format!("{short_flag}="))
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }

        if (*token == long_flag || *token == short_flag)
            && let Some(value) = tokens.get(index + 1)
            && !value.starts_with('-')
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }
    }

    None
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

fn validate_tmux_config(config: &store::TinyverseConfig) -> Result<()> {
    if config.tmux.initial_window_width == 0 {
        anyhow::bail!("invalid tmux.initial_window_width: must be greater than 0")
    }
    if config.tmux.initial_window_height == 0 {
        anyhow::bail!("invalid tmux.initial_window_height: must be greater than 0")
    }
    if !(1..=99).contains(&config.tmux.layout.secondary_percent) {
        anyhow::bail!("invalid tmux.layout.secondary_percent: must be between 1 and 99")
    }

    Ok(())
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
    use super::{resolve_agent_key, resolve_working_dir, validate_tmux_config};
    use crate::commands::config::store::{
        GitConfig, OpencodeConfig, ShellConfig, SpawnConfig, TinyverseConfig, TmuxConfig,
        TuiConfig, WorkspaceConfig,
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
            tmux: TmuxConfig::default(),
            tui: TuiConfig::default(),
            opencode: OpencodeConfig::default(),
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
            tmux: TmuxConfig::default(),
            tui: TuiConfig::default(),
            opencode: OpencodeConfig::default(),
        };

        let resolved = resolve_working_dir(&config).expect("workspace resolution should succeed");
        assert!(resolved.is_absolute());
        assert!(resolved.is_dir());
    }

    #[test]
    fn rejects_invalid_tmux_secondary_percent() {
        let mut config = TinyverseConfig::default();
        config.tmux.layout.secondary_percent = 0;

        let result = validate_tmux_config(&config);
        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_tmux_layout_config() {
        let config = TinyverseConfig::default();
        let result = validate_tmux_config(&config);
        assert!(result.is_ok());
    }
}
