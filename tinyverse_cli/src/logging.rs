use std::io::IsTerminal;

use anyhow::{anyhow, Context, Result};
use tinyverse_ui::{ActionLine, DefaultTheme, ErrorBlock, Panel, RenderContext, RenderMode, Tone};
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const RUST_INFO_ENV: &str = "RUST_INFO";
const NO_COLOR_ENV: &str = "NO_COLOR";
const DEFAULT_LOG_LEVEL: &str = "INFO";

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

pub fn init() -> Result<()> {
    let filter_value =
        std::env::var(RUST_INFO_ENV).unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_owned());
    let env_filter = EnvFilter::try_new(filter_value)
        .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_LEVEL))
        .context("failed to build env filter")?;

    let use_ansi = std::io::stdout().is_terminal() && std::env::var_os(NO_COLOR_ENV).is_none();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_ansi(use_ansi)
        .event_format(FancyFormat::new(use_ansi));

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|error| anyhow!("failed to initialize tracing subscriber: {error}"))?;

    Ok(())
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
