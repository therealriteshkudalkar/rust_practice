//! Tests for `src/main21.rs` — Valid Anagram.

mod main21 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main21.rs");

    #[cfg(test)]
    mod tests {
        use super::{count_chars, is_anagram};

        #[test]
        fn count_chars_counts_every_occurrence() {
            let counts = count_chars(&"aabbbc".to_string());
            assert_eq!(2, counts[&'a']);
            assert_eq!(3, counts[&'b']);
            assert_eq!(1, counts[&'c']);
            assert!(!counts.contains_key(&'d'));
        }

        #[test]
        fn example_one() {
            assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        }

        #[test]
        fn example_two() {
            assert!(!is_anagram("rat".to_string(), "car".to_string()));
        }

        #[test]
        fn empty_strings_are_anagrams() {
            assert!(is_anagram(String::new(), String::new()));
        }

        #[test]
        fn strings_of_different_length_are_not_anagrams() {
            assert!(!is_anagram("ab".to_string(), "a".to_string()));
            assert!(!is_anagram("a".to_string(), String::new()));
        }

        #[test]
        fn repeated_letters_must_have_the_same_multiplicity() {
            assert!(is_anagram("aabb".to_string(), "abab".to_string()));
            assert!(!is_anagram("aabb".to_string(), "aaab".to_string()));
        }
    }
}
