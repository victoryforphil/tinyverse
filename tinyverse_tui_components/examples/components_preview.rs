use ratatui::text::Line;
use tinyverse_tui_components::{
    ComponentTheme, KeyBind, KeyHintBar, LabeledField, ModalOverlayProps, SectionHeader,
    StatusPill, Tone,
};

fn main() {
    let theme = ComponentTheme::default();

    let hints = [
        KeyBind::new("q", "Quit"),
        KeyBind::new("r", "Refresh"),
        KeyBind::new("v", "View"),
    ];
    let lines = KeyHintBar::new(&hints).lines_wrapped(30, &theme);

    let label = LabeledField::new("Branch", "main").line_compact(&theme);
    let header = SectionHeader::new("Identity", theme.pill_info_fg).line(28, &theme);
    let pill = StatusPill::ok("clean", &theme).span();
    let toned = StatusPill::for_tone("ready", Tone::Accent, &theme).span_compact();
    let custom = StatusPill::custom("warn", theme.pill_warn_fg, theme.pill_warn_bg, true).span();
    let inline_hint = KeyBind::new("esc", "close").spans(&theme);
    let modal_props = ModalOverlayProps {
        title: String::from("Detail"),
        header_lines: vec![Line::from("Header")],
        body_lines: vec![Line::from("Body")],
        hint_line: Some(Line::from("esc close")),
        width: 40,
        height: 12,
        scroll_lines: 0,
    };

    println!("{}", header);
    println!("{}", label);
    println!("{}", pill.content);
    println!("{}", toned.content);
    println!("{}", custom.content);
    println!("inline hint spans: {}", inline_hint.len());
    println!("modal title: {}", modal_props.title);
    println!("wrapped hint lines: {}", lines.len());
}
