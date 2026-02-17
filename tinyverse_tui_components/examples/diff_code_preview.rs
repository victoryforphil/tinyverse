use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use tinyverse_tui_components::{
    CodeViewComponent, CodeViewLine, CodeViewMode, CodeViewProps, ComponentTheme,
    DiffLineNumberMode, DiffViewComponent, DiffViewProps,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let theme = ComponentTheme::default();
    let diff_lines = DiffViewComponent::parse_unified(SAMPLE_DIFF);
    let code_lines = sample_code_lines();
    let mut y_scroll = 0usize;
    let mut x_scroll = 0usize;

    loop {
        terminal.draw(|frame| {
            let [diff_area, code_area] =
                Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .areas(frame.area());

            DiffViewComponent::render(
                frame,
                diff_area,
                &theme,
                DiffViewProps {
                    lines: &diff_lines,
                    scroll: y_scroll,
                    horizontal_offset: x_scroll,
                    title: Some("ratatui/docs sample diff"),
                    mode: CodeViewMode::Normal,
                    line_number_mode: DiffLineNumberMode::Both,
                    empty_message: "(empty diff)",
                },
            );

            CodeViewComponent::render(
                frame,
                code_area,
                &theme,
                CodeViewProps {
                    lines: &code_lines,
                    scroll: y_scroll,
                    horizontal_offset: x_scroll,
                    title: Some("code preview (manual token styling)"),
                    show_line_numbers: true,
                    mode: CodeViewMode::Compact,
                    empty_message: "(no code)",
                },
            );
        })?;

        if event::poll(Duration::from_millis(120))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => y_scroll = y_scroll.saturating_add(1),
                KeyCode::Char('k') | KeyCode::Up => y_scroll = y_scroll.saturating_sub(1),
                KeyCode::Char('h') | KeyCode::Left => x_scroll = x_scroll.saturating_sub(2),
                KeyCode::Char('l') | KeyCode::Right => x_scroll = x_scroll.saturating_add(2),
                KeyCode::Char('g') => {
                    y_scroll = 0;
                    x_scroll = 0;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn sample_code_lines() -> Vec<CodeViewLine> {
    vec![
        CodeViewLine::with_style(1, "use ratatui::widgets::Paragraph;", keyword_style()),
        CodeViewLine::plain(2, ""),
        CodeViewLine {
            line_number: Some(3),
            marker: None,
            marker_style: None,
            gutter_label: None,
            content: Line::from(vec![
                Span::styled("pub ", keyword_style()),
                Span::styled("fn ", keyword_style()),
                Span::styled("render_diff", fn_style()),
                Span::raw("(frame: &mut Frame, area: Rect) {"),
            ]),
        },
        CodeViewLine::plain(4, "    let title = \"Diff\";"),
        CodeViewLine::plain(5, "    let block = Block::bordered().title(title);"),
        CodeViewLine::plain(6, "    frame.render_widget(block, area);"),
        CodeViewLine::plain(7, "}"),
    ]
}

fn keyword_style() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

fn fn_style() -> Style {
    Style::default().add_modifier(Modifier::UNDERLINED)
}

const SAMPLE_DIFF: &str = r#"diff --git a/docs/concepts/layout.md b/docs/concepts/layout.md
index 63db27a..7f8a9e2 100644
--- a/docs/concepts/layout.md
+++ b/docs/concepts/layout.md
@@ -16,8 +16,11 @@ fn layout(frame: &mut Frame) {
 let chunks = Layout::vertical([
-    Constraint::Length(3),
-    Constraint::Min(1),
+    Constraint::Length(2),
+    Constraint::Length(3),
+    Constraint::Min(1),
 ]).split(frame.area());

~ // reduced header footprint and added status row
 frame.render_widget(header(), chunks[0]);
+frame.render_widget(status_bar(), chunks[1]);
 frame.render_widget(content(), chunks[2]);
}"#;
