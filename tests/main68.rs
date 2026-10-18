//! Tests for `src/main68.rs` — Valid Number.
//!
//! This is a hand-written DFA; the test lists are taken from the LeetCode
//! examples and a few edge cases around the decimal/exponent states.

mod main68 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main68.rs");

    #[cfg(test)]
    mod tests {
        use super::is_number;

        fn check(expected: bool, inputs: &[&str]) {
            for input in inputs {
                assert_eq!(
                    expected,
                    is_number(input.to_string()),
                    "is_number({input:?})"
                );
            }
        }

        #[test]
        fn accepted_numbers() {
            check(
                true,
                &[
                    "2",
                    "0089",
                    "-0.1",
                    "+3.14",
                    "4.",
                    "-.9",
                    "2e10",
                    "-90E3",
                    "3e+7",
                    "+6e-1",
                    "53.5e93",
                    "-123.456e789",
                    "731671765313",
                    "+13.75e-45",
                    ".8",
                    "46.e3",
                ],
            );
        }

        #[test]
        fn rejected_numbers() {
            check(
                false,
                &[
                    "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "4e+", ".-4",
                    "1e.", " ", "1 2", "Infinity", "NaN",
                ],
            );
        }

        #[test]
        fn empty_string_is_not_a_number() {
            assert!(!is_number(String::new()));
        }

        #[test]
        fn exponent_with_optional_sign_and_digits() {
            check(true, &["1e1", "1E1", "1e+1", "1e-1", "0e0"]);
            check(false, &["1e", "1e+", "1e-", "e1", "1ee1"]);
        }

        #[test]
        fn decimal_points() {
            check(true, &[".5", "5.", "0.0", "-0.", "+.5"]);
            check(false, &[".", "+.", "-.", "..", "1..2"]);
        }
    }
}
