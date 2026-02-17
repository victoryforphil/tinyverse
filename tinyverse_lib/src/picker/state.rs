use super::filter::filter_items;

/// A selectable item in the picker list.
#[derive(Debug, Clone)]
pub struct PickerItem {
    /// Display label shown in the picker list.
    pub label: String,
    /// Unique key returned when this item is selected.
    pub key: String,
}

/// Result of running the interactive picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickerOutcome {
    /// User confirmed a selection.
    Selected(String),
    /// User cancelled with Esc or Ctrl+C.
    Cancelled,
}

/// Internal picker state driving the render/input loop.
pub struct PickerState {
    pub title: String,
    pub items: Vec<PickerItem>,
    labels: Vec<String>,
    pub query: String,
    pub filtered_indices: Vec<usize>,
    pub cursor: usize,
}

impl PickerState {
    pub fn new(title: String, items: Vec<PickerItem>) -> Self {
        let labels: Vec<String> = items.iter().map(|item| item.label.clone()).collect();
        let filtered_indices = filter_items(&labels, "");
        Self {
            title,
            items,
            labels,
            query: String::new(),
            filtered_indices,
            cursor: 0,
        }
    }

    /// Recompute filtered indices based on current query.
    pub fn refilter(&mut self) {
        self.filtered_indices = filter_items(&self.labels, &self.query);
        if self.cursor >= self.filtered_indices.len() {
            self.cursor = self.filtered_indices.len().saturating_sub(1);
        }
    }

    /// Append a character to the filter query.
    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
        self.refilter();
    }

    /// Remove the last character from the filter query.
    pub fn pop_char(&mut self) {
        self.query.pop();
        self.refilter();
    }

    /// Move cursor up by one (wraps to bottom).
    pub fn move_up(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.cursor == 0 {
            self.cursor = self.filtered_indices.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    /// Move cursor down by one (wraps to top).
    pub fn move_down(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        self.cursor = (self.cursor + 1) % self.filtered_indices.len();
    }

    /// Return the key of the currently selected item, if any.
    pub fn selected_key(&self) -> Option<&str> {
        let &item_index = self.filtered_indices.get(self.cursor)?;
        Some(&self.items[item_index].key)
    }

    /// Return the label of the currently selected item, if any.
    #[allow(dead_code)]
    pub fn selected_label(&self) -> Option<&str> {
        let &item_index = self.filtered_indices.get(self.cursor)?;
        Some(&self.items[item_index].label)
    }

    /// Visible count of filtered items.
    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{PickerItem, PickerState};

    fn sample_items() -> Vec<PickerItem> {
        vec![
            PickerItem {
                label: "TinyVerse: Redding".to_owned(),
                key: "tinyverse_redding".to_owned(),
            },
            PickerItem {
                label: "TinyVerse: Oakland".to_owned(),
                key: "tinyverse_oakland".to_owned(),
            },
            PickerItem {
                label: "TinyVerse: Red Bluff".to_owned(),
                key: "tinyverse_red_bluff".to_owned(),
            },
        ]
    }

    #[test]
    fn initial_state_shows_all_items() {
        let state = PickerState::new("Test".to_owned(), sample_items());
        assert_eq!(state.visible_count(), 3);
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn push_char_filters_items() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        state.push_char('r');
        state.push_char('e');
        state.push_char('d');
        assert_eq!(state.visible_count(), 2);
    }

    #[test]
    fn pop_char_widens_filter() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        state.push_char('r');
        state.push_char('e');
        state.push_char('d');
        assert_eq!(state.visible_count(), 2);
        state.pop_char();
        state.pop_char();
        state.pop_char();
        assert_eq!(state.visible_count(), 3);
    }

    #[test]
    fn move_down_wraps_around() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        assert_eq!(state.cursor, 0);
        state.move_down();
        assert_eq!(state.cursor, 1);
        state.move_down();
        assert_eq!(state.cursor, 2);
        state.move_down();
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn move_up_wraps_around() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        assert_eq!(state.cursor, 0);
        state.move_up();
        assert_eq!(state.cursor, 2);
        state.move_up();
        assert_eq!(state.cursor, 1);
    }

    #[test]
    fn selected_key_returns_correct_key() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        assert_eq!(state.selected_key(), Some("tinyverse_redding"));
        state.move_down();
        assert_eq!(state.selected_key(), Some("tinyverse_oakland"));
    }

    #[test]
    fn filter_then_select_maps_correctly() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        state.push_char('o');
        state.push_char('a');
        state.push_char('k');
        assert_eq!(state.visible_count(), 1);
        assert_eq!(state.selected_key(), Some("tinyverse_oakland"));
    }

    #[test]
    fn cursor_clamps_when_filter_shrinks_list() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        state.move_down();
        state.move_down();
        state.push_char('o');
        state.push_char('a');
        state.push_char('k');
        assert_eq!(state.cursor, 0);
        assert_eq!(state.selected_key(), Some("tinyverse_oakland"));
    }

    #[test]
    fn move_on_empty_list_does_not_panic() {
        let mut state = PickerState::new("Test".to_owned(), sample_items());
        state.push_char('z');
        state.push_char('z');
        state.push_char('z');
        assert_eq!(state.visible_count(), 0);
        state.move_down();
        state.move_up();
        assert_eq!(state.selected_key(), None);
    }
}
