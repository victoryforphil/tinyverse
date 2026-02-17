use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{SpawnSessionOptions, TmuxClient};

use super::args::{Agent, SpawnArgs};

pub fn execute(args: SpawnArgs) -> Result<()> {
    let session_name = default_session_name();
    let prompt = resolve_prompt(args.prompt.as_deref())?;
    let agent_command = build_agent_command(
        args.agent.clone(),
        prompt.as_deref(),
        args.agent_args.as_deref(),
    );

    let mut options = SpawnSessionOptions::new(session_name.clone());
    options.working_dir = std::env::current_dir().ok();
    options.agent_command = Some(agent_command);

    let client = TmuxClient::new();
    let result = client
        .spawn_session(options)
        .with_context(|| format!("failed to spawn session `{session_name}`"))?;

    info!(
        "CLI // Sessions // Spawned session (meta={{\"session\":\"{}\",\"agent\":\"{:?}\",\"prompt\":{},\"agent_args\":{}}})",
        result.session,
        args.agent,
        prompt.is_some(),
        args.agent_args.is_some()
    );

    println!("session: {}", result.session);
    println!("console_pane: {}", result.console_pane_id);
    println!("agent_pane: {}", result.agent_pane_id);

    Ok(())
}

fn default_session_name() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    format!("tinyverse_{millis}")
}

fn resolve_prompt(prompt_arg: Option<&str>) -> Result<Option<String>> {
    let Some(prompt_arg) = prompt_arg else {
        return Ok(None);
    };

    let trimmed = prompt_arg.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let path = std::path::Path::new(trimmed);
    if path.is_file() {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read prompt file `{trimmed}`"))?;
        return Ok(Some(contents.trim().to_owned()));
    }

    Ok(Some(trimmed.to_owned()))
}

fn build_agent_command(agent: Agent, prompt: Option<&str>, agent_args: Option<&str>) -> String {
    let binary = match agent {
        Agent::Opencode => "opencode",
    };

    let args_text = match (agent_args, prompt) {
        (Some(raw_args), Some(prompt_value)) if raw_args.contains("{prompt}") => {
            raw_args.replace("{prompt}", &shell_escape(prompt_value))
        }
        (Some(raw_args), Some(prompt_value)) => {
            format!("{raw_args} {}", shell_escape(prompt_value))
        }
        (Some(raw_args), None) => raw_args.to_owned(),
        (None, Some(prompt_value)) => shell_escape(prompt_value),
        (None, None) => String::new(),
    };

    if args_text.trim().is_empty() {
        return binary.to_owned();
    }

    format!("{binary} {args_text}")
}

fn shell_escape(value: &str) -> String {
    if value.is_empty() {
        return "''".to_owned();
    }

    let escaped = value.replace('"', "\\\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::{build_agent_command, shell_escape};
    use crate::commands::spawn::args::Agent;

    #[test]
    fn escapes_prompt_for_shell_command() {
        assert_eq!(shell_escape("hello world"), "\"hello world\"");
    }

    #[test]
    fn injects_prompt_placeholder_when_present() {
        let command = build_agent_command(
            Agent::Opencode,
            Some("run tests"),
            Some("--prompt {prompt} --model fast"),
        );

        assert_eq!(command, "opencode --prompt \"run tests\" --model fast");
    }
}
