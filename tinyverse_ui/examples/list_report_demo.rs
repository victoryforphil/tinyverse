use tinyverse_ui::example_outputs::{ExampleTheme, list_report_demo_output_with_theme};
use tinyverse_ui::render::RenderMode;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let force_plain = args.iter().any(|arg| arg == "--plain");
    let use_minimal_theme = args.iter().any(|arg| arg == "--minimal-theme");
    let mode = if force_plain {
        RenderMode::Plain
    } else {
        RenderMode::Ansi
    };

    let width = std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .or(Some(80));

    let theme = if use_minimal_theme {
        ExampleTheme::Minimal
    } else {
        ExampleTheme::Default
    };
    let output = list_report_demo_output_with_theme(mode, width, theme);

    println!("{output}");
}
