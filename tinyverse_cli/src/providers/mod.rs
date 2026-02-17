#[derive(Debug, Clone, Copy)]
pub struct ProviderMetadata {
    pub key: &'static str,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct LaunchContext<'a> {
    pub prompt: Option<&'a str>,
    pub model: Option<&'a str>,
    pub args: Option<&'a str>,
}

pub trait Provider: Sync {
    fn metadata(&self) -> ProviderMetadata;
    fn launch_command_template(&self) -> &'static str;
    fn launch_args_template(&self) -> &'static str;

    fn build_launch_command(&self, context: LaunchContext<'_>) -> String {
        let rendered_user_args = context.args.map(|value| {
            render_template(
                value,
                LaunchContext {
                    args: None,
                    ..context
                },
            )
        });

        let rendered_command = render_template(
            self.launch_command_template(),
            LaunchContext {
                args: rendered_user_args.as_deref(),
                ..context
            },
        );

        let rendered_args = render_template(
            self.launch_args_template(),
            LaunchContext {
                args: rendered_user_args.as_deref(),
                ..context
            },
        );

        if rendered_args.trim().is_empty() {
            return rendered_command.trim().to_owned();
        }

        format!("{} {}", rendered_command.trim(), rendered_args.trim())
    }
}

pub struct OpencodeProvider;

impl Provider for OpencodeProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            key: "opencode",
            name: "OpenCode",
        }
    }

    fn launch_command_template(&self) -> &'static str {
        "opencode"
    }

    fn launch_args_template(&self) -> &'static str {
        "--prompt {prompt} {args}"
    }

    fn build_launch_command(&self, context: LaunchContext<'_>) -> String {
        let rendered_user_args = context.args.map(|value| {
            render_template(
                value,
                LaunchContext {
                    args: None,
                    ..context
                },
            )
        });

        let rendered_prompt = context.prompt.map(shell_escape);
        let prompt_in_user_args = context
            .args
            .map(|value| value.contains("{prompt}"))
            .unwrap_or(false);

        let mut parts = vec![self.launch_command_template().to_owned()];
        if let Some(prompt) = rendered_prompt {
            if !prompt_in_user_args {
                parts.push("--prompt".to_owned());
                parts.push(prompt);
            }
        }

        if let Some(args) = rendered_user_args {
            let trimmed = args.trim();
            if !trimmed.is_empty() {
                parts.push(trimmed.to_owned());
            }
        }

        parts.join(" ")
    }
}

static OPENCODE_PROVIDER: OpencodeProvider = OpencodeProvider;
static PROVIDERS: [&dyn Provider; 1] = [&OPENCODE_PROVIDER];

pub fn all() -> &'static [&'static dyn Provider] {
    &PROVIDERS
}

pub fn find_by_key(key: &str) -> Option<&'static dyn Provider> {
    PROVIDERS
        .iter()
        .copied()
        .find(|provider| provider.metadata().key == key)
}

fn render_template(template: &str, context: LaunchContext<'_>) -> String {
    template
        .replace("{prompt}", &format_prompt(context.prompt))
        .replace("{model}", &format_model(context.model))
        .replace("{args}", context.args.unwrap_or_default())
}

fn format_prompt(prompt: Option<&str>) -> String {
    prompt.map(shell_escape).unwrap_or_default()
}

fn format_model(model: Option<&str>) -> String {
    model.map(shell_escape).unwrap_or_default()
}

fn shell_escape(value: &str) -> String {
    if value.is_empty() {
        return "''".to_owned();
    }

    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\'' => escaped.push_str("\\'"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }

    format!("$'{escaped}'")
}

#[cfg(test)]
mod tests {
    use super::{all, find_by_key, LaunchContext, Provider, ProviderMetadata};

    struct MockProvider;

    impl Provider for MockProvider {
        fn metadata(&self) -> ProviderMetadata {
            ProviderMetadata {
                key: "mock",
                name: "Mock Provider",
            }
        }

        fn launch_command_template(&self) -> &'static str {
            "mock-cli"
        }

        fn launch_args_template(&self) -> &'static str {
            "--model {model} {args}"
        }
    }

    #[test]
    fn defines_opencode_provider() {
        let provider = find_by_key("opencode").expect("opencode provider should exist");
        assert_eq!(provider.metadata().name, "OpenCode");
        assert_eq!(provider.launch_command_template(), "opencode");
    }

    #[test]
    fn builds_launch_command_with_prompt() {
        let provider = find_by_key("opencode").expect("opencode provider should exist");
        let command = provider.build_launch_command(LaunchContext {
            prompt: Some("run tests"),
            model: None,
            args: None,
        });

        assert_eq!(command, "opencode --prompt $'run tests'");
    }

    #[test]
    fn supports_prompt_and_model_replacements_in_user_args() {
        let provider = find_by_key("opencode").expect("opencode provider should exist");
        let command = provider.build_launch_command(LaunchContext {
            prompt: Some("triage bug"),
            model: Some("fast"),
            args: Some("--prompt {prompt} --model {model}"),
        });

        assert_eq!(command, "opencode --prompt $'triage bug' --model $'fast'");
    }

    #[test]
    fn supports_mock_provider_impl_for_tests() {
        let provider = MockProvider;
        let command = provider.build_launch_command(LaunchContext {
            prompt: Some("ignored"),
            model: Some("cheap"),
            args: Some("--dry-run"),
        });

        assert_eq!(command, "mock-cli --model $'cheap' --dry-run");
    }

    #[test]
    fn escapes_shell_sensitive_characters_in_prompt() {
        let provider = find_by_key("opencode").expect("opencode provider should exist");
        let command = provider.build_launch_command(LaunchContext {
            prompt: Some("run `pwd` and $HOME safely; don't expand"),
            model: None,
            args: None,
        });

        assert_eq!(
            command,
            "opencode --prompt $'run `pwd` and $HOME safely; don\\'t expand'"
        );
    }

    #[test]
    fn encodes_multiline_prompts_without_literal_newlines() {
        let provider = find_by_key("opencode").expect("opencode provider should exist");
        let command = provider.build_launch_command(LaunchContext {
            prompt: Some("line one\nline two"),
            model: None,
            args: None,
        });

        assert!(!command.contains('\n'));
        assert!(command.contains("line one\\nline two"));
    }

    #[test]
    fn exposes_provider_list() {
        assert_eq!(all().len(), 1);
        assert_eq!(all()[0].metadata().key, "opencode");
    }
}
