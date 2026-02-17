mod composer;
mod detail_modal;
mod messages;
mod parts;
mod popups;
mod types;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::Paragraph;

use crate::app::App;

use composer::render_chat_composer;
use detail_modal::render_chat_detail_modal;
use messages::render_chat_messages;
use popups::render_chat_popups;

const CHAT_COMPOSER_HEIGHT: u16 = 6;

pub(super) fn render_chat_tab(frame: &mut Frame, area: Rect, app: &mut App) {
    app.layout.chat = Default::default();
    app.layout.chat.root_rect = Some(area);

    if area.width < 24 || area.height < 8 {
        frame.render_widget(
            Paragraph::new("Chat panel is too small")
                .style(Style::default().fg(app.theme.text_muted)),
            area,
        );
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(CHAT_COMPOSER_HEIGHT)])
        .split(area);
    let messages_area = rows[0];
    let composer_area = rows[1];
    app.layout.chat.messages_rect = Some(messages_area);
    app.layout.chat.composer_rect = Some(composer_area);

    render_chat_messages(frame, messages_area, app);
    render_chat_composer(frame, composer_area, app);
    render_chat_popups(frame, area, composer_area, app);
    render_chat_detail_modal(frame, area, app);
}
