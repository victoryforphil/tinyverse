use tinyverse_ui::components::{
    ActionLine, DetailSection, ErrorBlock, GuidanceLine, LabeledField, SectionHeader, StatusBadge,
    StyledTable, SummaryFooter,
};
use tinyverse_ui::render::{RenderContext, RenderMode};
use tinyverse_ui::theme::{DefaultTheme, Tone};

#[test]
fn primitive_components_render_in_plain_mode() {
    let theme = DefaultTheme;
    let context = RenderContext::new(RenderMode::Plain, Some(80), &theme);

    let action = ActionLine::new("OK", "Spawned tinyverse_123", Tone::Success).render(&context);
    let section = SectionHeader::new("Session").render(&context);
    let field = LabeledField::new("Name", "tinyverse_123").render(&context);
    let badge = StatusBadge::new("ATTACHED", Tone::Info).render(&context);
    let guidance = GuidanceLine::new("Use tinyverse attach tinyverse_123").render(&context);
    let footer = SummaryFooter::new("1 session active").render(&context);

    assert!(action.contains("Spawned"));
    assert!(section.contains("Session"));
    assert!(field.contains("Name"));
    assert!(badge.contains("ATTACHED"));
    assert!(guidance.contains("Next:"));
    assert!(footer.contains("Summary:"));
}

#[test]
fn composed_blocks_render_expected_sections() {
    let theme = DefaultTheme;
    let context = RenderContext::new(RenderMode::Plain, Some(80), &theme);

    let details = DetailSection::new("Session details")
        .with_field(LabeledField::new("ID", "$1"))
        .with_field(LabeledField::new("Name", "tinyverse_alpha"))
        .render(&context);

    let table = StyledTable::new(vec!["ID", "NAME", "WINDOWS"])
        .with_row(vec![
            "$1".to_owned(),
            "tinyverse_alpha".to_owned(),
            "1".to_owned(),
        ])
        .render(&context);

    let error = ErrorBlock::new("Failed to send command")
        .with_detail("tmux returned exit code 1")
        .with_guidance("Run tinyverse list --all and retry")
        .render(&context);

    assert!(details.contains("Session details"));
    assert!(table.contains("WINDOWS"));
    assert!(error.contains("Failed to send command"));
    assert!(error.contains("Next:"));
}
