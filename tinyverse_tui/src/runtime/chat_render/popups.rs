use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use tinyverse_tui_components::{
    ListViewport, PopupAnchor, PopupItem, PopupOverlay, PopupOverlayProps,
};

use crate::app::App;
use crate::chat::ComposerAutocompleteMode;

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
    items: Vec<PopupItem>,
    selected: usize,
    query: Option<(String, String)>,
    hint: Option<String>,
    anchor: PopupAnchor,
    min_width: u16,
    max_width: u16,
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
        clear_popup_layout(app, PopupLayoutTarget::Model);
        return;
    }

    let raw_items = app.chat.model_selector_items();
    let mut items = Vec::with_capacity(raw_items.len());
    for label in raw_items {
        items.push(PopupItem {
            label,
            tag: None,
            active: false,
        });
    }
    let selected = app
        .chat
        .model_selector
        .selected
        .min(items.len().saturating_sub(1));

    let query = if app.chat.model_selector.raw_mode {
        (
            String::from("RAW"),
            app.chat.model_selector.raw_input.clone(),
        )
    } else {
        (
            String::from("FILTER"),
            app.chat.model_selector.query.clone(),
        )
    };

    render_popup(
        frame,
        parent,
        app,
        PopupConfig {
            target: PopupLayoutTarget::Model,
            title: "Model Picker",
            items,
            selected,
            query: Some(query),
            hint: None,
            anchor: PopupAnchor::At {
                x: app
                    .chat
                    .model_selector
                    .anchor_col
                    .unwrap_or_else(|| composer_area.x.saturating_add(2)),
                y: composer_area.y,
            },
            min_width: 34,
            max_width: 48,
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
        clear_popup_layout(app, PopupLayoutTarget::Agent);
        return;
    }

    let raw_items = app.chat.agent_selector_items();
    let mut items = Vec::with_capacity(raw_items.len());
    for label in raw_items {
        items.push(PopupItem {
            label,
            tag: None,
            active: false,
        });
    }
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
            hint: None,
            anchor: PopupAnchor::At {
                x: app
                    .chat
                    .agent_selector
                    .anchor_col
                    .unwrap_or_else(|| composer_area.x.saturating_add(18)),
                y: composer_area.y,
            },
            min_width: 28,
            max_width: 44,
        },
    );
}

fn render_autocomplete_popup(frame: &mut Frame, parent: Rect, composer_area: Rect, app: &mut App) {
    if !app.chat.is_autocomplete_open()
        || app.chat.is_model_selector_open()
        || app.chat.is_agent_selector_open()
    {
        clear_popup_layout(app, PopupLayoutTarget::Autocomplete);
        return;
    }

    let ac_items = &app.chat.autocomplete.items;
    let mut items = Vec::with_capacity(ac_items.len());
    for item in ac_items {
        items.push(PopupItem {
            label: item.label.clone(),
            tag: Some(item.tag.clone()),
            active: false,
        });
    }
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
            anchor: PopupAnchor::At {
                x: composer_area.x.saturating_add(col as u16),
                y: composer_area.y,
            },
            min_width: 32,
            max_width: 52,
        },
    );
}

fn render_popup(frame: &mut Frame, parent: Rect, app: &mut App, config: PopupConfig) {
    let selected = config.selected.min(config.items.len().saturating_sub(1));
    let (query_text, query_label) = match config.query {
        Some((label, value)) => (Some(value), Some(label)),
        None => (None, None),
    };
    let has_query = query_text.is_some();
    let has_hint = config.hint.is_some();

    let props = PopupOverlayProps {
        title: config.title.to_owned(),
        items: config.items,
        selected,
        query: query_text,
        query_label,
        hint: config.hint,
        anchor: config.anchor,
        max_visible: CHAT_POPUP_MAX_VISIBLE,
        min_width: config.min_width,
        max_width: config.max_width,
    };

    let Some(popup) = PopupOverlay::area(parent, &props) else {
        clear_popup_layout(app, config.target);
        return;
    };

    set_popup_rect(app, config.target, popup);
    let sections = popup_sections(popup, has_query, has_hint);
    set_popup_list_rect(app, config.target, sections.list);
    set_popup_query_rect(app, config.target, sections.query);

    let viewport = ListViewport::new(props.items.len(), sections.list.height as usize, selected);
    set_popup_list_start(app, config.target, viewport.start);

    PopupOverlay::render(frame, parent, &props, &app.theme);
}

#[derive(Clone, Copy)]
struct PopupSections {
    list: Rect,
    query: Option<Rect>,
}

fn popup_sections(popup: Rect, has_query: bool, has_hint: bool) -> PopupSections {
    let inner = Rect {
        x: popup.x.saturating_add(1),
        y: popup.y.saturating_add(1),
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };

    if inner.width == 0 || inner.height == 0 {
        return PopupSections {
            list: inner,
            query: None,
        };
    }

    let mut constraints = vec![Constraint::Min(1)];
    if has_query {
        constraints.push(Constraint::Length(1));
    }
    if has_hint {
        constraints.push(Constraint::Length(1));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    PopupSections {
        list: chunks[0],
        query: has_query.then(|| chunks[1]),
    }
}

fn clear_popup_layout(app: &mut App, target: PopupLayoutTarget) {
    match target {
        PopupLayoutTarget::Model => {
            app.layout.chat.model_selector_rect = None;
            app.layout.chat.model_selector_list_rect = None;
            app.layout.chat.model_selector_query_rect = None;
            app.layout.chat.model_selector_list_start = 0;
        }
        PopupLayoutTarget::Agent => {
            app.layout.chat.agent_selector_rect = None;
            app.layout.chat.agent_selector_list_rect = None;
            app.layout.chat.agent_selector_query_rect = None;
            app.layout.chat.agent_selector_list_start = 0;
        }
        PopupLayoutTarget::Autocomplete => {
            app.layout.chat.autocomplete_rect = None;
            app.layout.chat.autocomplete_list_rect = None;
            app.layout.chat.autocomplete_list_start = 0;
        }
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

fn set_popup_query_rect(app: &mut App, target: PopupLayoutTarget, rect: Option<Rect>) {
    match target {
        PopupLayoutTarget::Model => app.layout.chat.model_selector_query_rect = rect,
        PopupLayoutTarget::Agent => app.layout.chat.agent_selector_query_rect = rect,
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
