use crate::components::{
    ActionLine, DetailSection, ErrorBlock, GuidanceLine, LabeledField, SectionHeader, StatusBadge,
    StripeMode, StyledTable, SummaryFooter,
};
use crate::render::{RenderContext, RenderMode};
use crate::theme::{DefaultTheme, MinimalTheme, Theme, Tone};

/// Theme profile used by demo outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleTheme {
    Default,
    Minimal,
}

impl ExampleTheme {
    fn theme(self) -> &'static dyn Theme {
        static DEFAULT_THEME: DefaultTheme = DefaultTheme;
        static MINIMAL_THEME: MinimalTheme = MinimalTheme;
        match self {
            ExampleTheme::Default => &DEFAULT_THEME,
            ExampleTheme::Minimal => &MINIMAL_THEME,
        }
    }
}

pub fn primitive_gallery_output(mode: RenderMode, width: Option<usize>) -> String {
    primitive_gallery_output_with_theme(mode, width, ExampleTheme::Default)
}

pub fn primitive_gallery_output_with_theme(
    mode: RenderMode,
    width: Option<usize>,
    theme: ExampleTheme,
) -> String {
    let context = RenderContext::new(mode, width, theme.theme());

    let actions = vec![
        SectionHeader::new("Action Lines").render(&context),
        ActionLine::new("INFO", "Scanning for tinyverse sessions", Tone::Info).render(&context),
        ActionLine::new("DONE", "Session tinyverse_213 attached", Tone::Success).render(&context),
        ActionLine::new(
            "WARN",
            "Session tinyverse_old has no windows",
            Tone::Warning,
        )
        .render(&context),
        ActionLine::new("ERROR", "Session tinyverse_404 not found", Tone::Error).render(&context),
    ]
    .join("\n");

    let badges = vec![
        SectionHeader::new("Status Badges").render(&context),
        vec![
            StatusBadge::new("ACTIVE", Tone::Success).render(&context),
            StatusBadge::new("IDLE", Tone::Neutral).render(&context),
            StatusBadge::new("FAILED", Tone::Error).render(&context),
        ]
        .join("  "),
    ]
    .join("\n");

    let details = DetailSection::new("Session Details")
        .with_field(LabeledField::new("ID", "$3"))
        .with_field(LabeledField::new("Name", "tinyverse_213"))
        .with_field(LabeledField::new("Status", "attached"))
        .with_field(LabeledField::new("Windows", "2"))
        .render(&context);

    let table_section = vec![
        SectionHeader::new("Session Table").render(&context),
        StyledTable::new(vec!["ID", "NAME", "STATUS", "WINDOWS"])
            .with_numeric_columns(&[3])
            .with_stripe_mode(StripeMode::DimEvenRows)
            .with_row(vec![
                "$1".into(),
                "tinyverse_alpha".into(),
                "attached".into(),
                "2".into(),
            ])
            .with_row(vec![
                "$2".into(),
                "tinyverse_beta".into(),
                "idle".into(),
                "1".into(),
            ])
            .render(&context),
    ]
    .join("\n");

    let error = ErrorBlock::new("Unable to attach to tinyverse_404")
        .with_detail("tmux session was not found on this host")
        .with_guidance("Run tinyverse list --all to see available sessions")
        .render(&context);

    let footer = vec![
        SummaryFooter::new("2 active, 1 idle, 1 failed").render(&context),
        GuidanceLine::new("Run tinyverse attach <name> to connect").render(&context),
    ]
    .join("\n");

    vec![actions, badges, details, table_section, error, footer].join("\n\n")
}

pub fn list_report_demo_output(mode: RenderMode, width: Option<usize>) -> String {
    list_report_demo_output_with_theme(mode, width, ExampleTheme::Default)
}

pub fn list_report_demo_output_with_theme(
    mode: RenderMode,
    width: Option<usize>,
    theme: ExampleTheme,
) -> String {
    let context = RenderContext::new(mode, width, theme.theme());

    let header = SectionHeader::new("Sessions").render(&context);
    let status = ActionLine::new("INFO", "Found 3 tinyverse sessions", Tone::Info).render(&context);

    let table = StyledTable::new(vec!["ID", "NAME", "STATUS", "WINDOWS"])
        .with_numeric_columns(&[3])
        .with_stripe_mode(StripeMode::DimEvenRows)
        .with_row(vec![
            "$1".into(),
            "tinyverse_alpha".into(),
            "attached".into(),
            "3".into(),
        ])
        .with_row(vec![
            "$2".into(),
            "tinyverse_beta".into(),
            "idle".into(),
            "1".into(),
        ])
        .with_row(vec![
            "$3".into(),
            "tinyverse_long_project_name_for_truncation".into(),
            "attached".into(),
            "2".into(),
        ])
        .render(&context);

    let badges = vec![
        StatusBadge::new("2 attached", Tone::Success).render(&context),
        StatusBadge::new("1 idle", Tone::Neutral).render(&context),
    ]
    .join("  ");

    let summary = SummaryFooter::new("3 sessions (filtered by prefix tinyverse_)").render(&context);
    let hint = GuidanceLine::new("Use --all to include every tmux session").render(&context);

    vec![
        header,
        status,
        String::new(),
        table,
        String::new(),
        badges,
        summary,
        hint,
    ]
    .join("\n")
}
