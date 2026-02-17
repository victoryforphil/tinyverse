use super::filter::filter_items;
use super::state::{PickerItem, PickerState};

fn make_items(names: &[&str]) -> Vec<PickerItem> {
    names
        .iter()
        .map(|name| PickerItem {
            label: tinyverse_ui::format_display_name(name),
            key: name.to_string(),
        })
        .collect()
}

#[test]
fn full_workflow_filter_navigate_select() {
    let items = make_items(&[
        "tinyverse_redding",
        "tinyverse_oakland",
        "tinyverse_red_bluff",
        "tinyverse_sacramento",
    ]);

    let mut state = PickerState::new("Select session".to_owned(), items);
    assert_eq!(state.visible_count(), 4);

    for ch in "red".chars() {
        state.push_char(ch);
    }
    assert_eq!(state.visible_count(), 2);
    assert_eq!(state.selected_key(), Some("tinyverse_redding"));
    assert_eq!(
        state.selected_label(),
        Some("Redding do TinyVerse // Redding")
    );

    state.move_down();
    assert_eq!(state.selected_key(), Some("tinyverse_red_bluff"));

    state.pop_char();
    state.pop_char();
    state.pop_char();
    assert_eq!(state.visible_count(), 4);
}

#[test]
fn filter_items_preserves_original_order() {
    let labels = vec!["Charlie".to_owned(), "Alpha".to_owned(), "Beta".to_owned()];
    let result = filter_items(&labels, "a");
    assert_eq!(result, vec![0, 1, 2]);
}

#[test]
fn empty_items_produces_no_selection() {
    let state = PickerState::new("Empty".to_owned(), Vec::new());
    assert_eq!(state.visible_count(), 0);
    assert_eq!(state.selected_key(), None);
}
