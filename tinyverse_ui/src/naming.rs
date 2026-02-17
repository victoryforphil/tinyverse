/// Converts an internal session name like `tinyverse_redding` into a
/// user-facing display name like `Redding do TinyVerse // Redding`.
///
/// Rules:
/// - Strip the `tinyverse_` prefix (if present).
/// - Title-case the remainder (first letter of each `_`-separated word).
/// - Join with spaces.
/// - Render as `<City> do TinyVerse // <City>`.
///
/// Names that don't carry the prefix are returned unchanged.
pub fn format_display_name(raw: &str) -> String {
    let body = match raw.strip_prefix("tinyverse_") {
        Some(rest) if !rest.is_empty() => rest,
        _ => return raw.to_owned(),
    };

    let words: Vec<String> = body
        .split('_')
        .filter(|w| !w.is_empty())
        .map(title_case_word)
        .collect();

    let city = words.join(" ");
    format!("{city} do TinyVerse // {city}")
}

fn title_case_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_uppercase().to_string();
            result.extend(chars);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_display_name;

    #[test]
    fn formats_city_session_name() {
        assert_eq!(
            format_display_name("tinyverse_redding"),
            "Redding do TinyVerse // Redding"
        );
    }

    #[test]
    fn formats_compound_city_name() {
        assert_eq!(
            format_display_name("tinyverse_san_bernardino"),
            "San Bernardino do TinyVerse // San Bernardino"
        );
    }

    #[test]
    fn passes_through_non_prefixed_name() {
        assert_eq!(
            format_display_name("my-custom-session"),
            "my-custom-session"
        );
    }

    #[test]
    fn passes_through_bare_prefix() {
        assert_eq!(format_display_name("tinyverse_"), "tinyverse_");
    }

    #[test]
    fn formats_single_word() {
        assert_eq!(
            format_display_name("tinyverse_oakland"),
            "Oakland do TinyVerse // Oakland"
        );
    }
}
