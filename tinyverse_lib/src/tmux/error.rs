use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TmuxError {
    BinaryNotFound {
        binary: String,
        source: io::Error,
    },
    CommandIo {
        command: String,
        source: io::Error,
    },
    CommandFailed {
        command: String,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    ParseOutput {
        command: &'static str,
        details: String,
        output: String,
    },
    MissingPane {
        session: String,
        request: String,
    },
}

impl fmt::Display for TmuxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinaryNotFound { binary, .. } => {
                write!(f, "tmux binary not found: {binary}")
            }
            Self::CommandIo { command, source } => {
                write!(f, "failed to execute command `{command}`: {source}")
            }
            Self::CommandFailed {
                command,
                exit_code,
                stdout,
                stderr,
            } => {
                write!(
                    f,
                    "command `{command}` failed (exit_code={:?}, stdout={:?}, stderr={:?})",
                    exit_code, stdout, stderr
                )
            }
            Self::ParseOutput {
                command,
                details,
                output,
            } => {
                write!(
                    f,
                    "failed to parse `{command}` output: {details} (output={output:?})"
                )
            }
            Self::MissingPane { session, request } => {
                write!(
                    f,
                    "unable to resolve pane for session `{session}` with request `{request}`"
                )
            }
        }
    }
}

impl Error for TmuxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BinaryNotFound { source, .. } | Self::CommandIo { source, .. } => Some(source),
            Self::CommandFailed { .. } | Self::ParseOutput { .. } | Self::MissingPane { .. } => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::TmuxError;

    #[test]
    fn command_error_contains_context() {
        let error = TmuxError::CommandIo {
            command: "tmux list-sessions".to_owned(),
            source: io::Error::other("boom"),
        };

        let rendered = error.to_string();
        assert!(rendered.contains("tmux list-sessions"));
    }
}
