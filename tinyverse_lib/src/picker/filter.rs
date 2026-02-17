/// Case-insensitive substring filter for picker items.
///
/// Returns indices of items whose label contains the query substring.
pub fn filter_items(labels: &[String], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..labels.len()).collect();
    }

    let query_lower = query.to_ascii_lowercase();
    labels
        .iter()
        .enumerate()
        .filter(|(_, label)| label.to_ascii_lowercase().contains(&query_lower))
        .map(|(index, _)| index)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::filter_items;

    #[test]
    fn empty_query_returns_all() {
        let labels = vec!["Alpha".to_owned(), "Beta".to_owned(), "Gamma".to_owned()];
        let result = filter_items(&labels, "");
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn filters_by_substring() {
        let labels = vec![
            "tinyverse_redding".to_owned(),
            "tinyverse_oakland".to_owned(),
            "tinyverse_red_bluff".to_owned(),
        ];
        let result = filter_items(&labels, "red");
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn filter_is_case_insensitive() {
        let labels = vec![
            "TinyVerse: Redding".to_owned(),
            "TinyVerse: Oakland".to_owned(),
        ];
        let result = filter_items(&labels, "REDDING");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn no_matches_returns_empty() {
        let labels = vec!["Alpha".to_owned(), "Beta".to_owned()];
        let result = filter_items(&labels, "zzz");
        assert!(result.is_empty());
    }

    #[test]
    fn empty_labels_returns_empty() {
        let labels: Vec<String> = Vec::new();
        let result = filter_items(&labels, "test");
        assert!(result.is_empty());
    }
}
