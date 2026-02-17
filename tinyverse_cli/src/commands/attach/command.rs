use std::process::Command;

use anyhow::{bail, Context, Result};
use log::info;

use super::args::AttachArgs;

pub fn execute(args: AttachArgs) -> Result<()> {
    let tmux_args = build_tmux_attach_args(std::env::var_os("TMUX").is_some(), &args.session);

    let output = Command::new("tmux")
        .args(&tmux_args)
        .output()
        .with_context(|| format!("failed to run tmux for session `{}`", args.session))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!(
            "failed to attach/switch to session `{}` (stderr={stderr:?})",
            args.session
        );
    }

    info!(
        "CLI // Sessions // Attached to session (meta={{\"session\":\"{}\",\"in_tmux\":{}}})",
        args.session,
        std::env::var_os("TMUX").is_some()
    );

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
