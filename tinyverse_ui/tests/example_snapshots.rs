use insta::assert_snapshot;
use tinyverse_ui::example_outputs::{list_report_demo_output, primitive_gallery_output};
use tinyverse_ui::render::RenderMode;

#[test]
fn primitive_gallery_plain_snapshot() {
    let output = primitive_gallery_output(RenderMode::Plain, Some(80));
    assert_snapshot!("primitive_gallery_plain", output);
}

#[test]
fn primitive_gallery_ansi_snapshot() {
    let output = primitive_gallery_output(RenderMode::Ansi, Some(80));
    assert_snapshot!("primitive_gallery_ansi", output);
}

#[test]
fn list_report_demo_plain_snapshot() {
    let output = list_report_demo_output(RenderMode::Plain, Some(80));
    assert_snapshot!("list_report_demo_plain", output);
}

#[test]
fn list_report_demo_ansi_snapshot() {
    let output = list_report_demo_output(RenderMode::Ansi, Some(80));
    assert_snapshot!("list_report_demo_ansi", output);
}
