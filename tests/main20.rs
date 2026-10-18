//! Tests for `src/main20.rs` — Reverse Vowels of a String.

mod main20 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main20.rs");

    #[cfg(test)]
    mod tests {
        use super::{is_vowel, reverse_vowels};

        #[test]
        fn vowel_helper_recognises_upper_and_lower_case() {
            for ch in ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
                assert!(is_vowel(ch), "{ch} should be a vowel");
            }
            for ch in ['b', 'z', 'Y', ' ', '1'] {
                assert!(!is_vowel(ch), "{ch} should not be a vowel");
            }
        }

        #[test]
        fn example_one() {
            assert_eq!("holle", reverse_vowels(&"hello".to_string()));
        }

        #[test]
        fn example_two() {
            assert_eq!("leotcede", reverse_vowels(&"leetcode".to_string()));
        }

        #[test]
        fn only_vowels_are_reordered() {
            assert_eq!("uoiea", reverse_vowels(&"aeiou".to_string()));
            assert_eq!("Aa", reverse_vowels(&"aA".to_string()));
        }

        #[test]
        fn strings_without_vowels_are_unchanged() {
            assert_eq!("rhythm", reverse_vowels(&"rhythm".to_string()));
            assert_eq!("", reverse_vowels(&String::new()));
        }

        #[test]
        fn mixed_case_vowels_are_reversed() {
            assert_eq!("hOllE", reverse_vowels(&"hEllO".to_string()));
        }

        #[test]
        fn unchanged_when_there_is_nothing_to_swap() {
            assert_eq!("a.b", reverse_vowels(&"a.b".to_string()));
            assert_eq!(".,", reverse_vowels(&".,".to_string()));
        }
    }
}
