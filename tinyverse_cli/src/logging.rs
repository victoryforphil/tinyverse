use anyhow::{Context, Result, anyhow};
use nu_ansi_term::{Color, Style};
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const RUST_INFO_ENV: &str = "RUST_INFO";
const DEFAULT_LOG_LEVEL: &str = "INFO";
const LEVEL_BADGE_WIDTH: usize = 5;
pub struct TinyverseFormat;

impl<S, N> FormatEvent<S, N> for TinyverseFormat
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
        let badge = format!(
            "{:<width$}",
            level_label(metadata.level()),
            width = LEVEL_BADGE_WIDTH
        );
        let mut message_visitor = EventMessageVisitor::default();
        event.record(&mut message_visitor);
        let message = message_visitor.rendered_message();
        let mut lines = message.lines();

        if let Some(first_line) = lines.next() {
            writeln!(
                writer,
                "{} {}",
                level_style(metadata.level()).paint(badge),
                first_line
            )?;
            for line in lines {
                writeln!(writer, "{} {}", continuation_prefix(), line)?;
            }
            return Ok(());
        }

        writeln!(writer, "{}", level_style(metadata.level()).paint(badge))
    }
}

pub fn init() -> Result<()> {
    let filter_value =
        std::env::var(RUST_INFO_ENV).unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_owned());
    let env_filter = EnvFilter::try_new(filter_value)
        .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_LEVEL))
        .context("failed to build env filter")?;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .event_format(TinyverseFormat);

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|error| anyhow!("failed to initialize tracing subscriber: {error}"))?;

    Ok(())
}

fn level_style(level: &Level) -> Style {
    match *level {
        Level::ERROR => Style::new().on(Color::Red).fg(Color::White).bold(),
        Level::WARN => Style::new().on(Color::Yellow).fg(Color::Black).bold(),
        Level::INFO => Style::new().on(Color::Blue).fg(Color::White),
        Level::DEBUG => Style::new().on(Color::Cyan).fg(Color::Black),
        Level::TRACE => Style::new().on(Color::Purple).fg(Color::White),
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

fn continuation_prefix() -> &'static str {
    "      |-"
}

#[derive(Default)]
struct EventMessageVisitor {
    message: Option<String>,
    extras: Vec<String>,
}

impl EventMessageVisitor {
    fn rendered_message(self) -> String {
        match (self.message, self.extras.is_empty()) {
            (Some(message), true) => message,
            (Some(mut message), false) => {
                message.push_str(" (");
                message.push_str(&self.extras.join(", "));
                message.push(')');
                message
            }
            (None, false) => self.extras.join(", "),
            (None, true) => String::new(),
        }
    }

    fn record_pair(&mut self, field: &Field, value: String) {
        if field.name() == "message" {
            self.message = Some(value);
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
