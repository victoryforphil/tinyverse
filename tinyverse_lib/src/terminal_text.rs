pub fn strip_ansi_and_controls(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            match chars.peek().copied() {
                Some('[') => {
                    let _ = chars.next();
                    for c in chars.by_ref() {
                        if ('@'..='~').contains(&c) {
                            break;
                        }
                    }
                    continue;
                }
                Some(']') => {
                    let _ = chars.next();
                    let mut prev = '\0';
                    for c in chars.by_ref() {
                        if prev == '\u{1b}' && c == '\\' {
                            break;
                        }
                        if c == '\u{07}' {
                            break;
                        }
                        prev = c;
                    }
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }

        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }

        out.push(ch);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::strip_ansi_and_controls;

    #[test]
    fn strips_csi_sequences() {
        let input = "\u{1b}[31mERROR\u{1b}[0m done";
        assert_eq!(strip_ansi_and_controls(input), "ERROR done");
    }

    #[test]
    fn strips_osc_sequences() {
        let input = "\u{1b}]0;title\u{07}hello";
        assert_eq!(strip_ansi_and_controls(input), "hello");
    }

    #[test]
    fn keeps_newline_and_tab() {
        let input = "a\tb\n\u{08}c";
        assert_eq!(strip_ansi_and_controls(input), "a\tb\nc");
    }
}
