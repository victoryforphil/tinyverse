use ratatui::text::Line;

pub(super) struct RenderedChatLine {
    pub(super) line: Line<'static>,
    pub(super) toggle_key: Option<String>,
}
