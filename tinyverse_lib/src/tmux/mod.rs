mod error;
mod options;
mod types;

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use error::TmuxError;
pub use options::{CapturePaneOptions, ListSessionsOptions, SendKeysOptions, SpawnSessionOptions};
pub use types::{
    CapturedPane, PaneTarget, PanelRole, SessionSummary, SessionTarget, SpawnSessionResult,
};

const LIST_SESSIONS_FORMAT: &str =
    "#{session_id}\t#{session_name}\t#{session_attached}\t#{session_windows}";
const LIST_PANES_FORMAT: &str = "#{pane_id}\t#{pane_index}\t#{pane_title}";

#[derive(Debug, Clone)]
pub struct TmuxClient {
    binary: PathBuf,
}

impl TmuxClient {
    pub fn new() -> Self {
        Self {
            binary: PathBuf::from("tmux"),
        }
    }

    pub fn with_bin(binary: impl Into<PathBuf>) -> Self {
        Self {
            binary: binary.into(),
        }
    }

    pub fn spawn_session(
        &self,
        options: SpawnSessionOptions,
    ) -> Result<SpawnSessionResult, TmuxError> {
        let session = SessionTarget::new(options.session_name.clone());

        let mut new_session_args = vec![
            "new-session".to_owned(),
            "-d".to_owned(),
            "-P".to_owned(),
            "-F".to_owned(),
            "#{pane_id}".to_owned(),
            "-s".to_owned(),
            session.as_str().to_owned(),
        ];

        if let Some(working_dir) = options.working_dir.as_ref() {
            new_session_args.push("-c".to_owned());
            new_session_args.push(working_dir.display().to_string());
        }

        let console_pane_id = self.run_tmux(new_session_args)?;

        if console_pane_id.is_empty() {
            return Err(TmuxError::ParseOutput {
                command: "new-session",
                details: "missing pane id in command output".to_owned(),
                output: String::new(),
            });
        }

        let mut split_args = vec![
            "split-window".to_owned(),
            "-h".to_owned(),
            "-P".to_owned(),
            "-F".to_owned(),
            "#{pane_id}".to_owned(),
            "-t".to_owned(),
            console_pane_id.clone(),
        ];

        if let Some(working_dir) = options.working_dir.as_ref() {
            split_args.push("-c".to_owned());
            split_args.push(working_dir.display().to_string());
        }

        let agent_pane_id = self.run_tmux(split_args)?;

        if agent_pane_id.is_empty() {
            return Err(TmuxError::ParseOutput {
                command: "split-window",
                details: "missing pane id in command output".to_owned(),
                output: String::new(),
            });
        }

        self.run_tmux(vec![
            "select-pane".to_owned(),
            "-t".to_owned(),
            console_pane_id.clone(),
            "-T".to_owned(),
            PanelRole::Console.as_title().to_owned(),
        ])?;

        self.run_tmux(vec![
            "select-pane".to_owned(),
            "-t".to_owned(),
            agent_pane_id.clone(),
            "-T".to_owned(),
            PanelRole::Agent.as_title().to_owned(),
        ])?;

        if let Some(command) = options.console_command.as_deref() {
            self.send_literal_to_target(&console_pane_id, command, true)?;
        }

        if let Some(command) = options.agent_command.as_deref() {
            self.send_literal_to_target(&agent_pane_id, command, true)?;
        }

        self.run_tmux(vec![
            "select-pane".to_owned(),
            "-t".to_owned(),
            console_pane_id.clone(),
        ])?;

        Ok(SpawnSessionResult {
            session,
            console_pane_id,
            agent_pane_id,
        })
    }

    pub fn list_sessions(
        &self,
        _options: ListSessionsOptions,
    ) -> Result<Vec<SessionSummary>, TmuxError> {
        let args = vec![
            "list-sessions".to_owned(),
            "-F".to_owned(),
            LIST_SESSIONS_FORMAT.to_owned(),
        ];

        match self.run_tmux(args) {
            Ok(stdout) => parse_list_sessions_output(&stdout),
            Err(TmuxError::CommandFailed { stderr, .. })
                if stderr.contains("no server running")
                    || stderr.contains("failed to connect to server") =>
            {
                Ok(Vec::new())
            }
            Err(error) => Err(error),
        }
    }

    pub fn kill_session(&self, target: SessionTarget) -> Result<(), TmuxError> {
        self.run_tmux(vec![
            "kill-session".to_owned(),
            "-t".to_owned(),
            target.as_str().to_owned(),
        ])?;

        Ok(())
    }

    pub fn capture_pane(&self, options: CapturePaneOptions) -> Result<CapturedPane, TmuxError> {
        let pane_id = self.resolve_pane_id(&options.session, options.pane.as_ref())?;
        let mut args = vec![
            "capture-pane".to_owned(),
            "-p".to_owned(),
            "-J".to_owned(),
            "-t".to_owned(),
            pane_id.clone(),
        ];

        if let Some(start_line) = options.start_line {
            args.push("-S".to_owned());
            args.push(start_line.to_string());
        }

        if let Some(end_line) = options.end_line {
            args.push("-E".to_owned());
            args.push(end_line.to_string());
        }

        let text = self.run_tmux(args)?;

        Ok(CapturedPane {
            session: options.session,
            pane_id,
            text,
        })
    }

    pub fn send_keys(&self, options: SendKeysOptions) -> Result<(), TmuxError> {
        let pane_id = self.resolve_pane_id(&options.session, options.pane.as_ref())?;
        self.send_literal_to_target(&pane_id, &options.command, options.press_enter)
    }

    fn send_literal_to_target(
        &self,
        pane_target: &str,
        command: &str,
        press_enter: bool,
    ) -> Result<(), TmuxError> {
        if command.is_empty() && !press_enter {
            return Ok(());
        }

        let mut args = vec![
            "send-keys".to_owned(),
            "-t".to_owned(),
            pane_target.to_owned(),
        ];

        if !command.is_empty() {
            args.push("-l".to_owned());
            args.push(command.to_owned());
        }

        if press_enter {
            args.push("Enter".to_owned());
        }

        self.run_tmux(args)?;
        Ok(())
    }

    fn resolve_pane_id(
        &self,
        session: &SessionTarget,
        request: Option<&PaneTarget>,
    ) -> Result<String, TmuxError> {
        if let Some(PaneTarget::PaneId(pane_id)) = request {
            return Ok(pane_id.clone());
        }

        let panes = self.list_panes(session)?;
        resolve_pane_from_summaries(&panes, request).ok_or_else(|| TmuxError::MissingPane {
            session: session.as_str().to_owned(),
            request: request_description(request),
        })
    }

    fn list_panes(&self, session: &SessionTarget) -> Result<Vec<PaneSummary>, TmuxError> {
        let output = self.run_tmux(vec![
            "list-panes".to_owned(),
            "-t".to_owned(),
            session.as_str().to_owned(),
            "-F".to_owned(),
            LIST_PANES_FORMAT.to_owned(),
        ])?;

        parse_list_panes_output(&output)
    }

    fn run_tmux(&self, args: Vec<String>) -> Result<String, TmuxError> {
        let command = format_command(&self.binary, &args);
        let output = Command::new(&self.binary)
            .args(&args)
            .output()
            .map_err(|source| {
                if source.kind() == io::ErrorKind::NotFound {
                    TmuxError::BinaryNotFound {
                        binary: self.binary.display().to_string(),
                        source,
                    }
                } else {
                    TmuxError::CommandIo {
                        command: command.clone(),
                        source,
                    }
                }
            })?;

        if output.status.success() {
            return Ok(bytes_to_clean_string(&output.stdout));
        }

        Err(TmuxError::CommandFailed {
            command,
            exit_code: output.status.code(),
            stdout: bytes_to_clean_string(&output.stdout),
            stderr: bytes_to_clean_string(&output.stderr),
        })
    }
}

impl Default for TmuxClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PaneSummary {
    pane_id: String,
    pane_index: u32,
    pane_title: String,
}

fn parse_list_sessions_output(output: &str) -> Result<Vec<SessionSummary>, TmuxError> {
    if output.trim().is_empty() {
        return Ok(Vec::new());
    }

    output
        .lines()
        .map(|line| {
            let mut fields = line.splitn(4, '\t');
            let session_id = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-sessions",
                details: format!("expected 4 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let session_name = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-sessions",
                details: format!("expected 4 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let attached = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-sessions",
                details: format!("expected 4 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let windows = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-sessions",
                details: format!("expected 4 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let attached_clients =
                attached
                    .parse::<u32>()
                    .map_err(|parse_error| TmuxError::ParseOutput {
                        command: "list-sessions",
                        details: format!(
                            "invalid attached client count `{attached}`: {parse_error}"
                        ),
                        output: line.to_owned(),
                    })?;

            let windows = windows
                .parse::<u32>()
                .map_err(|parse_error| TmuxError::ParseOutput {
                    command: "list-sessions",
                    details: format!("invalid windows count `{windows}`: {parse_error}"),
                    output: line.to_owned(),
                })?;

            Ok(SessionSummary {
                session_id: session_id.to_owned(),
                session_name: session_name.to_owned(),
                attached_clients,
                windows,
            })
        })
        .collect()
}

fn parse_list_panes_output(output: &str) -> Result<Vec<PaneSummary>, TmuxError> {
    if output.trim().is_empty() {
        return Ok(Vec::new());
    }

    output
        .lines()
        .map(|line| {
            let mut fields = line.splitn(3, '\t');
            let pane_id = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-panes",
                details: format!("expected 3 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let pane_index = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-panes",
                details: format!("expected 3 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let pane_title = fields.next().ok_or_else(|| TmuxError::ParseOutput {
                command: "list-panes",
                details: format!("expected 3 fields, got less: {line:?}"),
                output: line.to_owned(),
            })?;

            let pane_index =
                pane_index
                    .parse::<u32>()
                    .map_err(|parse_error| TmuxError::ParseOutput {
                        command: "list-panes",
                        details: format!("invalid pane index `{pane_index}`: {parse_error}"),
                        output: line.to_owned(),
                    })?;

            Ok(PaneSummary {
                pane_id: pane_id.to_owned(),
                pane_index,
                pane_title: pane_title.to_owned(),
            })
        })
        .collect()
}

fn resolve_pane_from_summaries(
    panes: &[PaneSummary],
    request: Option<&PaneTarget>,
) -> Option<String> {
    if panes.is_empty() {
        return None;
    }

    let desired_role = match request {
        Some(PaneTarget::Role(role)) => Some(*role),
        Some(PaneTarget::PaneId(_)) => None,
        None => Some(PanelRole::Console),
    };

    if let Some(role) = desired_role
        && let Some(found) = panes
            .iter()
            .find(|pane| pane.pane_title.eq_ignore_ascii_case(role.as_title()))
    {
        return Some(found.pane_id.clone());
    }

    panes
        .iter()
        .min_by_key(|pane| pane.pane_index)
        .map(|pane| pane.pane_id.clone())
}

fn request_description(request: Option<&PaneTarget>) -> String {
    match request {
        Some(PaneTarget::PaneId(pane_id)) => format!("pane_id:{pane_id}"),
        Some(PaneTarget::Role(role)) => format!("role:{role}"),
        None => "role:console(default)".to_owned(),
    }
}

fn format_command(binary: &Path, args: &[String]) -> String {
    let mut parts = vec![binary.display().to_string()];
    parts.extend(args.iter().map(|arg| shell_quote(arg)));
    parts.join(" ")
}

fn shell_quote(arg: &str) -> String {
    if arg.is_empty() {
        return "\"\"".to_owned();
    }

    if arg.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'/' | b':' | b'%')
    }) {
        return arg.to_owned();
    }

    let escaped = arg.replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn bytes_to_clean_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        CapturePaneOptions, LIST_PANES_FORMAT, LIST_SESSIONS_FORMAT, ListSessionsOptions,
        PaneSummary, PaneTarget, PanelRole, SendKeysOptions, SessionTarget, SpawnSessionOptions,
        TmuxClient, TmuxError, format_command, parse_list_panes_output, parse_list_sessions_output,
        resolve_pane_from_summaries,
    };

    #[test]
    fn parse_sessions_output() {
        let output = "$1\tfirst\t1\t2\n$2\tsecond\t0\t4";
        let sessions = parse_list_sessions_output(output).expect("should parse sessions");

        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].session_id, "$1");
        assert_eq!(sessions[0].session_name, "first");
        assert_eq!(sessions[0].attached_clients, 1);
        assert_eq!(sessions[0].windows, 2);
    }

    #[test]
    fn parse_sessions_output_rejects_invalid_counts() {
        let output = "$1\tfirst\tnope\t2";
        let error = parse_list_sessions_output(output).expect_err("should reject invalid output");

        assert!(matches!(error, TmuxError::ParseOutput { .. }));
    }

    #[test]
    fn parse_panes_output() {
        let output = "%1\t0\tconsole\n%2\t1\tagent";
        let panes = parse_list_panes_output(output).expect("should parse panes");

        assert_eq!(panes.len(), 2);
        assert_eq!(panes[1].pane_id, "%2");
        assert_eq!(panes[1].pane_index, 1);
        assert_eq!(panes[1].pane_title, "agent");
    }

    #[test]
    fn resolve_target_prefers_role_title_and_falls_back_to_first() {
        let panes = vec![
            PaneSummary {
                pane_id: "%9".to_owned(),
                pane_index: 4,
                pane_title: "misc".to_owned(),
            },
            PaneSummary {
                pane_id: "%5".to_owned(),
                pane_index: 0,
                pane_title: "console".to_owned(),
            },
        ];

        let console =
            resolve_pane_from_summaries(&panes, Some(&PaneTarget::Role(PanelRole::Console)));
        assert_eq!(console, Some("%5".to_owned()));

        let fallback_agent =
            resolve_pane_from_summaries(&panes, Some(&PaneTarget::Role(PanelRole::Agent)));
        assert_eq!(fallback_agent, Some("%5".to_owned()));
    }

    #[test]
    fn format_command_quotes_complex_args() {
        let command = format_command(
            std::path::Path::new("tmux"),
            &[
                "send-keys".to_owned(),
                "-t".to_owned(),
                "%1".to_owned(),
                "-l".to_owned(),
                "echo hello world".to_owned(),
            ],
        );

        assert_eq!(command, "tmux send-keys -t %1 -l \"echo hello world\"");
    }

    #[test]
    fn constants_include_expected_formats() {
        assert!(LIST_SESSIONS_FORMAT.contains("session_name"));
        assert!(LIST_PANES_FORMAT.contains("pane_title"));
    }

    #[test]
    #[ignore]
    fn integration_spawn_send_capture_kill() {
        if std::process::Command::new("tmux")
            .arg("-V")
            .output()
            .is_err()
        {
            return;
        }

        let client = TmuxClient::new();
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_millis();
        let session_name = format!("tinyverse_test_{millis}");

        let spawn = client
            .spawn_session(SpawnSessionOptions::new(session_name.clone()))
            .expect("spawn should succeed");
        assert_eq!(spawn.session.as_str(), session_name);

        client
            .send_keys(SendKeysOptions {
                session: SessionTarget::from(session_name.as_str()),
                pane: Some(PaneTarget::Role(PanelRole::Console)),
                command: "echo tinyverse_tmux_smoke".to_owned(),
                press_enter: true,
            })
            .expect("send should succeed");

        std::thread::sleep(std::time::Duration::from_millis(150));

        let captured = client
            .capture_pane(CapturePaneOptions {
                session: SessionTarget::from(session_name.as_str()),
                pane: Some(PaneTarget::Role(PanelRole::Console)),
                start_line: Some(-100),
                end_line: None,
            })
            .expect("capture should succeed");

        let _ = client.kill_session(SessionTarget::from(session_name.as_str()));
        assert!(captured.text.contains("tinyverse_tmux_smoke"));
    }

    #[test]
    fn list_sessions_returns_error_when_binary_is_missing() {
        let client = TmuxClient::with_bin("/definitely/not/tmux");
        let result = client.list_sessions(ListSessionsOptions);
        assert!(result.is_err());
    }
}
