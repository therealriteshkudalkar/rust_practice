//! Tests for `src/main7.rs` — Climbing Stairs.

mod main7 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main7.rs");

    #[cfg(test)]
    mod tests {
        use super::climb_stairs;

        #[test]
        fn base_cases() {
            assert_eq!(1, climb_stairs(1));
            assert_eq!(2, climb_stairs(2));
        }

        #[test]
        fn small_staircases() {
            assert_eq!(3, climb_stairs(3));
            assert_eq!(5, climb_stairs(4));
            assert_eq!(8, climb_stairs(5));
            assert_eq!(89, climb_stairs(10));
        }

        #[test]
        fn largest_leetcode_input_fits_in_i32() {
            assert_eq!(1_836_311_903, climb_stairs(45));
        }

        #[test]
        fn zero_stairs_counts_as_one_empty_way() {
            assert_eq!(1, climb_stairs(0));
        }
    }
}
