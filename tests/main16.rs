//! Tests for `src/main16.rs` — Get Maximum in Generated Array.

mod main16 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main16.rs");

    #[cfg(test)]
    mod tests {
        use super::get_maximum_generated;

        #[test]
        fn base_cases() {
            assert_eq!(0, get_maximum_generated(0));
            assert_eq!(1, get_maximum_generated(1));
        }

        #[test]
        fn leetcode_small_examples() {
            assert_eq!(1, get_maximum_generated(2));
            assert_eq!(2, get_maximum_generated(3));
            assert_eq!(3, get_maximum_generated(7));
        }

        #[test]
        fn maximum_is_never_below_one_for_positive_n() {
            for n in 1..=50 {
                assert!(get_maximum_generated(n) >= 1, "n = {n}");
            }
        }

        #[test]
        fn result_is_non_decreasing_as_n_grows() {
            let mut previous = 0;
            for n in 0..=50 {
                let current = get_maximum_generated(n);
                assert!(current >= previous, "n = {n}");
                previous = current;
            }
        }
    }
}
