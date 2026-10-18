//! Tests for `src/main48.rs` — Reverse Integer.

mod main48 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main48.rs");

    #[cfg(test)]
    mod tests {
        use super::reverse;

        #[test]
        fn example_one() {
            assert_eq!(321, reverse(123));
        }

        #[test]
        fn example_two_keeps_the_sign() {
            assert_eq!(-321, reverse(-123));
        }

        #[test]
        fn example_three_drops_the_leading_zero() {
            assert_eq!(21, reverse(120));
        }

        #[test]
        fn zero_reverses_to_zero() {
            assert_eq!(0, reverse(0));
        }

        #[test]
        fn overflow_returns_zero() {
            assert_eq!(0, reverse(1_534_236_469));
            assert_eq!(0, reverse(-1_534_236_469));
            assert_eq!(0, reverse(2_147_483_647));
        }

        #[test]
        fn i32_min_and_max_are_special_cased() {
            assert_eq!(0, reverse(i32::MIN));
            assert_eq!(0, reverse(i32::MAX));
        }

        #[test]
        fn value_that_still_fits_is_reversed() {
            assert_eq!(2_147_483_641, reverse(1_463_847_412));
        }
    }
}
