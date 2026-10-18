//! Tests for `src/main17.rs` — Is Subsequence.

mod main17 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main17.rs");

    #[cfg(test)]
    mod tests {
        use super::is_subsequence;

        #[test]
        fn example_one() {
            assert!(is_subsequence(&"abc".to_string(), &"ahbgdc".to_string()));
        }

        #[test]
        fn example_two() {
            assert!(!is_subsequence(&"axc".to_string(), &"ahbgdc".to_string()));
        }

        #[test]
        fn empty_source_is_always_a_subsequence() {
            assert!(is_subsequence(&String::new(), &"anything".to_string()));
            assert!(is_subsequence(&String::new(), &String::new()));
        }

        #[test]
        fn non_empty_source_cannot_be_a_subsequence_of_an_empty_target() {
            assert!(!is_subsequence(&"a".to_string(), &String::new()));
        }

        #[test]
        fn accepts_non_contiguous_characters() {
            assert!(is_subsequence(&"ace".to_string(), &"abcde".to_string()));
        }

        #[test]
        fn rejects_characters_in_the_wrong_order() {
            assert!(!is_subsequence(&"aec".to_string(), &"abcde".to_string()));
        }
    }
}
