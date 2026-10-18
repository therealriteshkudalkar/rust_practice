//! Tests for `src/main13.rs` — N-th Tribonacci Number.

mod main13 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main13.rs");

    #[cfg(test)]
    mod tests {
        use super::tribonacci;

        #[test]
        fn base_cases() {
            assert_eq!(0, tribonacci(0));
            assert_eq!(1, tribonacci(1));
            assert_eq!(1, tribonacci(2));
        }

        #[test]
        fn early_sequence_values() {
            let expected = [0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149];
            for (n, value) in expected.iter().enumerate() {
                assert_eq!(*value, tribonacci(n as i32), "n = {n}");
            }
        }

        #[test]
        fn largest_leetcode_input_fits_in_i32() {
            assert_eq!(2_082_876_103, tribonacci(37));
        }

        #[test]
        fn values_follow_the_recurrence() {
            for n in 3..=25 {
                assert_eq!(
                    tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
                    tribonacci(n),
                    "n = {n}"
                );
            }
        }
    }
}
