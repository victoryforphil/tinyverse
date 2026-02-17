use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::app::App;
use crate::chat::ComposerAutocompleteMode;

use crate::runtime::helpers::{inset_rect, truncate_to};

const CHAT_POPUP_MAX_VISIBLE: usize = 8;

#[derive(Clone, Copy)]
enum PopupLayoutTarget {
    Model,
    Agent,
    Autocomplete,
}

struct PopupConfig {
    target: PopupLayoutTarget,
    title: &'static str,
    items: Vec<String>,
    selected: usize,
    query: Option<(String, String)>,
    hint: Option<String>,
    anchor_x: u16,
    anchor_y: u16,
    min_width: u16,
    with_query: bool,
}

pub(super) fn render_chat_popups(
    frame: &mut Frame,
    parent: Rect,
    composer_area: Rect,
    app: &mut App,
) {
    render_model_selector_popup(frame, parent, composer_area, app);
    render_agent_selector_popup(frame, parent, composer_area, app);
    render_autocomplete_popup(frame, parent, composer_area, app);
}

fn render_model_selector_popup(
    frame: &mut Frame,
    parent: Rect,
    composer_area: Rect,
    app: &mut App,
) {
    if !app.chat.is_model_selector_open() {
        app.layout.chat.model_selector_rect = None;
        app.layout.chat.model_selector_list_rect = None;
        app.layout.chat.model_selector_query_rect = None;
        app.layout.chat.model_selector_list_start = 0;
        return;
    }

    let items = app.chat.model_selector_items();
    let selected = app
        .chat
        .model_selector
        .selected
        .min(items.len().saturating_sub(1));

    render_popup(
        frame,
        parent,
        app,
        PopupConfig {
            target: PopupLayoutTarget::Model,
            title: "Model Picker",
            items,
            selected,
            query: Some((
                if app.chat.model_selector.raw_mode {
                    String::from("RAW")
                } else {
                    String::from("FILTER")
                },
                if app.chat.model_selector.raw_mode {
                    app.chat.model_selector.raw_input.clone()
                } else {
                    app.chat.model_selector.query.clone()
                },
            )),
            hint: Some(String::from("enter select  tab raw  esc close")),
            anchor_x: app
                .chat
                .model_selector
                .anchor_col
                .unwrap_or_else(|| composer_area.x.saturating_add(2)),
            anchor_y: composer_area.y,
            min_width: 34,
            with_query: true,
        },
    );
}

fn render_agent_selector_popup(
    frame: &mut Frame,
    parent: Rect,
    composer_area: Rect,
    app: &mut App,
) {
    if !app.chat.is_agent_selector_open() {
        app.layout.chat.agent_selector_rect = None;
        app.layout.chat.agent_selector_list_rect = None;
        app.layout.chat.agent_selector_query_rect = None;
        app.layout.chat.agent_selector_list_start = 0;
        return;
    }

    let items = app.chat.agent_selector_items();
    let selected = app
        .chat
        .agent_selector
        .selected
        .min(items.len().saturating_sub(1));

    render_popup(
        frame,
        parent,
        app,
        PopupConfig {
            target: PopupLayoutTarget::Agent,
            title: "Agent Picker",
            items,
            selected,
            query: Some((
                String::from("FILTER"),
                app.chat.agent_selector.query.clone(),
            )),
            hint: Some(String::from("enter select  esc close")),
            anchor_x: app
                .chat
                .agent_selector
                .anchor_col
                .unwrap_or_else(|| composer_area.x.saturating_add(18)),
            anchor_y: composer_area.y,
            min_width: 28,
            with_query: true,
        },
    );
}

fn render_autocomplete_popup(frame: &mut Frame, parent: Rect, composer_area: Rect, app: &mut App) {
    if !app.chat.is_autocomplete_open()
        || app.chat.is_model_selector_open()
        || app.chat.is_agent_selector_open()
    {
        app.layout.chat.autocomplete_rect = None;
        app.layout.chat.autocomplete_list_rect = None;
        app.layout.chat.autocomplete_list_start = 0;
        return;
    }

    let items = app
        .chat
        .autocomplete
        .items
        .iter()
        .map(|item| format!("{} [{}]", item.label, item.tag))
        .collect::<Vec<_>>();
    let selected = app
        .chat
        .autocomplete
        .selected
        .min(items.len().saturating_sub(1));
    let (_, col) = app.chat.autocomplete_anchor_position().unwrap_or((0, 0));
    let title = match app.chat.autocomplete_mode() {
        Some(ComposerAutocompleteMode::Slash) => "Commands",
        Some(ComposerAutocompleteMode::File) => "Context",
        None => "Autocomplete",
    };

    render_popup(
        frame,
        parent,
        app,
        PopupConfig {
            target: PopupLayoutTarget::Autocomplete,
            title,
            items,
            selected,
            query: None,
            hint: None,
            anchor_x: composer_area.x.saturating_add(col as u16),
            anchor_y: composer_area.y,
            min_width: 32,
            with_query: false,
        },
    );
}

fn render_popup(frame: &mut Frame, parent: Rect, app: &mut App, config: PopupConfig) {
    let popup = popup_rect(
        parent,
        config.anchor_x,
        config.anchor_y,
        config.min_width,
        config.items.len().max(1),
        config.with_query,
    );
    set_popup_rect(app, config.target, popup);

    frame.render_widget(Clear, popup);
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_focused_border));
    let inner = inset_rect(block.inner(popup), 1, 0);
    frame.render_widget(block, popup);

    let mut constraints = vec![Constraint::Min(1)];
    if config.query.is_some() {
        constraints.push(Constraint::Length(1));
    }
    if config.hint.is_some() {
        constraints.push(Constraint::Length(1));
    }
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);
    let list_area = sections[0];

    set_popup_list_rect(app, config.target, list_area);

    let viewport = list_viewport(
        config.items.len(),
        list_area.height as usize,
        config.selected,
    );
    set_popup_list_start(app, config.target, viewport.0);

    let mut lines = Vec::new();
    for (row, item) in config.items[viewport.0..viewport.1].iter().enumerate() {
        let index = viewport.0 + row;
        let is_selected = index == config.selected;
        lines.push(Line::from(vec![
            Span::styled(
                if is_selected { "▸ " } else { "  " },
                Style::default().fg(app.theme.text_muted),
            ),
            Span::styled(
                truncate_to(item, list_area.width.saturating_sub(3) as usize),
                if is_selected {
                    Style::default()
                        .fg(app.theme.pill_accent_fg)
                        .bg(app.theme.selected_card_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text_secondary)
                },
            ),
        ]));
    }
    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No matching options.",
            Style::default().fg(app.theme.text_muted),
        )));
    }
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), list_area);

    let mut section_index = 1usize;
    if let Some((label, value)) = config.query {
        if section_index < sections.len() {
            let query_area = sections[section_index];
            set_popup_query_rect(app, config.target, query_area);
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        format!(" {label} "),
                        Style::default()
                            .fg(app.theme.pill_accent_fg)
                            .bg(app.theme.pill_accent_bg),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("{value}_"),
                        Style::default().fg(app.theme.text_secondary),
                    ),
                ])),
                query_area,
            );
        }
        section_index = section_index.saturating_add(1);
    }

    if let Some(hint_text) = config.hint
        && section_index < sections.len()
    {
        frame.render_widget(
            Paragraph::new(hint_text).style(Style::default().fg(app.theme.text_muted)),
            sections[section_index],
        );
    }
}

fn popup_rect(
    parent: Rect,
    anchor_x: u16,
    anchor_y: u16,
    min_width: u16,
    item_len: usize,
    with_query: bool,
) -> Rect {
    let width = min_width.min(parent.width.saturating_sub(2)).max(22);
    let query_rows = u16::from(with_query);
    let list_rows = item_len.min(CHAT_POPUP_MAX_VISIBLE).max(1) as u16;
    let hint_rows = 1u16;
    let height = (2 + list_rows + query_rows + hint_rows)
        .min(parent.height.saturating_sub(1))
        .max(4);

    let x = anchor_x
        .saturating_sub(width / 2)
        .clamp(parent.x, parent.right().saturating_sub(width));
    let y = anchor_y
        .saturating_sub(height.saturating_sub(1))
        .clamp(parent.y, parent.bottom().saturating_sub(height));
    Rect {
        x,
        y,
        width,
        height,
    }
}

fn set_popup_rect(app: &mut App, target: PopupLayoutTarget, rect: Rect) {
    match target {
        PopupLayoutTarget::Model => app.layout.chat.model_selector_rect = Some(rect),
        PopupLayoutTarget::Agent => app.layout.chat.agent_selector_rect = Some(rect),
        PopupLayoutTarget::Autocomplete => app.layout.chat.autocomplete_rect = Some(rect),
    }
}

fn set_popup_list_rect(app: &mut App, target: PopupLayoutTarget, rect: Rect) {
    match target {
        PopupLayoutTarget::Model => app.layout.chat.model_selector_list_rect = Some(rect),
        PopupLayoutTarget::Agent => app.layout.chat.agent_selector_list_rect = Some(rect),
        PopupLayoutTarget::Autocomplete => app.layout.chat.autocomplete_list_rect = Some(rect),
    }
}

fn set_popup_query_rect(app: &mut App, target: PopupLayoutTarget, rect: Rect) {
    match target {
        PopupLayoutTarget::Model => app.layout.chat.model_selector_query_rect = Some(rect),
        PopupLayoutTarget::Agent => app.layout.chat.agent_selector_query_rect = Some(rect),
        PopupLayoutTarget::Autocomplete => {}
    }
}

fn set_popup_list_start(app: &mut App, target: PopupLayoutTarget, start: usize) {
    match target {
        PopupLayoutTarget::Model => app.layout.chat.model_selector_list_start = start,
        PopupLayoutTarget::Agent => app.layout.chat.agent_selector_list_start = start,
        PopupLayoutTarget::Autocomplete => app.layout.chat.autocomplete_list_start = start,
    }
}

fn list_viewport(total: usize, visible: usize, selected: usize) -> (usize, usize) {
    if total == 0 || visible == 0 {
        return (0, 0);
    }
    if total <= visible {
        return (0, total);
    }

    let max_start = total - visible;
    let mut start = selected.saturating_sub(visible / 2);
    if start > max_start {
        start = max_start;
    }
    (start, start + visible)
}
