use std::io::IsTerminal;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use log::{error, info};

use super::args::AttachArgs;

pub fn execute(args: AttachArgs) -> Result<()> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        error!(
            "Attach needs an interactive terminal (session: {})",
            args.session
        );
        bail!(
            "attach requires an interactive terminal (TTY); run this command directly in your shell"
        );
    }

    let tmux_args = build_tmux_attach_args(std::env::var_os("TMUX").is_some(), &args.session);

    let status = Command::new("tmux")
        .args(&tmux_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to run tmux for session `{}`", args.session))?;

    if !status.success() {
        error!(
            "Attach failed for session {} (exit code: {:?})",
            args.session,
            status.code()
        );
        bail!("failed to attach/switch to session `{}`", args.session);
    }

    info!("Attached to session {}", args.session);

    Ok(())
}

fn build_tmux_attach_args(in_tmux: bool, session: &str) -> Vec<String> {
    if in_tmux {
        return vec![
            "switch-client".to_owned(),
            "-t".to_owned(),
            session.to_owned(),
        ];
    }

    vec![
        "attach-session".to_owned(),
        "-t".to_owned(),
        session.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::build_tmux_attach_args;

    #[test]
    fn outside_tmux_uses_attach_session() {
        let args = build_tmux_attach_args(false, "tinyverse_1");
        assert_eq!(args, vec!["attach-session", "-t", "tinyverse_1"]);
    }

    #[test]
    fn inside_tmux_uses_switch_client() {
        let args = build_tmux_attach_args(true, "tinyverse_1");
        assert_eq!(args, vec!["switch-client", "-t", "tinyverse_1"]);
    }
}
