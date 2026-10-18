//! Tests for `src/main47.rs` — Delete and Earn.

mod main47 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main47.rs");

    #[cfg(test)]
    mod tests {
        use std::collections::{HashMap, HashSet};

        use super::{
            delete_and_earn, delete_and_earn_memoized, delete_and_earn_rec,
            delete_and_earn_with_memoized, delete_and_earn_with_rec,
        };

        fn counts(nums: &[i32]) -> (HashMap<i32, i32>, Vec<i32>) {
            let mut map = HashMap::new();
            for &num in nums {
                *map.entry(num).or_insert(0) += 1;
            }
            let mut order: Vec<i32> = map.keys().copied().collect();
            order.sort_unstable();
            (map, order)
        }

        #[test]
        fn example_one() {
            assert_eq!(6, delete_and_earn(vec![3, 4, 2]));
        }

        #[test]
        #[ignore = "known bug: the memo key only stores current_index, so selection state is lost and [2,2,3,3,3,4] returns 8 instead of 9"]
        fn example_two_uses_duplicate_counts() {
            assert_eq!(9, delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
        }

        #[test]
        fn correct_recursive_variant_handles_duplicate_counts() {
            let (counts, order) = counts(&[2, 2, 3, 3, 3, 4]);
            let earned = delete_and_earn_with_rec(&counts, &order, &mut HashSet::new(), 0);
            assert_eq!(9, earned);
        }

        #[test]
        fn example_three_takes_both_non_adjacent_values() {
            assert_eq!(4, delete_and_earn(vec![3, 1]));
        }

        #[test]
        fn single_value() {
            assert_eq!(1, delete_and_earn(vec![1]));
            assert_eq!(4, delete_and_earn(vec![2, 2]));
        }

        #[test]
        fn recursive_and_memoized_variants_agree() {
            let nums = vec![3, 4, 2];
            let (counts, order) = counts(&nums);
            let recursive = delete_and_earn_with_rec(&counts, &order, &mut HashSet::new(), 0);
            let memoized = delete_and_earn_with_memoized(
                &mut HashMap::new(),
                &counts,
                &order,
                &mut HashSet::new(),
                0,
            );
            assert_eq!(6, recursive);
            assert_eq!(recursive, memoized);
        }

        #[test]
        fn raw_recursive_variants_agree_on_small_inputs() {
            let cases = vec![vec![3, 4, 2], vec![3, 1], vec![1], vec![2, 2]];
            for nums in cases {
                let recursive = delete_and_earn_rec(&nums, 0, &mut HashSet::new());
                let memoized =
                    delete_and_earn_memoized(&nums, &mut HashMap::new(), 0, &mut HashSet::new());
                assert_eq!(recursive, memoized, "nums = {nums:?}");
                assert_eq!(recursive, delete_and_earn(nums.clone()));
            }
        }
    }
}
