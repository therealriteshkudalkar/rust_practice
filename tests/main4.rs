//! Tests for `src/main4.rs` — Remove Letter To Equalize Frequency.

mod main4 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main4.rs");

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;

        use super::{equal_frequency, has_equal_frequency};

        fn frequencies(pairs: &[(char, i8)]) -> HashMap<char, i8> {
            pairs.iter().copied().collect()
        }

        #[test]
        fn has_equal_frequency_detects_equal_and_unequal_maps() {
            assert!(has_equal_frequency(&frequencies(&[('a', 2), ('b', 2)])));
            assert!(has_equal_frequency(&frequencies(&[('a', 1)])));
            assert!(!has_equal_frequency(&frequencies(&[('a', 2), ('b', 1)])));
        }

        #[test]
        fn example_one_is_true() {
            assert!(equal_frequency("abcc".to_string()));
        }

        #[test]
        fn example_two_is_false() {
            assert!(!equal_frequency("aazz".to_string()));
        }

        #[test]
        fn already_equal_and_fixable_words_are_true() {
            assert!(equal_frequency("bac".to_string()));
            assert!(equal_frequency("cac".to_string()));
            assert!(equal_frequency("abbcc".to_string()));
            assert!(equal_frequency("abc".to_string()));
        }

        #[test]
        fn words_that_need_two_removals_are_false() {
            assert!(!equal_frequency("aabb".to_string()));
            assert!(!equal_frequency("aaabbbcc".to_string()));
        }

        #[test]
        fn repeated_single_letter_words_are_true() {
            assert!(equal_frequency("zz".to_string()));
            assert!(equal_frequency("aaaa".to_string()));
        }
    }
}
