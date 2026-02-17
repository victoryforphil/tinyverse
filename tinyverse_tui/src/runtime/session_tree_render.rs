use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use tinyverse_tui_components::{
    FileTreeBadge, FileTreeComponent, FileTreeProps, FileTreeRow, inset_rect,
};

use crate::app::{App, SessionTreeNode, SidebarTab};

use super::render::{relative_time_ago, render_sessions_view_tabs, styled_panel_transparent};

/// Renders the session tree view inside the sessions panel.
pub(super) fn render_session_tree(frame: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    app.layout.sessions_header_rect = Some(ratatui::layout::Rect {
        x: area.x.saturating_add(1),
        y: area.y,
        width: area.width.saturating_sub(2),
        height: 1,
    });

    let panel = styled_panel_transparent("Sessions", true, &app.theme);
    let inner = inset_rect(panel.inner(area), 1, 1);
    frame.render_widget(panel, area);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);
    render_sessions_view_tabs(frame, app, sections[0]);

    let tree_area = inset_rect(sections[1], 1, 0);
    app.layout.card_rects.clear();
    app.layout.card_kill_rects.clear();
    app.layout.session_tree_row_rects.clear();

    let rows = build_file_rows(app);
    let layout = FileTreeComponent::render(
        frame,
        tree_area,
        FileTreeProps {
            rows: &rows,
            selected: app.session_tree_cursor,
            scroll: app.session_tree_scroll,
            empty_message: "No sessions found",
        },
        &app.theme,
    );
    app.session_tree_scroll = layout.scroll;
    app.layout.session_tree_row_rects = layout
        .row_hitboxes
        .into_iter()
        .map(|hit| (hit.row_index, hit.rect))
        .collect();
}

// ── Row mapping helpers ────────────────────────────────────────────

/// Converts app-level `SessionTreeRow`s into generic `FileTreeRow`s
/// suitable for the shared component.
fn build_file_rows(app: &App) -> Vec<FileTreeRow> {
    app.session_tree_rows
        .iter()
        .map(|row| {
            let (status, meta) = row_status_and_meta(app, row);
            let icon = node_icon(&row.node);
            let selected_bg = node_selected_bg(app, &row.node);
            let badges = node_badges(app, row);
            FileTreeRow {
                label: row.label.clone(),
                depth: row.depth,
                is_last: row.is_last,
                ancestors_are_last: row.ancestors_are_last.clone(),
                icon: (!icon.is_empty()).then(|| icon.to_owned()),
                is_active: is_active_target(app, row),
                status,
                meta,
                selected_bg,
                badges,
            }
        })
        .collect()
}

fn row_status_and_meta(
    app: &App,
    row: &crate::app::SessionTreeRow,
) -> (Option<String>, Option<String>) {
    match &row.node {
        SessionTreeNode::SessionRoot { session_index } => {
            let Some(session) = app.sessions.get(*session_index) else {
                return (None, None);
            };
            let stamp = session.last_message_at.unwrap_or(session.updated_at);
            (
                Some(session.status_string.clone()),
                Some(relative_time_ago(stamp.and_utc().timestamp())),
            )
        }
        SessionTreeNode::SidebarPane { .. } | SessionTreeNode::ChatSession { .. } => (None, None),
    }
}

fn node_icon(node: &SessionTreeNode) -> &'static str {
    match node {
        SessionTreeNode::SessionRoot { .. } => "⬡",
        SessionTreeNode::SidebarPane { tab, .. } => match tab {
            SidebarTab::Console => "▪",
            SidebarTab::Agent => "◈",
            SidebarTab::Chat => "◆",
        },
        SessionTreeNode::ChatSession { .. } => "›",
    }
}

fn is_active_target(app: &App, row: &crate::app::SessionTreeRow) -> bool {
    let selected_index = app.selected_index.min(app.sessions.len().saturating_sub(1));
    match &row.node {
        SessionTreeNode::SessionRoot { session_index } => *session_index == selected_index,
        SessionTreeNode::SidebarPane { session_index, tab } => {
            *session_index == selected_index && *tab == app.sidebar_tab
        }
        SessionTreeNode::ChatSession {
            session_index,
            chat_session_id,
        } => {
            *session_index == selected_index
                && app.sidebar_tab == SidebarTab::Chat
                && app.chat_bridge.active_session_id() == Some(chat_session_id.as_str())
        }
    }
}

/// Maps each node kind to a distinct selection background tint.
fn node_selected_bg(app: &App, node: &SessionTreeNode) -> Option<Color> {
    Some(match node {
        SessionTreeNode::SessionRoot { .. } => app.theme.tree_tint_session,
        SessionTreeNode::SidebarPane { tab, .. } => match tab {
            SidebarTab::Console => app.theme.tree_tint_console,
            SidebarTab::Agent => app.theme.tree_tint_agent,
            SidebarTab::Chat => app.theme.tree_tint_chat,
        },
        SessionTreeNode::ChatSession { .. } => app.theme.tree_tint_thread,
    })
}

/// Builds contextual badges for a tree row.
fn node_badges(app: &App, row: &crate::app::SessionTreeRow) -> Vec<FileTreeBadge> {
    match &row.node {
        SessionTreeNode::SessionRoot { session_index } => {
            // Count immediate child rows for this session root.
            let child_count = count_session_children(app, *session_index);
            if child_count > 0 {
                vec![FileTreeBadge {
                    label: format!("threads:{child_count}"),
                    fg: app.theme.tree_badge_fg,
                    bg: app.theme.tree_badge_bg,
                }]
            } else {
                Vec::new()
            }
        }
        SessionTreeNode::ChatSession { .. } => {
            vec![FileTreeBadge {
                label: String::from("thread"),
                fg: app.theme.tree_badge_fg,
                bg: app.theme.tree_badge_bg,
            }]
        }
        SessionTreeNode::SidebarPane { .. } => Vec::new(),
    }
}

/// Counts the visible child rows belonging to a given session root.
///
/// For the selected (expanded) session this is the number of pane and
/// chat-session rows.  For collapsed sessions this is always 0.
fn count_session_children(app: &App, session_index: usize) -> usize {
    let mut count = 0usize;
    let mut found_root = false;
    for row in &app.session_tree_rows {
        match &row.node {
            SessionTreeNode::SessionRoot { session_index: si } if *si == session_index => {
                found_root = true;
            }
            SessionTreeNode::SessionRoot { .. } if found_root => {
                // Hit the next session root – stop counting.
                break;
            }
            _ if found_root => {
                count += 1;
            }
            _ => {}
        }
    }
    count
}
