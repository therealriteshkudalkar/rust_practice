//! Tests for `src/main25.rs` — Greatest Common Divisor of Strings.

mod main25 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main25.rs");

    #[cfg(test)]
    mod tests {
        use super::{find_factors_of_self, gcd_of_strings, is_a_factor};

        #[test]
        fn factor_check_accepts_repeated_base_strings() {
            assert!(is_a_factor("ABAB", "AB"));
            assert!(is_a_factor("ABCABC", "ABC"));
            assert!(is_a_factor("AAAA", "A"));
            assert!(is_a_factor("ABABAB", "ABABAB"));
        }

        #[test]
        fn factor_check_rejects_non_factors() {
            assert!(!is_a_factor("ABC", "AB"));
            assert!(!is_a_factor("ABAB", "ABABAB"));
            assert!(!is_a_factor("ABAB", "BA"));
        }

        #[test]
        fn find_factors_returns_self_for_a_repeated_pattern_string() {
            let factors = find_factors_of_self("ABAB");
            assert!(factors.contains(&"AB"));
            assert!(factors.contains(&"ABAB"));
        }

        #[test]
        fn example_one() {
            assert_eq!(
                "ABC",
                gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
            );
        }

        #[test]
        fn example_two() {
            assert_eq!(
                "AB",
                gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
            );
        }

        #[test]
        fn strings_with_no_common_base_return_empty_string() {
            assert_eq!("", gcd_of_strings("LEET".to_string(), "CODE".to_string()));
        }

        #[test]
        fn arguments_can_be_swapped() {
            assert_eq!(
                "AB",
                gcd_of_strings("ABAB".to_string(), "ABABAB".to_string())
            );
        }

        #[test]
        fn identical_strings_are_their_own_gcd() {
            assert_eq!("A", gcd_of_strings("A".to_string(), "A".to_string()));
        }
    }
}
