use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

use crate::app::{App, AppMode, FooterHotkeyAction, MENU_ACTIONS, MenuAction};

use super::helpers::{anchored_rect, centered_rect, inset_rect, key_hint, line_kv, truncate_to};

const CARD_WIDTH: u16 = 34;
const CARD_HEIGHT: u16 = 6;

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
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled(
            format!("{} session(s)", app.sessions.len()),
            Style::default().fg(Color::Gray),
        ),
    ]);

    frame.render_widget(Paragraph::new(title), area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &mut App) {
    app.layout.body_rect = Some(area);

    if app.inspector_visible && area.width >= 90 {
        let left_pct = app.inspector_ratio.clamp(45, 85);
        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(left_pct),
                Constraint::Percentage(100 - left_pct),
            ])
            .split(area);
        app.layout.divider_x = Some(split[0].right());
        render_cards(frame, split[0], app);
        render_inspector(frame, split[1], app);
        return;
    }

    app.layout.divider_x = None;
    render_cards(frame, area, app);
}

fn render_cards(frame: &mut Frame, area: Rect, app: &mut App) {
    if app.sessions.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from("No sessions found."),
            Line::from("Run `tinyverse spawn <name>` to create one."),
        ])
        .style(Style::default().fg(Color::Gray))
        .wrap(Wrap { trim: true });

        let popup = centered_rect(58, 5, area);
        frame.render_widget(
            Block::default()
                .title(" TinyVerse ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
            popup,
        );
        frame.render_widget(empty, inset_rect(popup, 1, 1));
        return;
    }

    let cols = (area.width / CARD_WIDTH).max(1);
    let cols_usize = cols as usize;
    let cell_width = (area.width / cols).max(CARD_WIDTH);
    let visible_rows = (area.height / CARD_HEIGHT).max(1) as usize;
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
        let x = area.x + col * cell_width;
        let y = area.y + row * CARD_HEIGHT;

        if y + CARD_HEIGHT > area.bottom() {
            break;
        }

        let mut width = cell_width;
        if x + width > area.right() {
            width = area.right().saturating_sub(x);
        }
        if width < 12 {
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
            Color::Cyan
        } else {
            Color::DarkGray
        };
        let title_style = if is_selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
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

        let body = Paragraph::new(vec![
            line_kv("key", &truncate_to(&session.session_key, 24)),
            line_kv("agent", &truncate_to(&session.agent_type, 20)),
            Line::from(vec![
                Span::styled("status: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    truncate_to(&session.status_string, 20),
                    Style::default().fg(status_color(&session.status_string)),
                ),
            ]),
            line_kv("tmux", &truncate_to(&session.tmux_session_name, 20)),
        ])
        .wrap(Wrap { trim: true });

        frame.render_widget(body, inner);
    }
}

fn render_inspector(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Inspector ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(session) = app.selected_session() {
        let inspector_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(9),
                Constraint::Min(3),
                Constraint::Min(3),
            ])
            .split(inner);

        let metadata = vec![
            line_kv("name", &session.session_name),
            line_kv("key", &session.session_key),
            line_kv("agent", &session.agent_type),
            Line::from(vec![
                Span::styled("status: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    session.status_string.clone(),
                    Style::default().fg(status_color(&session.status_string)),
                ),
            ]),
            line_kv("tmux", &session.tmux_session_name),
            line_kv("console", session.console_pane_id.as_deref().unwrap_or("-")),
            line_kv(
                "agent pane",
                session.agent_pane_id.as_deref().unwrap_or("-"),
            ),
            line_kv(
                "description",
                session.description.as_deref().unwrap_or("(none)"),
            ),
        ];
        frame.render_widget(
            Paragraph::new(metadata).wrap(Wrap { trim: true }),
            inspector_chunks[0],
        );

        let console_block = Block::default()
            .title(" Console Preview ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let console_inner = console_block.inner(inspector_chunks[1]);
        frame.render_widget(console_block, inspector_chunks[1]);

        let agent_block = Block::default()
            .title(" Agent Preview ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let agent_inner = agent_block.inner(inspector_chunks[2]);
        frame.render_widget(agent_block, inspector_chunks[2]);

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

        frame.render_widget(
            Paragraph::new(console_preview)
                .style(Style::default().fg(Color::Gray))
                .wrap(Wrap { trim: false }),
            console_inner,
        );
        frame.render_widget(
            Paragraph::new(agent_preview)
                .style(Style::default().fg(Color::Gray))
                .wrap(Wrap { trim: false }),
            agent_inner,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No session selected",
                Style::default().fg(Color::Gray),
            ))),
            inner,
        );
    }
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
            mode_hints.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
        }
        let hovered = app.footer_hover_action == Some(*action);
        mode_hints.extend(footer_hint(*action, hovered));
    }

    mode_hints.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
    mode_hints.push(Span::styled(
        app.status_message.as_str().to_owned(),
        Style::default().fg(Color::White),
    ));
    mode_hints.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
    mode_hints.push(Span::styled(refresh, Style::default().fg(Color::DarkGray)));

    let footer = Line::from(mode_hints);

    frame.render_widget(Paragraph::new(footer), area);
}

fn render_action_menu(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = if let Some((x, y)) = app.action_menu_anchor {
        anchored_rect(44, 12, x, y, area)
    } else {
        centered_rect(44, 12, area)
    };
    app.layout.action_menu_rect = Some(popup);

    frame.render_widget(Clear, popup);
    let block = Block::default()
        .title(" Actions ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let mut lines = Vec::with_capacity(MENU_ACTIONS.len());
    for (index, action) in MENU_ACTIONS.iter().enumerate() {
        let is_selected = index == app.action_menu_index;
        let prefix = if is_selected { ">" } else { " " };
        let style = if matches!(
            action,
            MenuAction::KillSession | MenuAction::KillAllSessions
        ) {
            if is_selected {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red)
            }
        } else if is_selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(vec![Span::styled(
            format!("{} {}. {}", prefix, index + 1, action.label()),
            style,
        )]));
    }

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn render_kill_confirmation(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = centered_rect(58, 7, area);
    app.layout.confirm_rect = Some(popup);

    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" Confirm Kill ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let target_name = app
        .selected_session()
        .map(|session| session.session_name.clone())
        .unwrap_or_else(|| String::from("(none)"));

    let body = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Kill session `"),
            Span::styled(target_name, Style::default().fg(Color::Red)),
            Span::raw("`?"),
        ]),
        Line::from("This will terminate tmux session and remove DB record."),
        Line::from(""),
        Line::from(Span::styled(
            "[y/enter] confirm  [n/esc] cancel",
            Style::default().fg(Color::Yellow),
        )),
    ])
    .wrap(Wrap { trim: true });
    frame.render_widget(body, inner);
}

fn render_kill_all_confirmation(frame: &mut Frame, area: Rect, app: &mut App) {
    let popup = centered_rect(58, 7, area);
    app.layout.confirm_rect = Some(popup);

    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" Confirm Kill All ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let count = app.sessions.len();
    let body = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Kill all "),
            Span::styled(count.to_string(), Style::default().fg(Color::Red)),
            Span::raw(" sessions?"),
        ]),
        Line::from("This will terminate tmux sessions and remove DB records."),
        Line::from(""),
        Line::from(Span::styled(
            "[y/enter] confirm  [n/esc] cancel",
            Style::default().fg(Color::Yellow),
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

    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .border_type(BorderType::Rounded)
        .title_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
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
        form_lines.push(prompt_editor_line());
        form_lines.push(Line::from(""));
        form_lines.push(footer_hint_line_for_mode(AppMode::SpawnInput));
        form_lines
    } else {
        let mut dialog_lines = vec![
            Line::from("Command:"),
            Line::from(Span::styled(
                app.input_buffer.as_str(),
                Style::default().fg(Color::White),
            )),
            Line::from(""),
        ];
        dialog_lines.push(footer_hint_line_for_mode(AppMode::SendInput));
        dialog_lines
    };
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn form_line(label: &str, value: &str, placeholder: &str, active: bool) -> Line<'static> {
    let mut rendered = if value.is_empty() && !active {
        placeholder.to_owned()
    } else {
        value.to_owned()
    };
    if active {
        rendered.push('█');
    }

    let placeholder_style = if value.is_empty() && !active {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    let value_style = if active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        placeholder_style
    };
    Line::from(vec![
        Span::styled(
            format!("{:>8}", label),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            " ▏",
            if active {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        Span::styled(rendered, value_style),
    ])
}

fn prompt_editor_line() -> Line<'static> {
    let mut spans = Vec::new();
    spans.extend(key_hint("e", "edit prompt in $EDITOR"));
    Line::from(spans)
}

fn footer_actions_for_mode(mode: AppMode) -> Vec<FooterHotkeyAction> {
    match mode {
        AppMode::Normal => vec![
            FooterHotkeyAction::Quit,
            FooterHotkeyAction::Navigate,
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

fn footer_hint(action: FooterHotkeyAction, hovered: bool) -> Vec<Span<'static>> {
    let key_style = if hovered {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    vec![
        Span::styled(format!(" {} ", action.key()), key_style),
        Span::styled(
            format!(" {}", action.label()),
            Style::default().fg(Color::Gray),
        ),
    ]
}

fn footer_hint_line_for_mode(mode: AppMode) -> Line<'static> {
    let actions = footer_actions_for_mode(mode);
    let mut spans = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
        }
        spans.extend(footer_hint(*action, false));
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

fn status_color(status: &str) -> Color {
    if status.eq_ignore_ascii_case("active") {
        return Color::Green;
    }
    if status.eq_ignore_ascii_case("stale") {
        return Color::Yellow;
    }
    if status.eq_ignore_ascii_case("dead") {
        return Color::Red;
    }
    Color::Gray
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
