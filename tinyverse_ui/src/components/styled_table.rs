use crate::render::{truncate_with_ellipsis, RenderContext};

pub struct StyledTable<'a> {
    pub headers: Vec<&'a str>,
    pub rows: Vec<Vec<String>>,
}

impl<'a> StyledTable<'a> {
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        if self.headers.is_empty() {
            return String::new();
        }

        let mut widths = self.measure_widths();
        if let Some(max_width) = context.width {
            fit_widths(&mut widths, max_width);
        }

        let mut lines = Vec::new();
        lines.push(render_row(
            &self
                .headers
                .iter()
                .enumerate()
                .map(|(index, value)| truncate_with_ellipsis(value, widths[index]))
                .collect::<Vec<_>>(),
            &widths,
        ));
        lines.push(render_separator(&widths));

        for row in &self.rows {
            let cells = row
                .iter()
                .enumerate()
                .map(|(index, value)| truncate_with_ellipsis(value, widths[index]))
                .collect::<Vec<_>>();
            lines.push(render_row(&cells, &widths));
        }

        lines.join("\n")
    }

    fn measure_widths(&self) -> Vec<usize> {
        let mut widths = self
            .headers
            .iter()
            .map(|header| header.chars().count())
            .collect::<Vec<_>>();
        for row in &self.rows {
            for (index, value) in row.iter().enumerate() {
                if let Some(width) = widths.get_mut(index) {
                    *width = (*width).max(value.chars().count());
                }
            }
        }
        widths
    }
}

fn fit_widths(widths: &mut [usize], max_width: usize) {
    if widths.is_empty() {
        return;
    }

    let min_col = 5usize;
    while table_width(widths) > max_width {
        let mut widest_index = None;
        let mut widest_value = 0usize;
        for (index, width) in widths.iter().enumerate() {
            if *width > widest_value && *width > min_col {
                widest_value = *width;
                widest_index = Some(index);
            }
        }

        let Some(index) = widest_index else {
            break;
        };
        widths[index] -= 1;
    }
}

fn table_width(widths: &[usize]) -> usize {
    let content = widths.iter().sum::<usize>();
    let spacing = widths.len().saturating_sub(1) * 3;
    content + spacing
}

fn render_separator(widths: &[usize]) -> String {
    widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect::<Vec<_>>()
        .join("-+-")
}

fn render_row(values: &[String], widths: &[usize]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| format!("{value:<width$}", width = widths[index]))
        .collect::<Vec<_>>()
        .join(" | ")
}

#[cfg(test)]
mod tests {
    use crate::render::{RenderContext, RenderMode};
    use crate::theme::DefaultTheme;

    use super::StyledTable;

    #[test]
    fn renders_table_with_headers_and_rows() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, None, &theme);
        let table = StyledTable {
            headers: vec!["ID", "NAME"],
            rows: vec![vec!["$1".to_owned(), "tinyverse_alpha".to_owned()]],
        };

        let rendered = table.render(&context);
        assert!(rendered.contains("ID"));
        assert!(rendered.contains("tinyverse_alpha"));
    }

    #[test]
    fn shrinks_columns_when_width_is_limited() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, Some(18), &theme);
        let table = StyledTable {
            headers: vec!["ID", "NAME"],
            rows: vec![vec![
                "$1".to_owned(),
                "tinyverse_really_long_name".to_owned(),
            ]],
        };

        let rendered = table.render(&context);
        assert!(rendered.contains("..."));
    }
}
