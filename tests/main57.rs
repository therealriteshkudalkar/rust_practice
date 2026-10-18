//! Tests for `src/main57.rs` — Trapping Rain Water.

mod main57 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main57.rs");

    #[cfg(test)]
    mod tests {
        use super::trap;

        #[test]
        fn example_one() {
            assert_eq!(6, trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        }

        #[test]
        fn example_two() {
            assert_eq!(9, trap(vec![4, 2, 0, 3, 2, 5]));
        }

        #[test]
        fn monotonic_heights_trap_no_water() {
            assert_eq!(0, trap(vec![1, 2, 3]));
            assert_eq!(0, trap(vec![3, 2, 1]));
        }

        #[test]
        fn single_bar_traps_no_water() {
            assert_eq!(0, trap(vec![5]));
        }

        #[test]
        fn valley_between_two_bars() {
            assert_eq!(2, trap(vec![2, 0, 2]));
            assert_eq!(1, trap(vec![4, 2, 3]));
        }

        #[test]
        fn multiple_valleys_accumulate() {
            assert_eq!(8, trap(vec![3, 0, 1, 0, 3]));
        }
    }
}
