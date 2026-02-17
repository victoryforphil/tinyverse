use std::io::IsTerminal;

use anyhow::{Result, bail};

use crate::picker::{PickerItem, PickerOutcome, run_picker};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgSelectOption {
    pub label: String,
    pub value: String,
}

impl ArgSelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredArgSelectConfig {
    pub arg_name: String,
    pub title: String,
    pub cli_example: String,
    pub cancelled_message: String,
}

impl RequiredArgSelectConfig {
    pub fn new(
        arg_name: impl Into<String>,
        title: impl Into<String>,
        cli_example: impl Into<String>,
    ) -> Self {
        Self {
            arg_name: arg_name.into(),
            title: title.into(),
            cli_example: cli_example.into(),
            cancelled_message: "selection cancelled".to_owned(),
        }
    }

    pub fn with_cancelled_message(mut self, message: impl Into<String>) -> Self {
        self.cancelled_message = message.into();
        self
    }
}

pub fn select_required_arg(
    config: RequiredArgSelectConfig,
    options: Vec<ArgSelectOption>,
) -> Result<String> {
    select_required_arg_with_runner(config, options, stdio_is_tty(), |title, items| {
        run_picker(title, items)
    })
}

fn select_required_arg_with_runner<F>(
    config: RequiredArgSelectConfig,
    options: Vec<ArgSelectOption>,
    tty_available: bool,
    run: F,
) -> Result<String>
where
    F: FnOnce(&str, Vec<PickerItem>) -> Result<PickerOutcome>,
{
    if !tty_available {
        bail!(
            "{} argument is required in non-interactive mode; pass it explicitly\nExample: {}",
            config.arg_name,
            config.cli_example
        );
    }

    if options.is_empty() {
        bail!("no available values to select for `{}`", config.arg_name);
    }

    let items: Vec<PickerItem> = options
        .into_iter()
        .map(|opt| PickerItem {
            label: opt.label,
            key: opt.value,
        })
        .collect();

    match run(&config.title, items)? {
        PickerOutcome::Selected(value) => Ok(value),
        PickerOutcome::Cancelled => bail!(config.cancelled_message),
    }
}

fn stdio_is_tty() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

#[cfg(test)]
mod tests {
    use super::{ArgSelectOption, RequiredArgSelectConfig, select_required_arg_with_runner};
    use crate::PickerOutcome;

    fn base_config() -> RequiredArgSelectConfig {
        RequiredArgSelectConfig::new(
            "session",
            "Select a session to attach",
            "tinyverse attach <session>",
        )
    }

    #[test]
    fn rejects_non_interactive_mode_with_example() {
        let err = select_required_arg_with_runner(base_config(), Vec::new(), false, |_, _| {
            Ok(PickerOutcome::Cancelled)
        })
        .expect_err("non-interactive selection should fail");

        let msg = err.to_string();
        assert!(msg.contains("session argument is required in non-interactive mode"));
        assert!(msg.contains("tinyverse attach <session>"));
    }

    #[test]
    fn rejects_empty_options() {
        let err = select_required_arg_with_runner(base_config(), Vec::new(), true, |_, _| {
            Ok(PickerOutcome::Cancelled)
        })
        .expect_err("empty options should fail");

        assert!(
            err.to_string()
                .contains("no available values to select for `session`")
        );
    }

    #[test]
    fn returns_selected_value() {
        let selected = select_required_arg_with_runner(
            base_config(),
            vec![ArgSelectOption::new("Redding", "tinyverse_redding")],
            true,
            |title, items| {
                assert_eq!(title, "Select a session to attach");
                assert_eq!(items.len(), 1);
                Ok(PickerOutcome::Selected(items[0].key.clone()))
            },
        )
        .expect("selection should succeed");

        assert_eq!(selected, "tinyverse_redding");
    }

    #[test]
    fn returns_custom_cancel_message() {
        let err = select_required_arg_with_runner(
            base_config().with_cancelled_message("session selection cancelled"),
            vec![ArgSelectOption::new("Redding", "tinyverse_redding")],
            true,
            |_, _| Ok(PickerOutcome::Cancelled),
        )
        .expect_err("cancel should return configured message");

        assert_eq!(err.to_string(), "session selection cancelled");
    }
}
