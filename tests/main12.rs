//! Tests for `src/main12.rs` — Min Cost Climbing Stairs.

mod main12 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main12.rs");

    #[cfg(test)]
    mod tests {
        use super::min_cost_climbing_stairs;

        #[test]
        fn example_one() {
            assert_eq!(15, min_cost_climbing_stairs(&vec![10, 15, 20]));
        }

        #[test]
        fn example_two() {
            let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
            assert_eq!(6, min_cost_climbing_stairs(&cost));
        }

        #[test]
        fn two_stairs_chooses_the_cheaper_start() {
            assert_eq!(1, min_cost_climbing_stairs(&vec![1, 2]));
            assert_eq!(0, min_cost_climbing_stairs(&vec![0, 0]));
        }

        #[test]
        fn top_is_past_the_last_step() {
            // Start on step 0 (2), jump to step 2 (2), then climb to the top: 4.
            assert_eq!(4, min_cost_climbing_stairs(&vec![2, 100, 2]));
        }
    }
}
