use insta::assert_snapshot;
use tinyverse_ui::components::{StripeMode, StyledTable};
use tinyverse_ui::render::{RenderContext, RenderMode};
use tinyverse_ui::theme::DefaultTheme;

#[test]
fn styled_table_plain_snapshot() {
    let theme = DefaultTheme;
    let context = RenderContext::new(RenderMode::Plain, Some(48), &theme);
    let output = StyledTable::new(vec!["ID", "NAME", "STATUS", "WINDOWS"])
        .with_numeric_columns(&[3])
        .with_row(vec![
            "$1".to_owned(),
            "tinyverse_alpha".to_owned(),
            "attached".to_owned(),
            "12".to_owned(),
        ])
        .with_row(vec![
            "$2".to_owned(),
            "tinyverse_beta".to_owned(),
            "idle".to_owned(),
            "3".to_owned(),
        ])
        .render(&context);

    assert_snapshot!("styled_table_plain", output);
}

#[test]
fn styled_table_ansi_snapshot() {
    let theme = DefaultTheme;
    let context = RenderContext::new(RenderMode::Ansi, Some(48), &theme);
    let output = StyledTable::new(vec!["ID", "NAME", "STATUS", "WINDOWS"])
        .with_numeric_columns(&[3])
        .with_stripe_mode(StripeMode::DimEvenRows)
        .with_row(vec![
            "$1".to_owned(),
            "tinyverse_alpha".to_owned(),
            "attached".to_owned(),
            "12".to_owned(),
        ])
        .with_row(vec![
            "$2".to_owned(),
            "tinyverse_beta".to_owned(),
            "idle".to_owned(),
            "3".to_owned(),
        ])
        .render(&context);

    assert_snapshot!("styled_table_ansi", output);
}
