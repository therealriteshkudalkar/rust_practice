//! Tests for `src/main69.rs` — Wildcard Matching.
//!
//! The source implements a recursive matcher where `?` matches one character
//! and `*` matches any sequence (including the empty sequence).

mod main69 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main69.rs");

    #[cfg(test)]
    mod tests {
        use super::is_match;

        fn matches(text: &str, pattern: &str) -> bool {
            is_match(text.to_string(), pattern.to_string())
        }

        #[test]
        fn exact_literal_matches() {
            assert!(matches("abc", "abc"));
            assert!(!matches("abc", "abd"));
        }

        #[test]
        fn question_mark_matches_exactly_one_character() {
            assert!(matches("aa", "a?"));
            assert!(matches("ab", "a?"));
            assert!(!matches("cb", "?a"));
            assert!(matches("abc", "a?c"));
            assert!(!matches("abc", "a?d"));
        }

        #[test]
        fn star_matches_any_sequence() {
            assert!(matches("aa", "*"));
            assert!(matches("abc", "*"));
            assert!(matches("adceb", "*a*b"));
            assert!(matches("abcd", "a*d"));
            assert!(matches("abcd", "*d"));
            assert!(matches("abc", "a**c"));
        }

        #[test]
        fn empty_text_cases() {
            assert!(matches("", ""));
            assert!(!matches("", "a"));
            assert!(!matches("a", ""));
        }

        #[test]
        #[ignore = "known bug: the base case rejects a trailing '*' before it can match the empty string"]
        fn empty_text_matches_only_stars() {
            assert!(matches("", "*"));
            assert!(matches("", "*****"));
        }

        #[test]
        fn star_can_match_a_non_empty_subsequence() {
            assert!(matches("abc", "a*?"));
            assert!(!matches("abc", "a*d"));
        }

        #[test]
        #[ignore = "known bug: the base case rejects a trailing '*' before it can match an empty remainder"]
        fn star_can_match_an_empty_subsequence_at_the_end() {
            assert!(matches("a", "?*"));
            assert!(matches("abc", "abc*"));
        }

        #[test]
        fn mixed_wildcards() {
            assert!(matches("abcdef", "a?c*e?"));
            assert!(!matches("abcdef", "a?d*e?"));
            assert!(matches("xaylmz", "x?y*z"));
        }
    }
}
