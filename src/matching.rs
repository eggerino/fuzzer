pub fn fuzzy_match(search_term: &str, content: &str) -> bool {
    let mut search_iter = search_term.chars();

    // If the search term is empty -> every content string matches
    let mut search_char = match search_iter.next() {
        Some(x) => x,
        None => return true,
    };

    for content_char in content.chars() {
        // Skipp current content char if it does not match

        if search_char == content_char {
            // advance the search iterator -> if it is completed the content string fuzzy matches the search term
            search_char = match search_iter.next() {
                Some(x) => x,
                None => return true,
            };
        }
    }

    // The search iterator is not completed while scanning the content string
    // The content string therefore does not fuzzy match the search term
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn same_strings_fuzzy_match() {
        assert!(fuzzy_match("hello, world", "hello, world"));
    }

    #[test]
    fn substring_fuzzy_matches_string() {
        assert!(fuzzy_match("lo, wor", "hello, world"));
    }

    #[test]
    fn string_does_not_fuzzy_match_substring() {
        assert_eq!(fuzzy_match("hello, world", "lo, wor"), false);
    }

    #[test]
    fn missing_chars_fuzzy_match() {
        assert!(fuzzy_match("hlo ol", "hello, world"));
    }

    #[test]
    fn wrong_capitalization_does_not_fuzzy_match() {
        assert_eq!(fuzzy_match("Hello, World", "hello, world"), false);
    }
}
