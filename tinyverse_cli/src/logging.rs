use std::fs::OpenOptions;
use std::io::{IsTerminal, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow};
use tinyverse_lib::resolve_tinyverse_paths;
use tinyverse_ui::{ActionLine, DefaultTheme, ErrorBlock, Panel, RenderContext, RenderMode, Tone};
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const RUST_INFO_ENV: &str = "RUST_INFO";
const NO_COLOR_ENV: &str = "NO_COLOR";
const DEFAULT_LOG_LEVEL: &str = "INFO";
const TUI_DEFAULT_LOG_LEVEL: &str = "DEBUG";
const LOGS_DIR_NAME: &str = "logs";

#[derive(Debug, Clone, Copy)]
pub struct InitOptions {
    pub stdout_enabled: bool,
    pub default_level: &'static str,
}

impl InitOptions {
    pub fn cli_default() -> Self {
        Self {
            stdout_enabled: true,
            default_level: DEFAULT_LOG_LEVEL,
        }
    }

    pub fn tui_mode() -> Self {
        Self {
            stdout_enabled: false,
            default_level: TUI_DEFAULT_LOG_LEVEL,
        }
    }
}

pub struct FancyFormat {
    use_ansi: bool,
}

impl FancyFormat {
    fn new(use_ansi: bool) -> Self {
        Self { use_ansi }
    }

    fn render_context(&self) -> RenderContext<'static> {
        static THEME: DefaultTheme = DefaultTheme;
        let mode = if self.use_ansi {
            RenderMode::Ansi
        } else {
            RenderMode::Plain
        };
        RenderContext::new(mode, None, &THEME)
    }
}

impl<S, N> FormatEvent<S, N> for FancyFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();
        let context = self.render_context();
        let mut message_visitor = EventMessageVisitor::default();
        event.record(&mut message_visitor);

        if *metadata.level() == Level::ERROR && message_visitor.should_render_error_block() {
            let primary_message = message_visitor.primary_message();
            let mut message_lines = primary_message.lines();
            let title = message_lines.next().unwrap_or_default();
            let mut error_block = ErrorBlock::new(title);
            if let Some(detail) = message_lines.next() {
                error_block = error_block.with_detail(detail);
            }
            if let Some(guidance) = message_visitor.guidance.as_deref() {
                error_block = error_block.with_guidance(guidance);
            }

            let panel = Panel::new(error_block.render(&context))
                .with_title("Error")
                .with_tone(Tone::Error);
            writeln!(writer, "{}", panel.render(&context))?;

            for line in message_lines {
                writeln!(writer, "{} {}", continuation_prefix(), line)?;
            }

            if let Some(extras) = message_visitor.extras_line() {
                writeln!(writer, "{} {}", continuation_prefix(), extras)?;
            }

            return Ok(());
        }

        let rendered_message = message_visitor.rendered_message();
        if rendered_message.contains('\n') {
            let panel = match *metadata.level() {
                Level::WARN => Panel::new(rendered_message)
                    .with_title(level_label(metadata.level()))
                    .with_tone(Tone::Warning),
                Level::ERROR => Panel::new(rendered_message)
                    .with_title(level_label(metadata.level()))
                    .with_tone(Tone::Error),
                _ => Panel::new(rendered_message).with_title(level_label(metadata.level())),
            };
            writeln!(writer, "{}", panel.render(&context))?;
            return Ok(());
        }
        let mut lines = rendered_message.lines();

        if let Some(first_line) = lines.next() {
            let line = ActionLine::new(
                level_label(metadata.level()),
                first_line,
                level_tone(metadata.level()),
            )
            .render(&context);
            writeln!(writer, "{line}")?;
            for line in lines {
                writeln!(writer, "{} {}", continuation_prefix(), line)?;
            }
            return Ok(());
        }

        let line = ActionLine::new(
            level_label(metadata.level()),
            "",
            level_tone(metadata.level()),
        )
        .render(&context);
        writeln!(writer, "{line}")
    }
}

pub fn init(tinyverse_dir_home_override: Option<&Path>, options: InitOptions) -> Result<()> {
    let filter_value = std::env::var(RUST_INFO_ENV)
        .unwrap_or_else(|_| default_filter_for_options(options).to_owned());
    let env_filter = EnvFilter::try_new(filter_value)
        .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_LEVEL))
        .context("failed to build env filter")?;

    let tinyverse_paths = resolve_tinyverse_paths(tinyverse_dir_home_override)
        .context("failed to resolve tinyverse paths for log file")?;
    let logs_dir = tinyverse_paths.home_dir.join(LOGS_DIR_NAME);
    std::fs::create_dir_all(&logs_dir)
        .with_context(|| format!("failed to create log directory `{}`", logs_dir.display()))?;

    let log_file_path = logs_dir.join(format!("tinyverse-{}.log", unix_timestamp_millis()));
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .with_context(|| format!("failed to open log file `{}`", log_file_path.display()))?;
    let log_file = Arc::new(Mutex::new(log_file));

    let use_ansi = options.stdout_enabled
        && std::io::stdout().is_terminal()
        && std::env::var_os(NO_COLOR_ENV).is_none();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_ansi(use_ansi)
        .event_format(FancyFormat::new(use_ansi))
        .with_filter(tracing_subscriber::filter::filter_fn(move |_| {
            options.stdout_enabled
        }));
    let file_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(false)
        .with_writer({
            let log_file = Arc::clone(&log_file);
            move || SharedFileWriter::new(Arc::clone(&log_file))
        });

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(file_layer)
        .try_init()
        .map_err(|error| anyhow!("failed to initialize tracing subscriber: {error}"))?;

    Ok(())
}

fn default_filter_for_options(options: InitOptions) -> &'static str {
    if !options.stdout_enabled && options.default_level.eq_ignore_ascii_case("DEBUG") {
        return "warn,tinyverse_cli=debug,tinyverse_lib=debug,tinyverse_tui=debug,tinyverse_ui=debug,tinyverse_tui_components=debug";
    }

    options.default_level
}

fn unix_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis())
}

struct SharedFileWriter {
    file: Arc<Mutex<std::fs::File>>,
}

impl SharedFileWriter {
    fn new(file: Arc<Mutex<std::fs::File>>) -> Self {
        Self { file }
    }
}

impl Write for SharedFileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = self
            .file
            .lock()
            .map_err(|_| std::io::Error::other("log file writer lock poisoned"))?;
        file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self
            .file
            .lock()
            .map_err(|_| std::io::Error::other("log file writer lock poisoned"))?;
        file.flush()
    }
}

fn level_label(level: &Level) -> &'static str {
    match *level {
        Level::ERROR => "ERROR",
        Level::WARN => "WARN",
        Level::INFO => "INFO",
        Level::DEBUG => "DEBUG",
        Level::TRACE => "TRACE",
    }
}

fn level_tone(level: &Level) -> Tone {
    match *level {
        Level::ERROR => Tone::Error,
        Level::WARN => Tone::Warning,
        Level::INFO => Tone::Info,
        Level::DEBUG | Level::TRACE => Tone::Neutral,
    }
}

fn continuation_prefix() -> &'static str {
    "          |-"
}

#[derive(Default)]
struct EventMessageVisitor {
    message: Option<String>,
    guidance: Option<String>,
    extras: Vec<String>,
}

impl EventMessageVisitor {
    fn rendered_message(&self) -> String {
        match (&self.message, self.extras.is_empty()) {
            (Some(message), true) => message.clone(),
            (Some(message), false) => format!("{message} ({})", self.extras.join(", ")),
            (None, false) => self.extras.join(", "),
            (None, true) => String::new(),
        }
    }

    fn primary_message(&self) -> String {
        self.message
            .clone()
            .unwrap_or_else(|| self.extras.join(", "))
    }

    fn extras_line(&self) -> Option<String> {
        if self.extras.is_empty() {
            None
        } else {
            Some(self.extras.join(", "))
        }
    }

    fn should_render_error_block(&self) -> bool {
        true
    }

    fn record_pair(&mut self, field: &Field, value: String) {
        if field.name() == "message" {
            self.message = Some(value);
        } else if field.name() == "guidance" || field.name() == "hint" {
            self.guidance = Some(value);
        } else if field.name().starts_with("log.") {
            return;
        } else {
            self.extras.push(format!("{}={value}", field.name()));
        }
    }
}

impl Visit for EventMessageVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.record_pair(field, value.to_owned());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.record_pair(field, value.to_string());
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.record_pair(field, value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.record_pair(field, value.to_string());
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.record_pair(field, value.to_string());
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.record_pair(field, value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.record_pair(field, format!("{value:?}"));
    }
}
