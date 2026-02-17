use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};

use crate::app::{
    ACTION_MENU_DANGER_SPLIT_AFTER, App, AppMode, FooterHotkeyAction, MENU_ACTIONS, MenuAction,
    SidebarTab,
};
use crate::chat::{ChatMessageRole, ComposerAutocompleteMode};

use super::helpers::{
    anchored_rect, centered_rect, inset_rect, key_hint, line_kv, status_pill, styled_panel,
    tag_pill, truncate_to,
};

const CARD_WIDTH: u16 = 36;
const CARD_HEIGHT: u16 = 8;
const CARD_GAP_X: u16 = 2;
const CARD_GAP_Y: u16 = 1;
const CHAT_COMPOSER_HEIGHT: u16 = 6;
const CHAT_POPUP_MAX_VISIBLE: usize = 8;

pub(crate) fn render_frame(frame: &mut Frame, app: &mut App) {
    let root = frame.area();
    app.layout = Default::default();

    frame.render_widget(Clear, root);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(root);

    render_header(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
    app.layout.footer_rect = Some(chunks[2]);
    render_footer(frame, chunks[2], app);

    match app.mode {
        AppMode::ActionMenu => render_action_menu(frame, root, app),
        AppMode::ConfirmKill => render_kill_confirmation(frame, root, app),
        AppMode::ConfirmKillAll => render_kill_all_confirmation(frame, root, app),
        AppMode::SendInput => render_input_overlay(frame, root, app, "Send to console"),
        AppMode::SpawnInput => render_input_overlay(frame, root, app, "Spawn session"),
        AppMode::Normal => {}
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = Line::from(vec![
        Span::styled(
            "tinyverse tui",
            Style::default()
                .fg(app.theme.pane_focused_border)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled(
            format!("{} session(s)", app.sessions.len()),
            Style::default().fg(app.theme.text_secondary),
        ),
    ]);

    frame.render_widget(Paragraph::new(title), area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &mut App) {
    app.layout.body_rect = Some(area);

    let main_chunks = if app.inspector_visible && area.height >= 14 {
        let inspector_height = app
            .inspector_height
            .clamp(6, area.height.saturating_sub(6).max(6));
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(inspector_height)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(0)])
            .split(area)
    };

    let top_area = main_chunks[0];
    app.layout.divider_y = if app.inspector_visible && main_chunks[1].height > 0 {
        Some(top_area.bottom())
    } else {
        None
    };

    if top_area.width >= 90 {
        let left_pct = app.inspector_ratio.clamp(40, 80);
        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(left_pct),
                Constraint::Percentage(100 - left_pct),
            ])
            .split(top_area);
        app.layout.divider_x = Some(split[0].right());
        render_cards(frame, split[0], app);
        render_sidebar(frame, split[1], app);
    } else {
        app.layout.divider_x = None;
        app.layout.sidebar_tab_rects.clear();
        app.layout.sidebar_preview_rect = None;
        render_cards(frame, top_area, app);
    }

    if app.inspector_visible && main_chunks[1].height > 0 {
        render_bottom_inspector(frame, main_chunks[1], app);
    }
}

fn render_cards(frame: &mut Frame, area: Rect, app: &mut App) {
    let panel = styled_panel("Sessions", true, &app.theme);
    let inner = inset_rect(panel.inner(area), 2, 1);
    frame.render_widget(panel, area);

    if app.sessions.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from("No sessions found."),
            Line::from("Run `tinyverse spawn <name>` to create one."),
        ])
        .style(Style::default().fg(app.theme.text_secondary))
        .wrap(Wrap { trim: true });

        let popup = centered_rect(58, 5, inner);
        frame.render_widget(
            Block::default()
                .title(" TinyVerse ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.pane_unfocused_border)),
            popup,
        );
        frame.render_widget(empty, inset_rect(popup, 1, 1));
        return;
    }

    let stride_x = CARD_WIDTH + CARD_GAP_X;
    let stride_y = CARD_HEIGHT + CARD_GAP_Y;
    let cols = ((inner.width + CARD_GAP_X) / stride_x).max(1);
    let cols_usize = cols as usize;
    let visible_rows = ((inner.height + CARD_GAP_Y) / stride_y).max(1) as usize;
    let selected_row = app.selected_index / cols_usize;

    if selected_row < app.scroll_row {
        app.scroll_row = selected_row;
    }
    if selected_row >= app.scroll_row + visible_rows {
        app.scroll_row = selected_row + 1 - visible_rows;
    }

    let start_index = app.scroll_row * cols_usize;
    let max_visible_cards = visible_rows * cols_usize;

    app.layout.card_rects.clear();
    app.layout.card_kill_rects.clear();

    for (view_index, (session_index, session)) in app
        .sessions
        .iter()
        .enumerate()
        .skip(start_index)
        .take(max_visible_cards)
        .enumerate()
    {
        let row = (view_index as u16) / cols;
        let col = (view_index as u16) % cols;
        let x = inner.x + col * stride_x;
        let y = inner.y + row * stride_y;

        if y + CARD_HEIGHT > inner.bottom() {
            break;
        }

        let width = CARD_WIDTH.min(inner.right().saturating_sub(x));
        if width < 20 {
            continue;
        }

        let card_area = Rect {
            x,
            y,
            width,
            height: CARD_HEIGHT,
        };
        app.layout.card_rects.push((session_index, card_area));

        let is_selected = session_index == app.selected_index;
        let border_color = if is_selected {
            app.theme.pane_focused_border
        } else {
            app.theme.pane_unfocused_border
        };
        let title_style = if is_selected {
            Style::default()
                .fg(app.theme.pane_focused_border)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.text_secondary)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Line::from(vec![Span::styled(
                format!(" {} ", truncate_to(&session.session_name, 24)),
                title_style,
            )]));

        let inner = block.inner(card_area);
        frame.render_widget(block, card_area);

        if card_area.width >= 10 {
            let kill_rect = Rect {
                x: card_area.right().saturating_sub(4),
                y: card_area.y,
                width: 3,
                height: 1,
            };
            app.layout.card_kill_rects.push((session_index, kill_rect));

            let kill_style = if is_selected {
                Style::default()
                    .fg(app.theme.pill_err_fg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text_muted)
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::raw(" "),
                    Span::styled("X", kill_style),
                    Span::raw(" "),
                ])),
                kill_rect,
            );
        }

        let card_inner = inset_rect(inner, 1, 0);

        let body = Paragraph::new(vec![
            line_kv("key", &truncate_to(&session.session_key, 24), &app.theme),
            Line::from(vec![
                status_pill(&session.status_string, &app.theme),
                Span::raw(" "),
                tag_pill(&truncate_to(&session.agent_type, 14), &app.theme),
            ]),
            line_kv(
                "tmux",
                &truncate_to(&session.tmux_session_name, 20),
                &app.theme,
            ),
        ])
        .style(if is_selected {
            Style::default().bg(app.theme.selected_card_bg)
        } else {
            Style::default()
        })
        .wrap(Wrap { trim: true });

        frame.render_widget(body, card_inner);
    }
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = styled_panel(app.sidebar_tab.title(), true, &app.theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(3)])
        .split(inner);

    app.layout.sidebar_preview_rect = Some(sidebar_chunks[1]);
    render_sidebar_tabs(frame, app, sidebar_chunks[0]);

    if app.sidebar_tab == SidebarTab::Chat {
        render_chat_tab(frame, sidebar_chunks[1], app);
        return;
    }

    let Some(session) = app.selected_session().cloned() else {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No session selected",
                Style::default().fg(app.theme.text_secondary),
            ))),
            sidebar_chunks[1],
        );
        return;
    };

    let preview = app.pane_preview_cache.get(&session.session_key).cloned();

    let console_preview = preview
        .as_ref()
        .map(|value| {
            if value.console.trim().is_empty() {
                String::from("(console pane is empty)")
            } else {
                value.console.clone()
            }
        })
        .unwrap_or_else(|| String::from("Loading console preview..."));

    let agent_preview = preview
        .as_ref()
        .map(|value| {
            if value.agent.trim().is_empty() {
                String::from("(agent pane is empty)")
            } else {
                value.agent.clone()
            }
        })
        .unwrap_or_else(|| String::from("Loading agent preview..."));

    match app.sidebar_tab {
        SidebarTab::Console => {
            let fitted = fit_preview_text(&console_preview, sidebar_chunks[1]);
            frame.render_widget(
                Paragraph::new(fitted).style(Style::default().fg(app.theme.text_secondary)),
                sidebar_chunks[1],
            );
        }
        SidebarTab::Agent => {
            let fitted = fit_preview_text(&agent_preview, sidebar_chunks[1]);
            frame.render_widget(
                Paragraph::new(fitted).style(Style::default().fg(app.theme.text_secondary)),
                sidebar_chunks[1],
            );
        }
        SidebarTab::Chat => {}
    }
}

fn render_bottom_inspector(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_panel("Inspector", true, &app.theme);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(session) = app.selected_session() else {
        frame.render_widget(
            Paragraph::new("No session selected")
                .style(Style::default().fg(app.theme.text_secondary))
                .wrap(Wrap { trim: true }),
            inner,
        );
        return;
    };

    let left_rows = vec![
        Row::new(vec![
            Cell::from("Name"),
            Cell::from(session.session_name.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Key"),
            Cell::from(session.session_key.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Agent"),
            Cell::from(session.agent_type.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Status"),
            Cell::from(session.status_string.as_str()).style(
                Style::default()
                    .fg(status_color(&session.status_string, app))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let right_rows = vec![
        Row::new(vec![
            Cell::from("Tmux"),
            Cell::from(session.tmux_session_name.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Console Pane"),
            Cell::from(session.console_pane_id.as_deref().unwrap_or("-")),
        ]),
        Row::new(vec![
            Cell::from("Agent Pane"),
            Cell::from(session.agent_pane_id.as_deref().unwrap_or("-")),
        ]),
        Row::new(vec![
            Cell::from("Description"),
            Cell::from(session.description.as_deref().unwrap_or("(none)")),
        ]),
    ];

    if inner.width >= 92 {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);
        let left_table = Table::new(left_rows, [Constraint::Length(12), Constraint::Min(10)])
            .column_spacing(1)
            .style(Style::default().fg(app.theme.text_secondary));
        let right_table = Table::new(right_rows, [Constraint::Length(14), Constraint::Min(10)])
            .column_spacing(1)
            .style(Style::default().fg(app.theme.text_secondary));
        frame.render_widget(left_table, columns[0]);
        frame.render_widget(right_table, columns[1]);
    } else {
        let rows = left_rows.into_iter().chain(right_rows).collect::<Vec<_>>();
        let table = Table::new(rows, [Constraint::Length(14), Constraint::Min(10)])
            .column_spacing(1)
            .style(Style::default().fg(app.theme.text_secondary));
        frame.render_widget(table, inner);
    }
}

fn render_sidebar_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    app.layout.sidebar_tab_rects.clear();
    let mut spans = Vec::new();
    let mut cursor_x = area.x;
    for (index, tab) in SidebarTab::all().iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(" ", Style::default().fg(app.theme.text_muted)));
            cursor_x = cursor_x.saturating_add(1);
        }
        let selected = *tab == app.sidebar_tab;
        let number_style = if selected {
            Style::default()
                .fg(app.theme.key_hint_key_fg)
                .bg(app.theme.key_hint_key_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(app.theme.pill_accent_fg)
                .bg(app.theme.pill_muted_bg)
        };
        let label_style = if selected {
            Style::default()
                .fg(app.theme.text_primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.text_secondary)
        };
        let short = match tab {
            SidebarTab::Console => "Cons",
            SidebarTab::Agent => "Agent",
            SidebarTab::Chat => "Chat",
        };
        let width = (short.chars().count() + 5) as u16;
        app.layout.sidebar_tab_rects.push((
            *tab,
            Rect {
                x: cursor_x,
                y: area.y,
                width,
                height: 1,
            },
        ));
        cursor_x = cursor_x.saturating_add(width);
        spans.push(Span::styled(
            format!(" {} ", tab.hotkey_index()),
            number_style,
        ));
        spans.push(Span::styled(format!(" {} ", short), label_style));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_chat_tab(frame: &mut Frame, area: Rect, app: &mut App) {
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
    render_model_selector_popup(frame, area, composer_area, app);
    render_agent_selector_popup(frame, area, composer_area, app);
    render_autocomplete_popup(frame, area, composer_area, app);
}

fn render_chat_messages(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Messages ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_unfocused_border));
    let inner = inset_rect(block.inner(area), 1, 0);
    frame.render_widget(block, area);

    let max_lines = inner.height as usize;
    let mut lines = Vec::new();
    for message in app.chat.messages.iter() {
        let role_color = match message.role {
            ChatMessageRole::System => app.theme.pill_warn_fg,
            ChatMessageRole::User => app.theme.pill_info_fg,
            ChatMessageRole::Assistant => app.theme.pill_accent_fg,
        };
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {} ", message.role.label()),
                Style::default().fg(role_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {}", message.created_at),
                Style::default().fg(app.theme.text_muted),
            ),
        ]));
        for text_line in message.text.lines() {
            lines.push(Line::from(Span::styled(
                text_line.to_owned(),
                Style::default().fg(app.theme.text_secondary),
            )));
        }
        lines.push(Line::from(""));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No messages yet. Press c to compose.",
            Style::default().fg(app.theme.text_muted),
        )));
    }

    let overflow = lines.len().saturating_sub(max_lines);
    let scroll = app.chat.scroll_lines as usize;
    let start = overflow.saturating_sub(scroll);
    let visible = lines
        .into_iter()
        .skip(start)
        .take(max_lines)
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(visible).wrap(Wrap { trim: false }), inner);
}

fn render_chat_composer(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Composer ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.chat.composing {
            app.theme.pane_focused_border
        } else {
            app.theme.pane_unfocused_border
        }));
    let inner = inset_rect(block.inner(area), 1, 0);
    frame.render_widget(block, area);
    if inner.width < 4 || inner.height < 2 {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    let model_text = format!("model:{}", truncate_to(&app.chat.active_model, 28));
    let agent_text = format!("agent:{}", truncate_to(&app.chat.active_agent, 22));
    let bridge = app.chat_bridge.status();
    let source_text = format!("source:{}", bridge.mode.label());
    let model_width = model_text.chars().count() as u16 + 2;
    let agent_width = agent_text.chars().count() as u16 + 2;
    let model_rect = Rect {
        x: rows[0].x,
        y: rows[0].y,
        width: model_width.min(rows[0].width),
        height: 1,
    };
    let agent_x = model_rect
        .x
        .saturating_add(model_rect.width)
        .saturating_add(2);
    let agent_rect = Rect {
        x: agent_x,
        y: rows[0].y,
        width: agent_width.min(rows[0].right().saturating_sub(agent_x)),
        height: 1,
    };
    app.layout.chat.model_chip_rect = Some(model_rect);
    app.layout.chat.agent_chip_rect = Some(agent_rect);
    app.layout.chat.composer_input_rect = Some(rows[1]);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!(" {model_text} "),
                Style::default()
                    .fg(app.theme.pill_accent_fg)
                    .bg(app.theme.pill_accent_bg),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" {agent_text} "),
                Style::default()
                    .fg(app.theme.pill_info_fg)
                    .bg(app.theme.pill_info_bg),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" {source_text} "),
                Style::default()
                    .fg(app.theme.pill_warn_fg)
                    .bg(app.theme.pill_warn_bg),
            ),
        ])),
        rows[0],
    );

    let composer_line = if app.chat.composing {
        app.chat.draft_with_cursor()
    } else {
        String::from("Press c to compose, enter to send")
    };
    frame.render_widget(
        Paragraph::new(composer_line).style(Style::default().fg(if app.chat.composing {
            app.theme.text_primary
        } else {
            app.theme.text_muted
        })),
        rows[1],
    );
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
    let popup = popup_rect(
        parent,
        app.chat
            .model_selector
            .anchor_col
            .unwrap_or_else(|| composer_area.x.saturating_add(2)),
        composer_area.y,
        34,
        items.len().max(1),
        true,
    );
    app.layout.chat.model_selector_rect = Some(popup);

    render_popup(
        frame,
        popup,
        "Model Picker",
        &items,
        selected,
        Some((
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
        Some(String::from("enter select  tab raw  esc close")),
        app,
        true,
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
    let popup = popup_rect(
        parent,
        app.chat
            .agent_selector
            .anchor_col
            .unwrap_or_else(|| composer_area.x.saturating_add(18)),
        composer_area.y,
        28,
        items.len().max(1),
        true,
    );
    app.layout.chat.agent_selector_rect = Some(popup);

    render_popup(
        frame,
        popup,
        "Agent Picker",
        &items,
        selected,
        Some((
            String::from("FILTER"),
            app.chat.agent_selector.query.clone(),
        )),
        Some(String::from("enter select  esc close")),
        app,
        false,
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
    let popup = popup_rect(
        parent,
        composer_area.x.saturating_add(col as u16),
        composer_area.y,
        32,
        items.len().max(1),
        false,
    );
    app.layout.chat.autocomplete_rect = Some(popup);

    let title = match app.chat.autocomplete_mode() {
        Some(ComposerAutocompleteMode::Slash) => "Commands",
        Some(ComposerAutocompleteMode::File) => "Context",
        None => "Autocomplete",
    };

    render_popup(
        frame, popup, title, &items, selected, None, None, app, false,
    );
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

fn render_popup(
    frame: &mut Frame,
    popup: Rect,
    title: &str,
    items: &[String],
    selected: usize,
    query: Option<(String, String)>,
    hint: Option<String>,
    app: &mut App,
    is_model: bool,
) {
    frame.render_widget(Clear, popup);
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_focused_border));
    let inner = inset_rect(block.inner(popup), 1, 0);
    frame.render_widget(block, popup);

    let mut constraints = vec![Constraint::Min(1)];
    if query.is_some() {
        constraints.push(Constraint::Length(1));
    }
    if hint.is_some() {
        constraints.push(Constraint::Length(1));
    }
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);
    let list_area = sections[0];

    if is_model {
        app.layout.chat.model_selector_list_rect = Some(list_area);
    } else if title == "Agent Picker" {
        app.layout.chat.agent_selector_list_rect = Some(list_area);
    } else {
        app.layout.chat.autocomplete_list_rect = Some(list_area);
    }

    let viewport = list_viewport(items.len(), list_area.height as usize, selected);
    if is_model {
        app.layout.chat.model_selector_list_start = viewport.0;
    } else if title == "Agent Picker" {
        app.layout.chat.agent_selector_list_start = viewport.0;
    } else {
        app.layout.chat.autocomplete_list_start = viewport.0;
    }

    let mut lines = Vec::new();
    for (row, item) in items[viewport.0..viewport.1].iter().enumerate() {
        let index = viewport.0 + row;
        let is_selected = index == selected;
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
    if let Some((label, value)) = query {
        if section_index < sections.len() {
            let query_area = sections[section_index];
            if is_model {
                app.layout.chat.model_selector_query_rect = Some(query_area);
            } else {
                app.layout.chat.agent_selector_query_rect = Some(query_area);
            }
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

    if let Some(hint_text) = hint {
        if section_index < sections.len() {
            frame.render_widget(
                Paragraph::new(hint_text).style(Style::default().fg(app.theme.text_muted)),
                sections[section_index],
            );
        }
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

fn fit_preview_text(text: &str, area: Rect) -> String {
    let max_width = area.width.saturating_sub(1) as usize;
    let max_lines = area.height as usize;
    if max_width == 0 || max_lines == 0 {
        return String::new();
    }

    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(max_lines);
    lines[start..]
        .iter()
        .map(|line| line.chars().take(max_width).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let refresh = app
        .last_refresh_at
        .map(|time| format!("refreshed {}s ago", time.elapsed().as_secs()))
        .unwrap_or_else(|| String::from("never refreshed"));

    let mut mode_hints: Vec<Span<'static>> = Vec::new();
    let actions = footer_actions_for_mode(app.mode);
    for (index, action) in actions.iter().enumerate() {
        if index > 0 {
            mode_hints.push(Span::styled(
                " | ",
                Style::default().fg(app.theme.key_hint_bracket_fg),
            ));
        }
        let hovered = app.footer_hover_action == Some(*action);
        mode_hints.extend(footer_hint(*action, hovered, app));
    }

    mode_hints.push(Span::styled(
        " | ",
        Style::default().fg(app.theme.key_hint_bracket_fg),
    ));
    mode_hints.push(Span::styled(
        footer_status_message(app),
        Style::default().fg(app.theme.text_primary),
    ));
    mode_hints.push(Span::styled(
        " | ",
        Style::default().fg(app.theme.key_hint_bracket_fg),
    ));
    mode_hints.push(Span::styled(
        refresh,
        Style::default().fg(app.theme.text_muted),
    ));

    let footer = Line::from(mode_hints);

    frame.render_widget(Paragraph::new(footer), area);
}

fn footer_status_message(app: &App) -> String {
    if let Some(last_assistant) = app
        .chat
        .messages
        .iter()
        .rev()
        .find(|message| message.role == ChatMessageRole::Assistant)
    {
        let preview_source = last_assistant
            .text
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or(last_assistant.text.as_str())
            .trim();
        let preview = truncate_to(preview_source, 72);
        let model = truncate_to(&app.chat.active_model, 24);
        let agent = truncate_to(&app.chat.active_agent, 16);
        return format!("{agent} · {model} · {preview}");
    }

    app.status_message.clone()
}

fn render_action_menu(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = if let Some((x, y)) = app.action_menu_anchor {
        anchored_rect(44, 13, x, y, area)
    } else {
        centered_rect(44, 13, area)
    };
    app.layout.action_menu_rect = Some(popup);

    frame.render_widget(Clear, popup);
    let block = styled_panel("Actions", true, &app.theme);
    let inner = inset_rect(block.inner(popup), 1, 0);
    frame.render_widget(block, popup);

    let mut lines = Vec::with_capacity(MENU_ACTIONS.len() + 1);
    for (index, action) in MENU_ACTIONS.iter().enumerate() {
        if index == ACTION_MENU_DANGER_SPLIT_AFTER + 1 {
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width.saturating_sub(1) as usize),
                Style::default().fg(app.theme.pane_unfocused_border),
            )));
        }

        let is_selected = index == app.action_menu_index;
        let is_danger = matches!(
            action,
            MenuAction::KillSession | MenuAction::KillAllSessions
        );
        let is_close = matches!(action, MenuAction::CloseMenu);
        let base_fg = if is_danger {
            app.theme.pill_err_fg
        } else if is_close {
            app.theme.text_muted
        } else {
            app.theme.text_secondary
        };
        let mut row_style = Style::default().fg(base_fg);
        if is_selected {
            row_style = row_style.bg(if is_danger {
                Color::Rgb(45, 18, 18)
            } else {
                app.theme.selected_card_bg
            });
            if !is_close {
                row_style = row_style.add_modifier(Modifier::BOLD);
            }
        }

        let prefix = if is_selected { "▸ " } else { "  " };
        let hotkey = action.hotkey();
        let mut spans = vec![Span::styled(prefix, row_style)];

        if is_close {
            let text = format!("{hotkey} {}", action.label());
            spans.push(Span::styled(text.clone(), row_style));
            let used = prefix.chars().count() + text.chars().count();
            let pad = inner.width.saturating_sub(used as u16) as usize;
            if pad > 0 {
                spans.push(Span::styled(" ".repeat(pad), row_style));
            }
        } else {
            let hotkey_text = format!(" {hotkey} ");
            let label_text = format!(" {}", action.label());
            let hotkey_style = Style::default()
                .fg(app.theme.key_hint_key_fg)
                .bg(app.theme.key_hint_key_bg)
                .add_modifier(Modifier::BOLD);

            spans.push(Span::styled(hotkey_text.clone(), hotkey_style));
            spans.push(Span::styled(label_text.clone(), row_style));

            let used =
                prefix.chars().count() + hotkey_text.chars().count() + label_text.chars().count();
            let pad = inner.width.saturating_sub(used as u16) as usize;
            if pad > 0 {
                spans.push(Span::styled(" ".repeat(pad), row_style));
            }
        }

        lines.push(Line::from(spans));
    }

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn render_kill_confirmation(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = centered_rect(58, 7, area);
    app.layout.confirm_rect = Some(popup);

    frame.render_widget(Clear, popup);

    let block = styled_panel("Confirm Kill", true, &app.theme);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let target_name = app
        .selected_session()
        .map(|session| session.session_name.clone())
        .unwrap_or_else(|| String::from("(none)"));

    let body = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Kill session `"),
            Span::styled(target_name, Style::default().fg(app.theme.pill_err_fg)),
            Span::raw("`?"),
        ]),
        Line::from("This will terminate tmux session and remove DB record."),
        Line::from(""),
        Line::from(Span::styled(
            "[y/enter] confirm  [n/esc] cancel",
            Style::default().fg(app.theme.pill_warn_fg),
        )),
    ])
    .wrap(Wrap { trim: true });
    frame.render_widget(body, inner);
}

fn render_kill_all_confirmation(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = centered_rect(58, 7, area);
    app.layout.confirm_rect = Some(popup);

    frame.render_widget(Clear, popup);

    let block = styled_panel("Confirm Kill All", true, &app.theme);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let count = app.sessions.len();
    let body = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Kill all "),
            Span::styled(
                count.to_string(),
                Style::default().fg(app.theme.pill_err_fg),
            ),
            Span::raw(" sessions?"),
        ]),
        Line::from("This will terminate tmux sessions and remove DB records."),
        Line::from(""),
        Line::from(Span::styled(
            "[y/enter] confirm  [n/esc] cancel",
            Style::default().fg(app.theme.pill_warn_fg),
        )),
    ])
    .wrap(Wrap { trim: true });
    frame.render_widget(body, inner);
}

fn render_input_overlay(frame: &mut Frame, area: Rect, app: &mut App, title: &str) {
    let popup_height = if app.mode == AppMode::SpawnInput {
        11
    } else {
        6
    };
    let popup = centered_rect(70, popup_height, area);
    app.layout.overlay.dialog_rect = Some(popup);
    app.layout.overlay.field_rects.clear();
    app.layout.overlay.prompt_editor_rect = None;
    frame.render_widget(Clear, popup);

    let block = styled_panel(title, true, &app.theme);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = if app.mode == AppMode::SpawnInput {
        let fields = vec![
            (
                "Session",
                app.spawn_form.session_name.as_str(),
                "(auto city name)",
            ),
            ("Agent", app.spawn_form.agent_type.as_str(), "opencode"),
            ("Model", app.spawn_form.model.as_str(), "(optional)"),
            (
                "Prompt",
                app.spawn_form.prompt.as_str(),
                "(inline text or file path)",
            ),
        ];

        let mut form_lines: Vec<Line<'static>> = Vec::new();
        for (index, (label, value, placeholder)) in fields.iter().enumerate() {
            let y = inner.y.saturating_add(index as u16);
            app.layout.overlay.field_rects.push(Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            });
            form_lines.push(form_line(
                label,
                value,
                placeholder,
                app.spawn_form.active_field == index,
                app,
            ));
        }

        let editor_line_y = inner.y.saturating_add(5);
        app.layout.overlay.prompt_editor_rect = Some(Rect {
            x: inner.x,
            y: editor_line_y,
            width: inner.width,
            height: 1,
        });

        form_lines.push(Line::from(""));
        form_lines.push(prompt_editor_line(app));
        form_lines.push(Line::from(""));
        form_lines.push(footer_hint_line_for_mode(AppMode::SpawnInput, app));
        form_lines
    } else {
        let mut dialog_lines = vec![
            Line::from("Command:"),
            Line::from(Span::styled(
                app.input_buffer.as_str(),
                Style::default().fg(app.theme.text_primary),
            )),
            Line::from(""),
        ];
        dialog_lines.push(footer_hint_line_for_mode(AppMode::SendInput, app));
        dialog_lines
    };
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn form_line(
    label: &str,
    value: &str,
    placeholder: &str,
    active: bool,
    app: &App,
) -> Line<'static> {
    let mut rendered = if value.is_empty() && !active {
        placeholder.to_owned()
    } else {
        value.to_owned()
    };
    if active {
        rendered.push('█');
    }

    let placeholder_style = if value.is_empty() && !active {
        Style::default().fg(app.theme.text_muted)
    } else {
        Style::default().fg(app.theme.text_secondary)
    };

    let value_style = if active {
        Style::default()
            .fg(app.theme.text_primary)
            .add_modifier(Modifier::BOLD)
    } else {
        placeholder_style
    };
    Line::from(vec![
        Span::styled(
            format!("{:>8}", label),
            Style::default().fg(app.theme.text_muted),
        ),
        Span::styled(
            " ▏",
            if active {
                Style::default().fg(app.theme.pane_focused_border)
            } else {
                Style::default().fg(app.theme.text_muted)
            },
        ),
        Span::styled(rendered, value_style),
    ])
}

fn prompt_editor_line(app: &App) -> Line<'static> {
    let mut spans = Vec::new();
    spans.extend(key_hint("e", "edit prompt in $EDITOR", &app.theme));
    Line::from(spans)
}

fn footer_actions_for_mode(mode: AppMode) -> Vec<FooterHotkeyAction> {
    match mode {
        AppMode::Normal => vec![
            FooterHotkeyAction::Quit,
            FooterHotkeyAction::Navigate,
            FooterHotkeyAction::SidebarTab,
            FooterHotkeyAction::Refresh,
            FooterHotkeyAction::ToggleInspector,
            FooterHotkeyAction::OpenActions,
            FooterHotkeyAction::Attach,
            FooterHotkeyAction::Spawn,
            FooterHotkeyAction::Kill,
        ],
        AppMode::ActionMenu => vec![
            FooterHotkeyAction::FormSubmit,
            FooterHotkeyAction::FormCancel,
        ],
        AppMode::ConfirmKill | AppMode::ConfirmKillAll => {
            vec![FooterHotkeyAction::Confirm, FooterHotkeyAction::Cancel]
        }
        AppMode::SendInput => vec![
            FooterHotkeyAction::FormSubmit,
            FooterHotkeyAction::FormCancel,
        ],
        AppMode::SpawnInput => vec![
            FooterHotkeyAction::FormNextField,
            FooterHotkeyAction::FormEditPrompt,
            FooterHotkeyAction::FormSubmit,
            FooterHotkeyAction::FormCancel,
        ],
    }
}

fn footer_hint(action: FooterHotkeyAction, hovered: bool, app: &App) -> Vec<Span<'static>> {
    let key_style = if hovered {
        Style::default()
            .fg(app.theme.key_hint_key_fg)
            .bg(app.theme.key_hint_key_bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(app.theme.pill_accent_fg)
            .add_modifier(Modifier::BOLD)
    };
    vec![
        Span::styled(format!(" {} ", action.key()), key_style),
        Span::styled(
            format!(" {}", action.label()),
            Style::default().fg(app.theme.text_secondary),
        ),
    ]
}

fn footer_hint_line_for_mode(mode: AppMode, app: &App) -> Line<'static> {
    let actions = footer_actions_for_mode(mode);
    let mut spans = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(
                " | ",
                Style::default().fg(app.theme.key_hint_bracket_fg),
            ));
        }
        spans.extend(footer_hint(*action, false, app));
    }
    Line::from(spans)
}

pub(super) fn footer_hotkey_hit_test(app: &App, col: u16, row: u16) -> Option<FooterHotkeyAction> {
    let area = app.layout.footer_rect?;
    if row != area.y || col < area.x || col >= area.right() {
        return None;
    }

    let actions = footer_actions_for_mode(app.mode);
    let mut cursor_x = area.x;
    for (index, action) in actions.iter().enumerate() {
        if index > 0 {
            cursor_x = cursor_x.saturating_add(3);
        }
        let key_width = action.key().chars().count() as u16 + 2;
        if col >= cursor_x && col < cursor_x.saturating_add(key_width) {
            return Some(*action);
        }
        cursor_x = cursor_x.saturating_add(key_width);
        cursor_x = cursor_x.saturating_add(action.label().chars().count() as u16 + 1);
    }

    None
}

fn status_color(status: &str, app: &App) -> Color {
    if status.eq_ignore_ascii_case("active") {
        return app.theme.pill_ok_fg;
    }
    if status.eq_ignore_ascii_case("stale") {
        return app.theme.pill_warn_fg;
    }
    if status.eq_ignore_ascii_case("dead") {
        return app.theme.pill_err_fg;
    }
    app.theme.text_secondary
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime};
    use insta::assert_snapshot;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use tinyverse_lib::StoredSession;

    use super::render_frame;
    use crate::TuiRunOptions;
    use crate::app::{App, AppMode};

    #[test]
    fn renders_empty_state_snapshot() {
        let mut app = App::new(TuiRunOptions::default());
        app.status_message = String::from("No sessions found");

        let snapshot = render_snapshot(&mut app, 100, 30);
        assert_snapshot!("tui_empty_state", snapshot);
    }

    #[test]
    fn renders_inspector_and_menu_snapshot() {
        let mut app = App::new(TuiRunOptions::default());
        app.sessions
            .push(mock_session("tinyverse_alpha", "opencode"));
        app.mode = AppMode::ActionMenu;

        let snapshot = render_snapshot(&mut app, 120, 36);
        assert_snapshot!("tui_action_menu", snapshot);
    }

    #[test]
    fn renders_confirm_kill_snapshot() {
        let mut app = App::new(TuiRunOptions::default());
        app.sessions
            .push(mock_session("tinyverse_alpha", "opencode"));
        app.mode = AppMode::ConfirmKill;

        let snapshot = render_snapshot(&mut app, 120, 36);
        assert_snapshot!("tui_confirm_kill", snapshot);
    }

    fn render_snapshot(app: &mut App, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        terminal
            .draw(|frame| render_frame(frame, app))
            .expect("draw should succeed");

        buffer_to_text(terminal.backend().buffer())
    }

    fn buffer_to_text(buffer: &Buffer) -> String {
        let area = buffer.area();
        let mut out = String::new();
        for y in 0..area.height {
            let mut row = String::new();
            for x in 0..area.width {
                row.push_str(buffer[(x, y)].symbol());
            }
            let trimmed = row.trim_end_matches(' ');
            out.push_str(trimmed);
            out.push('\n');
        }
        out
    }

    fn mock_session(name: &str, agent: &str) -> StoredSession {
        StoredSession {
            id: 1,
            session_key: name.to_owned(),
            session_name: name.to_owned(),
            agent_type: agent.to_owned(),
            description: Some(String::from("Snapshot test session")),
            status_string: String::from("active"),
            tmux_session_name: name.to_owned(),
            tmux_session_id: Some(String::from("$1")),
            console_pane_id: Some(String::from("%1")),
            agent_pane_id: Some(String::from("%2")),
            created_at: epoch_naive(),
            last_message_at: None,
            updated_at: epoch_naive(),
        }
    }

    fn epoch_naive() -> NaiveDateTime {
        DateTime::UNIX_EPOCH.naive_utc()
    }
}
