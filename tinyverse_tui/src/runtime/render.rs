use ansi_to_tui::IntoText;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};
use std::time::{SystemTime, UNIX_EPOCH};
use tinyverse_tui_components::{
    KeyBind, PaneBlockComponent, StatusPill, Tone, anchored_rect, centered_rect, compact_text,
    inset_rect,
};

use crate::app::{
    ACTION_MENU_DANGER_SPLIT_AFTER, App, AppMode, FooterHotkeyAction, MENU_ACTIONS, MenuAction,
    SessionTreeNode, SessionsViewMode, SidebarTab,
};
use crate::chat::ChatMessageRole;
use crate::theme::UiTheme;

use super::chat_render::render_chat_tab;
use super::session_tree_render::render_session_tree;

const CARD_HEIGHT: u16 = 10;
const CARD_GAP_X: u16 = 2;
const CARD_GAP_Y: u16 = 1;
const CARD_MIN_GRID_WIDTH: u16 = 34;
const MINIMIZED_PANEL_WIDTH: u16 = 7;
const MINIMIZED_SIDEBAR_WIDTH: u16 = 9;

pub(crate) fn render_frame(frame: &mut Frame, app: &mut App) {
    let root = frame.area();
    app.layout = Default::default();

    frame.render_widget(
        Block::default().style(Style::default().bg(app.theme.base_bg)),
        root,
    );

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
        AppMode::PaneFocus => {}
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
        let split = if app.sessions_minimized {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(MINIMIZED_PANEL_WIDTH),
                    Constraint::Min(20),
                ])
                .split(top_area)
        } else if app.sidebar_minimized {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(20),
                    Constraint::Length(MINIMIZED_SIDEBAR_WIDTH),
                ])
                .split(top_area)
        } else {
            let left_pct = app.inspector_ratio.clamp(40, 80);
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(left_pct),
                    Constraint::Percentage(100 - left_pct),
                ])
                .split(top_area)
        };
        app.layout.divider_x = Some(split[0].right());
        render_sessions_panel(frame, split[0], app);
        render_sidebar(frame, split[1], app);
    } else {
        app.layout.divider_x = None;
        app.layout.sidebar_tab_rects.clear();
        app.layout.sidebar_preview_rect = None;
        render_sessions_panel(frame, top_area, app);
    }

    if app.inspector_visible && main_chunks[1].height > 0 {
        render_bottom_inspector(frame, main_chunks[1], app);
    }
}

fn render_cards(frame: &mut Frame, area: Rect, app: &mut App) {
    app.layout.sessions_header_rect = Some(Rect {
        x: area.x.saturating_add(1),
        y: area.y,
        width: area.width.saturating_sub(2),
        height: 1,
    });

    if app.sessions_minimized {
        let panel = styled_panel("S.", true, &app.theme);
        let inner = inset_rect(panel.inner(area), 1, 1);
        frame.render_widget(panel, area);
        if inner.width > 0 && inner.height > 0 {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    "◂",
                    Style::default().fg(app.theme.text_secondary),
                ))),
                inner,
            );
        }
        app.layout.card_rects.clear();
        app.layout.card_kill_rects.clear();
        app.layout.session_tree_row_rects.clear();
        return;
    }

    let panel = styled_panel("Sessions", true, &app.theme);
    let inner = inset_rect(panel.inner(area), 1, 1);
    frame.render_widget(panel, area);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);
    render_sessions_view_tabs(frame, app, sections[0]);
    let content = inset_rect(sections[1], 1, 0);

    let body_sections = if content.height >= 16 {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(7)])
            .split(content)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(0)])
            .split(content)
    };
    let cards_area = body_sections[0];
    let threads_area = body_sections[1];

    app.layout.session_tree_row_rects.clear();

    if app.sessions.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from("No sessions found."),
            Line::from("Run `tinyverse spawn <name>` to create one."),
        ])
        .style(Style::default().fg(app.theme.text_secondary))
        .wrap(Wrap { trim: true });

        let popup = centered_rect(58, 5, cards_area);
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

    let session_count = app.sessions.len();
    let (cols, card_width) = card_grid_layout(cards_area.width, session_count);
    let stride_x = card_width + CARD_GAP_X;
    let stride_y = CARD_HEIGHT + CARD_GAP_Y;
    let cols_usize = cols as usize;
    let visible_rows = ((cards_area.height + CARD_GAP_Y) / stride_y).max(1) as usize;
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
        let x = cards_area.x + col * stride_x;
        let y = cards_area.y + row * stride_y;

        if y + CARD_HEIGHT > cards_area.bottom() {
            break;
        }

        let width = card_width.min(cards_area.right().saturating_sub(x));
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

        let created = session.created_at.format("%b %d %H:%M").to_string();
        let last_update = session.last_message_at.unwrap_or(session.updated_at);
        let ago = relative_time_ago(last_update.and_utc().timestamp());
        let bottom_meta_style = Style::default().fg(app.theme.text_muted);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Line::from(vec![Span::styled(
                format!(
                    " {} ",
                    truncate_to(
                        &session.session_name,
                        card_area.width.saturating_sub(12).max(10) as usize,
                    )
                ),
                title_style,
            )]))
            .title_bottom(Line::from(vec![
                Span::styled(format!(" {created} "), bottom_meta_style),
                Span::styled("·", bottom_meta_style),
                Span::styled(format!(" {ago} "), bottom_meta_style),
            ]));

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
        let max_preview_width = card_area.width.saturating_sub(6).max(8) as usize;

        let message_count = app.chat.messages.len();
        let message_label = if message_count == 1 {
            String::from("1 msg")
        } else {
            format!("{message_count} msgs")
        };

        let mut card_lines = vec![
            Line::from(vec![
                status_pill(&session.status_string, &app.theme),
                Span::raw(" "),
                tag_pill(&truncate_to(&session.agent_type, 12), &app.theme),
            ]),
            Line::from(vec![
                pill_badge(
                    &truncate_to(
                        &app.chat.active_model,
                        card_area.width.saturating_sub(16).max(8) as usize,
                    ),
                    app.theme.pill_info_fg,
                    app.theme.pill_info_bg,
                    false,
                ),
                Span::raw(" "),
                pill_badge(
                    &message_label,
                    app.theme.pill_warn_fg,
                    app.theme.pill_warn_bg,
                    true,
                ),
            ]),
        ];

        if app.repo_name.is_some() || app.git_branch.is_some() {
            let label = match (app.repo_name.as_deref(), app.git_branch.as_deref()) {
                (Some(repo), Some(branch)) => {
                    format!("{}#{}", truncate_to(repo, 10), truncate_to(branch, 12))
                }
                (Some(repo), None) => truncate_to(repo, 16),
                (None, Some(branch)) => format!("#{}", truncate_to(branch, 14)),
                (None, None) => String::new(),
            };

            if !label.is_empty() {
                card_lines.push(Line::from(vec![Span::styled(
                    format!(" {label} "),
                    Style::default()
                        .fg(app.theme.pill_ok_fg)
                        .bg(app.theme.pill_ok_bg),
                )]));
            }
        }

        let should_show_preview = is_selected || app.show_card_preview_on_all_cards;
        if should_show_preview
            && let Some(preview_text) = card_preview_source_text(app, &session.session_key)
        {
            let preview_line_count = if is_selected { 3 } else { 1 };
            card_lines.push(Line::from(Span::styled(
                "─".repeat(max_preview_width.min(card_inner.width as usize)),
                Style::default().fg(app.theme.pane_unfocused_border),
            )));
            for line in preview_excerpt_lines(&preview_text, max_preview_width, preview_line_count)
            {
                card_lines.push(Line::from(Span::styled(
                    format!("{line}"),
                    if is_selected {
                        Style::default().fg(app.theme.text_primary)
                    } else {
                        Style::default().fg(app.theme.text_muted)
                    },
                )));
            }
        }

        let body = Paragraph::new(card_lines).style(if is_selected {
            Style::default().bg(app.theme.selected_card_bg)
        } else {
            Style::default()
        });

        frame.render_widget(body, card_inner);
    }

    if threads_area.height > 0 {
        render_threads_panel(frame, threads_area, app);
    }
}

fn render_threads_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let selected_index = app.selected_index.min(app.sessions.len().saturating_sub(1));
    let threads = app
        .session_tree_rows
        .iter()
        .filter_map(|row| match &row.node {
            SessionTreeNode::ChatSession {
                session_index,
                chat_session_id,
            } if *session_index == selected_index => {
                Some((row.label.as_str(), chat_session_id.as_str()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let title = format!("Threads ({})", threads.len());
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.pane_unfocused_border));
    let inner = inset_rect(block.inner(area), 1, 0);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    if threads.is_empty() {
        frame.render_widget(
            Paragraph::new("No chat threads for selected session")
                .style(Style::default().fg(app.theme.text_muted))
                .wrap(Wrap { trim: true }),
            inner,
        );
        return;
    }

    let active_id = app.chat_bridge.active_session_id();
    let lines = threads
        .into_iter()
        .map(|(label, id)| {
            let is_active = active_id == Some(id);
            let marker = if is_active { "* " } else { "  " };
            let text = compact_text(label, inner.width.saturating_sub(2) as usize);
            let style = if is_active {
                Style::default()
                    .fg(app.theme.text_primary)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text_secondary)
            };
            Line::from(Span::styled(format!("{marker}{text}"), style))
        })
        .collect::<Vec<_>>();

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_sessions_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    if app.sessions_minimized {
        render_cards(frame, area, app);
        return;
    }

    match app.sessions_view_mode {
        SessionsViewMode::Graphical => render_cards(frame, area, app),
        SessionsViewMode::Tree => render_session_tree(frame, area, app),
    }
}

pub(super) fn render_sessions_view_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    app.layout.sessions_view_tab_rects.clear();
    let mut spans = Vec::new();
    let mut cursor_x = area.x;
    for (index, mode) in SessionsViewMode::all().iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(" ", Style::default().fg(app.theme.text_muted)));
            cursor_x = cursor_x.saturating_add(1);
        }

        let selected = *mode == app.sessions_view_mode;
        let label = mode.title();
        let style = if selected {
            Style::default()
                .fg(app.theme.key_hint_key_fg)
                .bg(app.theme.key_hint_key_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(app.theme.text_secondary)
                .bg(app.theme.pill_muted_bg)
        };
        let text = format!(" {label} ");
        let width = text.chars().count() as u16;
        app.layout.sessions_view_tab_rects.push((
            *mode,
            Rect {
                x: cursor_x,
                y: area.y,
                width,
                height: 1,
            },
        ));
        cursor_x = cursor_x.saturating_add(width);
        spans.push(Span::styled(text, style));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn card_grid_layout(inner_width: u16, session_count: usize) -> (u16, u16) {
    if inner_width == 0 {
        return (1, 0);
    }

    let three_col_min = CARD_MIN_GRID_WIDTH.saturating_mul(3) + CARD_GAP_X.saturating_mul(2);
    let two_col_min = CARD_MIN_GRID_WIDTH.saturating_mul(2) + CARD_GAP_X;

    let mut cols: u16 = if inner_width >= three_col_min {
        3
    } else if inner_width >= two_col_min {
        2
    } else {
        1
    };

    if session_count <= 1 {
        cols = 1;
    }

    if cols == 1 {
        return (1, inner_width);
    }

    let total_gap = CARD_GAP_X.saturating_mul(cols.saturating_sub(1));
    let width = inner_width.saturating_sub(total_gap) / cols;
    (cols, width)
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &mut App) {
    app.layout.sidebar_header_rect = Some(Rect {
        x: area.x.saturating_add(1),
        y: area.y,
        width: area.width.saturating_sub(2),
        height: 1,
    });

    if app.sidebar_minimized {
        let block = styled_panel("P.", true, &app.theme);
        let inner = inset_rect(block.inner(area), 1, 1);
        frame.render_widget(block, area);
        if inner.width > 0 && inner.height > 0 {
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    "▸",
                    Style::default().fg(app.theme.text_secondary),
                ))),
                inner,
            );
        }
        app.layout.sidebar_tab_rects.clear();
        app.layout.sidebar_preview_rect = None;
        return;
    }

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

    let preview_area = sidebar_chunks[1];
    let is_focus_mode = app.mode == AppMode::PaneFocus;

    match app.sidebar_tab {
        SidebarTab::Console => {
            render_pane_preview(
                frame,
                preview_area,
                &console_preview,
                "Console",
                &session.session_name,
                is_focus_mode,
                app,
            );
        }
        SidebarTab::Agent => {
            render_pane_preview(
                frame,
                preview_area,
                &agent_preview,
                "Agent",
                &session.session_name,
                is_focus_mode,
                app,
            );
        }
        SidebarTab::Chat => {}
    }
}

fn render_pane_preview(
    frame: &mut Frame,
    area: Rect,
    preview_text: &str,
    pane_label: &str,
    session_name: &str,
    is_focus_mode: bool,
    app: &App,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    let mode_badge = if is_focus_mode {
        (" live ", app.theme.pill_ok_fg, app.theme.pill_ok_bg)
    } else {
        (
            " snapshot ",
            app.theme.pill_muted_fg,
            app.theme.pill_muted_bg,
        )
    };

    let header = Line::from(vec![
        Span::styled(
            format!(" {} ", pane_label),
            Style::default()
                .fg(app.theme.key_hint_key_fg)
                .bg(app.theme.key_hint_key_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            mode_badge.0,
            Style::default()
                .fg(mode_badge.1)
                .bg(mode_badge.2)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            compact_text(session_name, chunks[0].width.saturating_sub(18) as usize),
            Style::default().fg(app.theme.text_muted),
        ),
    ]);
    frame.render_widget(Paragraph::new(header), chunks[0]);

    if let Some(text) = fit_preview_ansi_text(preview_text, chunks[1]) {
        frame.render_widget(Paragraph::new(text), chunks[1]);
    } else {
        let fitted = fit_preview_text(preview_text, chunks[1]);
        frame.render_widget(
            Paragraph::new(fitted).style(Style::default().fg(app.theme.text_secondary)),
            chunks[1],
        );
    }
}

fn fit_preview_ansi_text(text: &str, area: Rect) -> Option<Text<'static>> {
    if area.width == 0 || area.height == 0 || text.trim().is_empty() {
        return None;
    }

    let parsed = text.as_bytes().to_vec().into_text().ok()?;
    if parsed.lines.is_empty() {
        return None;
    }

    let max_lines = area.height as usize;
    let start = parsed.lines.len().saturating_sub(max_lines);
    let lines = parsed.lines[start..]
        .iter()
        .map(convert_core_line)
        .collect::<Vec<_>>();

    Some(Text::from(lines))
}

fn convert_core_line(line: &ratatui_core::text::Line<'_>) -> Line<'static> {
    let mut spans = Vec::new();
    for span in &line.spans {
        let content = span.content.as_ref().to_owned();
        if content.is_empty() {
            continue;
        }
        spans.push(Span::styled(content, convert_core_style(span.style)));
    }

    Line::from(spans)
}

fn convert_core_style(style: ratatui_core::style::Style) -> Style {
    let mut out = Style::default();
    if let Some(fg) = style.fg {
        out = out.fg(convert_core_color(fg));
    }
    let add = convert_core_modifier(style.add_modifier);
    if !add.is_empty() {
        out = out.add_modifier(add);
    }
    let sub = convert_core_modifier(style.sub_modifier);
    if !sub.is_empty() {
        out = out.remove_modifier(sub);
    }

    out
}

fn convert_core_color(color: ratatui_core::style::Color) -> Color {
    match color {
        ratatui_core::style::Color::Reset => Color::Reset,
        ratatui_core::style::Color::Black => Color::Black,
        ratatui_core::style::Color::Red => Color::Red,
        ratatui_core::style::Color::Green => Color::Green,
        ratatui_core::style::Color::Yellow => Color::Yellow,
        ratatui_core::style::Color::Blue => Color::Blue,
        ratatui_core::style::Color::Magenta => Color::Magenta,
        ratatui_core::style::Color::Cyan => Color::Cyan,
        ratatui_core::style::Color::Gray => Color::Gray,
        ratatui_core::style::Color::DarkGray => Color::DarkGray,
        ratatui_core::style::Color::LightRed => Color::LightRed,
        ratatui_core::style::Color::LightGreen => Color::LightGreen,
        ratatui_core::style::Color::LightYellow => Color::LightYellow,
        ratatui_core::style::Color::LightBlue => Color::LightBlue,
        ratatui_core::style::Color::LightMagenta => Color::LightMagenta,
        ratatui_core::style::Color::LightCyan => Color::LightCyan,
        ratatui_core::style::Color::White => Color::White,
        ratatui_core::style::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        ratatui_core::style::Color::Indexed(v) => Color::Indexed(v),
    }
}

fn convert_core_modifier(modifier: ratatui_core::style::Modifier) -> Modifier {
    let mut out = Modifier::empty();
    if modifier.contains(ratatui_core::style::Modifier::BOLD) {
        out |= Modifier::BOLD;
    }
    if modifier.contains(ratatui_core::style::Modifier::DIM) {
        out |= Modifier::DIM;
    }
    if modifier.contains(ratatui_core::style::Modifier::ITALIC) {
        out |= Modifier::ITALIC;
    }
    if modifier.contains(ratatui_core::style::Modifier::UNDERLINED) {
        out |= Modifier::UNDERLINED;
    }
    if modifier.contains(ratatui_core::style::Modifier::SLOW_BLINK) {
        out |= Modifier::SLOW_BLINK;
    }
    if modifier.contains(ratatui_core::style::Modifier::RAPID_BLINK) {
        out |= Modifier::RAPID_BLINK;
    }
    if modifier.contains(ratatui_core::style::Modifier::REVERSED) {
        out |= Modifier::REVERSED;
    }
    if modifier.contains(ratatui_core::style::Modifier::HIDDEN) {
        out |= Modifier::HIDDEN;
    }
    if modifier.contains(ratatui_core::style::Modifier::CROSSED_OUT) {
        out |= Modifier::CROSSED_OUT;
    }
    out
}

fn render_bottom_inspector(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_panel_transparent("Inspector", true, &app.theme);
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

    let created = session.created_at.format("%b %d %H:%M").to_string();
    let last_update = session.last_message_at.unwrap_or(session.updated_at);
    let ago = relative_time_ago(last_update.and_utc().timestamp());
    let bridge = app.chat_bridge.status();
    let git_label = match (app.repo_name.as_deref(), app.git_branch.as_deref()) {
        (Some(repo), Some(branch)) => format!("{repo}#{branch}"),
        (Some(repo), None) => repo.to_owned(),
        (None, Some(branch)) => format!("#{branch}"),
        (None, None) => String::from("-"),
    };
    let assistant_preview =
        latest_assistant_preview(app).unwrap_or_else(|| String::from("No assistant messages yet"));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(3)])
        .split(inner);

    let badge_line = Line::from(vec![
        status_pill(&session.status_string, &app.theme),
        Span::raw(" "),
        tag_pill(&truncate_to(&session.agent_type, 14), &app.theme),
        Span::raw(" "),
        pill_badge(
            &format!("{} msg", app.chat.messages.len()),
            app.theme.pill_info_fg,
            app.theme.pill_info_bg,
            false,
        ),
        Span::raw(" "),
        pill_badge(
            &format!("{}", truncate_to(&app.chat.active_model, 24)),
            app.theme.pill_accent_fg,
            app.theme.pill_accent_bg,
            false,
        ),
    ]);
    frame.render_widget(Paragraph::new(badge_line), chunks[0]);

    let left_rows = vec![
        Row::new(vec![
            Cell::from("Name"),
            Cell::from(session.session_name.as_str()),
        ]),
        Row::new(vec![Cell::from("Created"), Cell::from(created)]),
        Row::new(vec![Cell::from("Last Update"), Cell::from(ago)]),
        Row::new(vec![
            Cell::from("Assistant"),
            Cell::from(truncate_to(&assistant_preview, 44)),
        ]),
    ];

    let right_rows = vec![
        Row::new(vec![
            Cell::from("Key"),
            Cell::from(session.session_key.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Tmux"),
            Cell::from(session.tmux_session_name.as_str()),
        ]),
        Row::new(vec![
            Cell::from("Panes"),
            Cell::from(format!(
                "console={} agent={}",
                session.console_pane_id.as_deref().unwrap_or("-"),
                session.agent_pane_id.as_deref().unwrap_or("-")
            )),
        ]),
        Row::new(vec![
            Cell::from("Git"),
            Cell::from(truncate_to(&git_label, 44)),
        ]),
        Row::new(vec![
            Cell::from("Chat Bridge"),
            Cell::from(format!(
                "{} ({})",
                bridge.mode.label(),
                truncate_to(&bridge.detail, 28)
            )),
        ]),
    ];

    if chunks[1].width >= 92 {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
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
        frame.render_widget(table, chunks[1]);
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

    let status = footer_status_message(app);
    if !status.is_empty() {
        mode_hints.push(Span::styled(
            " | ",
            Style::default().fg(app.theme.key_hint_bracket_fg),
        ));
        mode_hints.push(Span::styled(
            status,
            Style::default().fg(app.theme.text_primary),
        ));
    }

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
    app.status_message.clone()
}

fn latest_assistant_preview(app: &App) -> Option<String> {
    app.chat
        .messages
        .iter()
        .rev()
        .find(|message| message.role == ChatMessageRole::Assistant)
        .map(|message| message.preview_line())
}

fn card_preview_source_text(app: &App, session_key: &str) -> Option<String> {
    let _ = session_key;
    latest_assistant_preview(app)
}

fn preview_excerpt_lines(text: &str, width: usize, max_lines: usize) -> Vec<String> {
    if width == 0 || max_lines == 0 {
        return Vec::new();
    }

    let mut out = Vec::new();
    for source in text.lines() {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut current = String::new();
        for word in trimmed.split_whitespace() {
            let next_len = if current.is_empty() {
                word.chars().count()
            } else {
                current.chars().count() + 1 + word.chars().count()
            };

            if next_len > width {
                if !current.is_empty() {
                    out.push(current.clone());
                    if out.len() >= max_lines {
                        return out;
                    }
                    current.clear();
                }
                if word.chars().count() > width {
                    out.push(truncate_to(word, width));
                    if out.len() >= max_lines {
                        return out;
                    }
                } else {
                    current.push_str(word);
                }
            } else {
                if !current.is_empty() {
                    current.push(' ');
                }
                current.push_str(word);
            }
        }

        if !current.is_empty() {
            out.push(current);
            if out.len() >= max_lines {
                return out;
            }
        }
    }

    out
}

pub(super) fn relative_time_ago(event_unix_seconds: i64) -> String {
    if event_unix_seconds <= 0 {
        return String::from("unknown");
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let delta = now.saturating_sub(event_unix_seconds);

    if delta > 86_400 * 365 * 10 {
        return String::from(">10y");
    }

    if delta < 60 {
        String::from("now")
    } else if delta < 3600 {
        format!("{}m ago", delta / 60)
    } else if delta < 86_400 {
        format!("{}h ago", delta / 3600)
    } else {
        format!("{}d ago", delta / 86_400)
    }
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
            FooterHotkeyAction::SessionView,
            FooterHotkeyAction::PaneFocus,
        ],
        AppMode::PaneFocus => vec![
            FooterHotkeyAction::PaneFocus,
            FooterHotkeyAction::PaneFocusExit,
            FooterHotkeyAction::Navigate,
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

pub(super) fn styled_panel<'a>(title: &'a str, focused: bool, theme: &UiTheme) -> Block<'a> {
    PaneBlockComponent::build(title, focused, theme)
}

pub(super) fn styled_panel_transparent<'a>(
    title: &'a str,
    focused: bool,
    theme: &UiTheme,
) -> Block<'a> {
    PaneBlockComponent::build_transparent(title, focused, theme)
}

fn status_pill(status: &str, theme: &UiTheme) -> Span<'static> {
    let lowered = status.to_ascii_lowercase();
    let tone = match lowered.as_str() {
        "active" => Tone::Ok,
        "stale" => Tone::Warn,
        "dead" => Tone::Error,
        _ => Tone::Muted,
    };
    StatusPill::for_tone(lowered, tone, theme).span()
}

fn tag_pill(label: &str, theme: &UiTheme) -> Span<'static> {
    StatusPill::accent(label, theme).span()
}

fn pill_badge(label: &str, fg: Color, bg: Color, bold: bool) -> Span<'static> {
    StatusPill::custom(label, fg, bg, bold).span()
}

fn truncate_to(value: &str, max_len: usize) -> String {
    compact_text(value, max_len)
}

fn key_hint(key: &'static str, action: &'static str, theme: &UiTheme) -> Vec<Span<'static>> {
    KeyBind::new(key, action).spans(theme)
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
            agent_base_url: Some(String::from("http://127.0.0.1:4150")),
            agent_session_id: None,
            created_at: epoch_naive(),
            last_message_at: None,
            updated_at: epoch_naive(),
        }
    }

    fn epoch_naive() -> NaiveDateTime {
        DateTime::UNIX_EPOCH.naive_utc()
    }
}
