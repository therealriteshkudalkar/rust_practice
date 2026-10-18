//! Tests for `src/main67.rs` — Implement strStr().

mod main67 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main67.rs");

    #[cfg(test)]
    mod tests {
        use super::str_str;

        #[test]
        fn example_one() {
            assert_eq!(0, str_str("sadbutsad".to_string(), "sad".to_string()));
        }

        #[test]
        fn example_two() {
            assert_eq!(-1, str_str("leetcode".to_string(), "leeto".to_string()));
        }

        #[test]
        fn single_character_match() {
            assert_eq!(0, str_str("a".to_string(), "a".to_string()));
        }

        #[test]
        fn match_in_the_middle() {
            assert_eq!(2, str_str("hello".to_string(), "ll".to_string()));
        }

        #[test]
        fn no_match_returns_minus_one() {
            assert_eq!(-1, str_str("aaaa".to_string(), "bba".to_string()));
        }

        #[test]
        fn empty_needle_is_found_at_zero() {
            assert_eq!(0, str_str("abc".to_string(), String::new()));
        }

        #[test]
        fn needle_longer_than_haystack_is_not_found() {
            assert_eq!(-1, str_str("ab".to_string(), "abc".to_string()));
        }
    }
}
