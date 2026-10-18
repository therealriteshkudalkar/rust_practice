//! Tests for `src/main2.rs` — Palindrome Number.

mod main2 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main2.rs");

    #[cfg(test)]
    mod tests {
        use super::is_palindrome;

        #[test]
        fn accepts_single_digit_numbers() {
            for x in 0..=9 {
                assert!(is_palindrome(x), "{x} should be a palindrome");
            }
        }

        #[test]
        fn accepts_odd_length_palindromes() {
            assert!(is_palindrome(121));
            assert!(is_palindrome(12321));
        }

        #[test]
        fn rejects_non_palindromes() {
            assert!(!is_palindrome(123));
            assert!(!is_palindrome(1234));
        }

        #[test]
        fn rejects_all_negative_numbers() {
            assert!(!is_palindrome(-121));
            assert!(!is_palindrome(-1));
        }

        #[test]
        fn accepts_even_length_palindromes() {
            assert!(is_palindrome(1221));
            assert!(is_palindrome(9009));
        }

        #[test]
        fn rejects_numbers_ending_in_zero() {
            assert!(!is_palindrome(10));
            assert!(!is_palindrome(100));
        }
    }
}
