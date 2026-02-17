use crate::render::{truncate_with_ellipsis, RenderContext, RenderMode};

/// Per-column text alignment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    Left,
    Right,
}

/// Optional ANSI row striping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripeMode {
    None,
    DimEvenRows,
}

/// Width-aware table renderer for CLI reports.
pub struct StyledTable<'a> {
    pub headers: Vec<&'a str>,
    pub rows: Vec<Vec<String>>,
    alignments: Vec<ColumnAlignment>,
    stripe_mode: StripeMode,
}

impl<'a> StyledTable<'a> {
    /// Creates a table with left-aligned columns.
    pub fn new(headers: Vec<&'a str>) -> Self {
        let alignments = headers
            .iter()
            .map(|_| ColumnAlignment::Left)
            .collect::<Vec<_>>();
        Self {
            headers,
            rows: Vec::new(),
            alignments,
            stripe_mode: StripeMode::None,
        }
    }

    /// Appends one data row.
    pub fn with_row(mut self, row: Vec<String>) -> Self {
        self.rows.push(row);
        self
    }

    /// Marks provided column indexes as right-aligned.
    pub fn with_numeric_columns(mut self, columns: &[usize]) -> Self {
        for index in columns {
            if let Some(alignment) = self.alignments.get_mut(*index) {
                *alignment = ColumnAlignment::Right;
            }
        }
        self
    }

    /// Enables or disables ANSI striping for table rows.
    pub fn with_stripe_mode(mut self, stripe_mode: StripeMode) -> Self {
        self.stripe_mode = stripe_mode;
        self
    }

    /// Renders the table in plain or ANSI mode.
    pub fn render(&self, context: &RenderContext<'_>) -> String {
        if self.headers.is_empty() {
            return String::new();
        }

        let mut widths = self.measure_widths();
        if let Some(max_width) = context.width {
            fit_widths(&mut widths, max_width);
        }

        let mut lines = Vec::new();

        let header_cells: Vec<String> = self
            .headers
            .iter()
            .enumerate()
            .map(|(index, value)| truncate_with_ellipsis(value, widths[index]))
            .collect();
        let header_row = render_row(&header_cells, &widths, &self.alignments);
        match context.mode {
            RenderMode::Plain => lines.push(header_row),
            RenderMode::Ansi => lines.push(
                context
                    .theme
                    .table_header_style()
                    .paint(header_row)
                    .to_string(),
            ),
        }
        lines.push(render_separator(&widths));

        for (row_number, row) in self.rows.iter().enumerate() {
            let mut cells = widths
                .iter()
                .map(|_| String::new())
                .collect::<Vec<String>>();
            for (index, value) in row.iter().enumerate().take(widths.len()) {
                cells[index] = truncate_with_ellipsis(value, widths[index]);
            }
            let rendered_row = render_row(&cells, &widths, &self.alignments);
            if matches!(context.mode, RenderMode::Ansi)
                && matches!(self.stripe_mode, StripeMode::DimEvenRows)
                && (row_number + 1) % 2 == 0
            {
                lines.push(
                    context
                        .theme
                        .table_stripe_style()
                        .paint(rendered_row)
                        .to_string(),
                );
            } else {
                lines.push(rendered_row);
            }
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
            for (index, value) in row.iter().enumerate().take(widths.len()) {
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

fn render_row(values: &[String], widths: &[usize], alignments: &[ColumnAlignment]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            match alignments
                .get(index)
                .copied()
                .unwrap_or(ColumnAlignment::Left)
            {
                ColumnAlignment::Left => format!("{value:<width$}", width = widths[index]),
                ColumnAlignment::Right => format!("{value:>width$}", width = widths[index]),
            }
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

#[cfg(test)]
mod tests {
    use crate::render::{RenderContext, RenderMode};
    use crate::theme::DefaultTheme;

    use super::{StripeMode, StyledTable};

    #[test]
    fn renders_table_with_headers_and_rows() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, None, &theme);
        let table = StyledTable::new(vec!["ID", "NAME"])
            .with_row(vec!["$1".to_owned(), "tinyverse_alpha".to_owned()]);

        let rendered = table.render(&context);
        assert!(rendered.contains("ID"));
        assert!(rendered.contains("tinyverse_alpha"));
    }

    #[test]
    fn shrinks_columns_when_width_is_limited() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, Some(18), &theme);
        let table = StyledTable::new(vec!["ID", "NAME"]).with_row(vec![
            "$1".to_owned(),
            "tinyverse_really_long_name".to_owned(),
        ]);

        let rendered = table.render(&context);
        assert!(rendered.contains("..."));
    }

    #[test]
    fn right_aligns_numeric_columns() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Plain, None, &theme);
        let table = StyledTable::new(vec!["NAME", "WINDOWS"])
            .with_numeric_columns(&[1])
            .with_row(vec!["alpha".to_owned(), "2".to_owned()])
            .with_row(vec!["beta".to_owned(), "10".to_owned()]);

        let rendered = table.render(&context);
        assert!(rendered.contains("alpha |       2"));
        assert!(rendered.contains("beta  |      10"));
    }

    #[test]
    fn applies_ansi_striping_when_enabled() {
        let theme = DefaultTheme;
        let context = RenderContext::new(RenderMode::Ansi, None, &theme);
        let table = StyledTable::new(vec!["NAME"])
            .with_stripe_mode(StripeMode::DimEvenRows)
            .with_row(vec!["alpha".to_owned()])
            .with_row(vec!["beta".to_owned()]);

        let rendered = table.render(&context);
        assert!(!rendered.contains("\u{1b}[2malpha"));
        assert!(rendered.contains("\u{1b}[2mbeta"));
    }
}
