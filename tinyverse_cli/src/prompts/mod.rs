use anyhow::{Context, Result};

const OPENCODE_DEFAULT_CONTEXT_TEMPLATE: &str =
    include_str!("../../prompts/opencode_default_context.md");
const USER_PROMPT_BLOCK_PLACEHOLDER: &str = "{{USER_PROMPT_BLOCK}}";

pub fn resolve_user_prompt(prompt_arg: Option<&str>) -> Result<Option<String>> {
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

pub fn resolve_launch_prompt(provider_key: &str, user_prompt: Option<&str>) -> Option<String> {
    if provider_key == "opencode" {
        return Some(render_opencode_prompt(user_prompt));
    }

    user_prompt
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn render_opencode_prompt(user_prompt: Option<&str>) -> String {
    let user_block = match user_prompt.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => format!("## Task Request\n\n{value}\n"),
        None => String::new(),
    };

    let rendered = OPENCODE_DEFAULT_CONTEXT_TEMPLATE
        .replace(USER_PROMPT_BLOCK_PLACEHOLDER, user_block.trim_end())
        .trim()
        .to_owned();

    if rendered.is_empty() {
        user_block.trim().to_owned()
    } else {
        rendered
    }
}

#[cfg(test)]
mod tests {
    use super::{USER_PROMPT_BLOCK_PLACEHOLDER, resolve_launch_prompt, resolve_user_prompt};

    #[test]
    fn opencode_prompt_includes_default_context_without_user_prompt() {
        let prompt =
            resolve_launch_prompt("opencode", None).expect("opencode should include prompt");

        assert!(prompt.contains("# TinyVerse Agent Context"));
        assert!(!prompt.contains(USER_PROMPT_BLOCK_PLACEHOLDER));
        assert!(!prompt.contains("## Task Request"));
    }

    #[test]
    fn opencode_prompt_appends_user_prompt_block() {
        let prompt = resolve_launch_prompt("opencode", Some("Fix flaky tests"))
            .expect("opencode should include prompt");

        assert!(prompt.contains("## Task Request"));
        assert!(prompt.contains("Fix flaky tests"));
    }

    #[test]
    fn non_opencode_uses_user_prompt_only() {
        let prompt = resolve_launch_prompt("custom", Some("Run checks"));
        assert_eq!(prompt.as_deref(), Some("Run checks"));
    }

    #[test]
    fn empty_user_prompt_resolves_to_none() {
        let prompt = resolve_user_prompt(Some("   ")).expect("prompt resolve should succeed");
        assert!(prompt.is_none());
    }
}
