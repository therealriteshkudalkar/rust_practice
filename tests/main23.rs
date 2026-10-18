//! Tests for `src/main23.rs` — Reverse Words in a String.

mod main23 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main23.rs");

    #[cfg(test)]
    mod tests {
        use super::reverse_words;

        #[test]
        fn example_one() {
            assert_eq!(
                "blue is sky the",
                reverse_words(&"the sky is blue".to_string())
            );
        }

        #[test]
        fn trims_leading_and_trailing_spaces() {
            assert_eq!("world hello", reverse_words(&"  hello world  ".to_string()));
        }

        #[test]
        fn collapses_multiple_spaces_between_words() {
            assert_eq!(
                "example good a",
                reverse_words(&"a good   example".to_string())
            );
        }

        #[test]
        fn single_word_is_returned_unchanged() {
            assert_eq!("single", reverse_words(&"single".to_string()));
        }

        #[test]
        fn empty_string_returns_empty_string() {
            assert_eq!("", reverse_words(&String::new()));
            assert_eq!("", reverse_words(&"   ".to_string()));
        }
    }
}
