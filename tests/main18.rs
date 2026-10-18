//! Tests for `src/main18.rs` — Count Sorted Vowel Strings.

mod main18 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main18.rs");

    #[cfg(test)]
    mod tests {
        use super::count_vowel_strings;

        #[test]
        fn length_one_has_five_strings() {
            assert_eq!(5, count_vowel_strings(1));
        }

        #[test]
        fn length_two_has_fifteen_strings() {
            assert_eq!(15, count_vowel_strings(2));
        }

        #[test]
        fn small_lengths_follow_the_binomial_formula() {
            // C(n + 4, 4)
            assert_eq!(35, count_vowel_strings(3));
            assert_eq!(70, count_vowel_strings(4));
            assert_eq!(126, count_vowel_strings(5));
            assert_eq!(330, count_vowel_strings(7));
        }

        #[test]
        fn largest_leetcode_input() {
            assert_eq!(66_045, count_vowel_strings(33));
        }
    }
}
