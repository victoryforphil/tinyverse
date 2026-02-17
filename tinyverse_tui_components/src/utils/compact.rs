const ELLIPSIS: &str = "…";

/// Truncates text to `max_len` with an ellipsis suffix.
pub fn compact_text(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }

    if max_len == 0 {
        return String::new();
    }

    let head = value
        .chars()
        .take(max_len.saturating_sub(1))
        .collect::<String>();
    format!("{head}{ELLIPSIS}")
}

/// Trims and newline-normalizes text before compacting.
pub fn compact_text_normalized(value: &str, max_len: usize) -> String {
    let normalized = value.trim().replace('\n', " ");
    if normalized.chars().count() <= max_len {
        return normalized;
    }

    if max_len == 0 {
        return String::new();
    }

    let head = normalized
        .chars()
        .take(max_len.saturating_sub(1))
        .collect::<String>();
    format!("{head}{ELLIPSIS}")
}

/// Keeps the trailing portion of a long string and prefixes an ellipsis.
pub fn compact_tail(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_len {
        return trimmed.to_string();
    }

    if max_len <= 1 {
        return ELLIPSIS.repeat(max_len);
    }

    let keep = max_len - 1;
    let tail = trimmed
        .chars()
        .rev()
        .take(keep)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{ELLIPSIS}{tail}")
}

/// Compacts an optional label, using `-` for missing values.
pub fn compact_label(value: Option<&str>, max_len: usize) -> String {
    let Some(value) = value else {
        return "-".to_string();
    };

    if value.chars().count() <= max_len {
        return value.to_string();
    }

    if max_len <= 1 {
        return ELLIPSIS.repeat(max_len);
    }

    let head = value.chars().take(max_len - 1).collect::<String>();
    format!("{head}{ELLIPSIS}")
}

/// Compacts IDs to the default length used in TUI lists.
pub fn compact_id(value: &str) -> String {
    compact_id_len(value, 12)
}

/// Compacts IDs to a custom maximum length.
pub fn compact_id_len(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_len {
        return trimmed.to_string();
    }

    let head = trimmed.chars().take(max_len).collect::<String>();
    format!("{head}{ELLIPSIS}")
}

/// Keeps the tail of a locator string when it exceeds the target width.
pub fn compact_locator(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_len {
        return trimmed.to_string();
    }

    if max_len <= 1 {
        return ELLIPSIS.repeat(max_len);
    }

    let suffix_len = max_len.saturating_sub(1);
    let tail = trimmed
        .chars()
        .rev()
        .take(suffix_len)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{ELLIPSIS}{tail}")
}

/// Formats timestamps into concise display-friendly strings.
pub fn compact_timestamp(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "-".to_string();
    }

    if let Some((date, rest)) = trimmed.split_once('T') {
        let time = rest.trim_end_matches('Z').split('.').next().unwrap_or(rest);
        return format!("{date} {time}");
    }

    trimmed.to_string()
}

/// Returns a short display slice for session identifiers.
pub fn compact_session_id(value: &str) -> &str {
    if value.len() <= 14 {
        value
    } else {
        &value[..14]
    }
}
