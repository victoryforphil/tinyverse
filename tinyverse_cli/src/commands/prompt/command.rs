use anyhow::Result;

use super::args::{PromptCommands, RenderPromptArgs};
use crate::prompts::{resolve_launch_prompt, resolve_user_prompt};

pub fn execute(command: PromptCommands) -> Result<()> {
    match command {
        PromptCommands::Render(args) => execute_render(args),
    }
}

fn execute_render(args: RenderPromptArgs) -> Result<()> {
    let user_prompt = resolve_user_prompt(args.prompt.as_deref())?;
    let rendered =
        resolve_launch_prompt(args.agent.as_str(), user_prompt.as_deref()).unwrap_or_default();
    println!("{rendered}");
    Ok(())
}
