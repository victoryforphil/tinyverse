use tinyverse_ui::example_outputs::{ExampleTheme, primitive_gallery_output_with_theme};
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

    let theme = if use_minimal_theme {
        ExampleTheme::Minimal
    } else {
        ExampleTheme::Default
    };

    let output = primitive_gallery_output_with_theme(mode, Some(80), theme);
    println!("{output}");
}
