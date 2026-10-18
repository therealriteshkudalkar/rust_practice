//! Tests for `src/main66.rs` — Longest Palindromic Substring.

mod main66 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main66.rs");

    #[cfg(test)]
    mod tests {
        use super::{is_palindrome, longest_palindrome, longest_palindrome_optimized};

        #[test]
        fn palindrome_checker_accepts_odd_and_even_palindromes() {
            assert!(is_palindrome(""));
            assert!(is_palindrome("a"));
            assert!(is_palindrome("aba"));
            assert!(is_palindrome("abba"));
        }

        #[test]
        fn palindrome_checker_rejects_non_palindromes() {
            assert!(!is_palindrome("ab"));
            assert!(!is_palindrome("abc"));
            assert!(!is_palindrome("abab"));
        }

        #[test]
        fn optimized_solution_examples() {
            assert_eq!("bab", longest_palindrome_optimized("babad".to_string()));
            assert_eq!("bb", longest_palindrome_optimized("cbbd".to_string()));
            assert_eq!("a", longest_palindrome_optimized("a".to_string()));
            assert_eq!("a", longest_palindrome_optimized("ac".to_string()));
        }

        #[test]
        fn optimized_solution_finds_long_embedded_palindromes() {
            assert_eq!(
                "geeksskeeg",
                longest_palindrome_optimized("forgeeksskeegfor".to_string())
            );
            assert_eq!("aaaa", longest_palindrome_optimized("aaaa".to_string()));
        }

        #[test]
        fn naive_solution_matches_on_short_examples() {
            assert_eq!("bab", longest_palindrome("babad".to_string()));
            assert_eq!("bb", longest_palindrome("cbbd".to_string()));
            assert_eq!("a", longest_palindrome("a".to_string()));
            assert_eq!("a", longest_palindrome("ac".to_string()));
        }

        #[test]
        fn both_solutions_return_a_palindrome_of_the_same_length() {
            let cases = vec![
                "babad",
                "cbbd",
                "a",
                "ac",
                "racecar",
                "abacdfgdcaba",
                "forgeeksskeegfor",
            ];
            for input in cases {
                let naive = longest_palindrome(input.to_string());
                let optimized = longest_palindrome_optimized(input.to_string());
                assert!(is_palindrome(&naive), "naive result for {input}: {naive}");
                assert!(
                    is_palindrome(&optimized),
                    "optimized result for {input}: {optimized}"
                );
                assert_eq!(
                    naive.chars().count(),
                    optimized.chars().count(),
                    "input = {input}"
                );
            }
        }
    }
}
