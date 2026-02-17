use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, AppMode, MenuAction, MENU_ACTIONS};

use super::helpers::{anchored_rect, centered_rect, inset_rect, line_kv, truncate_to};

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
    render_footer(frame, chunks[2], app);

    match app.mode {
        AppMode::ActionMenu => render_action_menu(frame, root, app),
        AppMode::ConfirmKill => render_kill_confirmation(frame, root, app),
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
            line_kv("status", &truncate_to(&session.status_string, 20)),
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

    let lines = if let Some(session) = app.selected_session() {
        vec![
            line_kv("name", &session.session_name),
            line_kv("key", &session.session_key),
            line_kv("agent", &session.agent_type),
            line_kv("status", &session.status_string),
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
        ]
    } else {
        vec![Line::from(Span::styled(
            "No session selected",
            Style::default().fg(Color::Gray),
        ))]
    };

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let refresh = app
        .last_refresh_at
        .map(|time| format!("refreshed {}s ago", time.elapsed().as_secs()))
        .unwrap_or_else(|| String::from("never refreshed"));

    let mode_hints = match app.mode {
        AppMode::Normal => "[q] quit [arrows/hjkl] nav [r] refresh [i] inspector [enter] menu",
        AppMode::ActionMenu => "[j/k] menu [enter] select [1-7] quick [esc] close",
        AppMode::ConfirmKill => "Confirm kill: [y/enter] yes [n/esc] no",
        AppMode::SendInput => "Send mode: type command [enter] send [esc] cancel",
        AppMode::SpawnInput => "Spawn mode: type session name [enter] spawn [esc] cancel",
    };

    let footer = Line::from(vec![
        Span::styled(mode_hints, Style::default().fg(Color::Gray)),
        Span::raw(" | "),
        Span::styled(
            app.status_message.as_str(),
            Style::default().fg(Color::White),
        ),
        Span::raw(" | "),
        Span::styled(refresh, Style::default().fg(Color::DarkGray)),
    ]);

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
        let style = if matches!(action, MenuAction::KillSession) {
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

fn render_input_overlay(frame: &mut Frame, area: Rect, app: &App, title: &str) {
    let popup = centered_rect(70, 6, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let prompt = match app.mode {
        AppMode::SendInput => "Command:",
        AppMode::SpawnInput => "Session name:",
        _ => "Input:",
    };

    let lines = vec![
        Line::from(prompt),
        Line::from(Span::styled(
            app.input_buffer.as_str(),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[enter] submit  [esc] cancel",
            Style::default().fg(Color::Gray),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime};
    use insta::assert_snapshot;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::Terminal;
    use tinyverse_lib::StoredSession;

    use super::render_frame;
    use crate::app::{App, AppMode};
    use crate::TuiRunOptions;

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
