//! Tests for `src/main24.rs` — Merge Strings Alternately.

mod main24 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main24.rs");

    #[cfg(test)]
    mod tests {
        use super::merge_alternately;

        #[test]
        fn equal_length_words_are_interleaved() {
            assert_eq!(
                "apbqcr",
                merge_alternately("abc".to_string(), "pqr".to_string())
            );
        }

        #[test]
        fn longer_second_word_contributes_its_tail() {
            assert_eq!(
                "apbqrs",
                merge_alternately("ab".to_string(), "pqrs".to_string())
            );
        }

        #[test]
        fn longer_first_word_contributes_its_tail() {
            assert_eq!(
                "apbqcd",
                merge_alternately("abcd".to_string(), "pq".to_string())
            );
        }

        #[test]
        fn empty_word_returns_the_other_word() {
            assert_eq!("ab", merge_alternately(String::new(), "ab".to_string()));
            assert_eq!("ab", merge_alternately("ab".to_string(), String::new()));
        }

        #[test]
        fn both_words_empty_produces_empty_string() {
            assert_eq!("", merge_alternately(String::new(), String::new()));
        }
    }
}
