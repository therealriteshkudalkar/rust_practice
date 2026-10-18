//! Tests for `src/main5.rs` — Distribute Money to Maximum Children.
//!
//! LeetCode constraints: 1 <= money <= 200 and 2 <= children <= 30.

mod main5 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main5.rs");

    #[cfg(test)]
    mod tests {
        use super::{dist_money, find_index};

        #[test]
        fn example_one() {
            assert_eq!(1, dist_money(20, 3));
        }

        #[test]
        fn example_two() {
            assert_eq!(2, dist_money(16, 2));
        }

        #[test]
        fn exact_eight_per_child() {
            assert_eq!(3, dist_money(24, 3));
        }

        #[test]
        fn four_dollars_left_over_must_be_avoided() {
            // 8 + 8 + 8 + 4 is invalid, so only two children can get 8.
            assert_eq!(2, dist_money(28, 4));
        }

        #[test]
        fn twelve_for_two_children_cannot_produce_an_eight() {
            // 8 + 4 is invalid and 7 + 5 has no 8.
            assert_eq!(0, dist_money(12, 2));
        }

        #[test]
        fn too_little_money_is_impossible() {
            assert_eq!(-1, dist_money(5, 6));
            assert_eq!(-1, dist_money(1, 2));
        }

        #[test]
        fn more_than_eight_per_child_leaves_one_child_without_eight() {
            assert_eq!(9, dist_money(100, 10));
        }

        #[test]
        fn find_index_prefers_an_existing_eight_then_avoids_four() {
            // All amounts are already 8, so add the remainder to the last child.
            assert_eq!(2, find_index(&vec![8, 8, 8], 5));
            // A child with 3 must not become 4, so pick the 8 instead.
            assert_eq!(0, find_index(&vec![8, 3], 1));
        }
    }
}
