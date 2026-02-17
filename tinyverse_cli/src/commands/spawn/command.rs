use anyhow::{Context, Result};
use log::info;
use tinyverse_lib::tmux::{SpawnSessionOptions, TmuxClient};
use tinyverse_lib::{CreateSessionInput, SessionStore};

use super::args::SpawnArgs;
use super::session_name::resolve_session_name;
use crate::providers::{find_by_key, LaunchContext};

pub fn execute(args: SpawnArgs) -> Result<()> {
    let mut store = SessionStore::open_default()?;
    let session_name = resolve_session_name(args.key.as_deref(), &mut store)?;
    let prompt = resolve_prompt(args.prompt.as_deref())?;
    let provider = find_by_key(args.agent.as_str())
        .with_context(|| format!("unknown provider `{}`", args.agent))?;
    let agent_command = provider.build_launch_command(LaunchContext {
        prompt: prompt.as_deref(),
        model: args.model.as_deref(),
        args: args.agent_args.as_deref(),
    });

    let mut options = SpawnSessionOptions::new(session_name.clone());
    options.working_dir = std::env::current_dir().ok();
    options.agent_command = Some(agent_command);

    let client = TmuxClient::new();
    let result = client
        .spawn_session(options)
        .with_context(|| format!("failed to spawn session `{session_name}`"))?;

    info!(
        "Started session {} with {}",
        result.session,
        provider.metadata().name
    );
    info!(
        "Panes: console={}, agent={}",
        result.console_pane_id, result.agent_pane_id
    );

    let stored = store.create_session(&CreateSessionInput {
        session_name: session_name.clone(),
        agent_type: args.agent.clone(),
        description: None,
        tmux_session_name: session_name,
        tmux_session_id: None,
        console_pane_id: Some(result.console_pane_id.clone()),
        agent_pane_id: Some(result.agent_pane_id.clone()),
    })?;
    info!(
        "Stored session metadata (key={}, status={})",
        stored.session_key, stored.status_string
    );

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::resolve_prompt;

    #[test]
    fn treats_empty_prompt_as_none() {
        let resolved = resolve_prompt(Some("   ")).expect("prompt resolution should succeed");
        assert!(resolved.is_none());
    }
}
