//! Tests for `src/main41.rs` — 3Sum.
//!
//! The source has a sort-and-two-pointer solution plus two alternative
//! implementations. Triplet order from the HashSet-based versions is not stable,
//! so the tests normalise both the inner triplets and the outer list.

mod main41 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main41.rs");

    #[cfg(test)]
    mod tests {
        use super::{three_sum, three_sum_faster, three_sum_naive, two_sum_fast};

        fn normalise(mut triplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            for triplet in &mut triplets {
                triplet.sort_unstable();
            }
            triplets.sort();
            triplets
        }

        #[test]
        fn two_sum_helper_returns_indices() {
            assert_eq!(vec![0, 1], two_sum_fast(&vec![2, 7, 11, 15], 9));
            assert_eq!(vec![1, 2], two_sum_fast(&vec![3, 2, 4], 6));
            assert!(two_sum_fast(&vec![1, 2, 3], 100).is_empty());
            assert_eq!(vec![0, 1], two_sum_fast(&vec![3, 3], 6));
        }

        #[test]
        fn example_one() {
            assert_eq!(
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
                normalise(three_sum(vec![-1, 0, 1, 2, -1, -4]))
            );
        }

        #[test]
        fn example_two_has_no_triplet() {
            assert!(three_sum(vec![0, 1, 1]).is_empty());
        }

        #[test]
        fn example_three_is_all_zeroes() {
            assert_eq!(vec![vec![0, 0, 0]], normalise(three_sum(vec![0, 0, 0])));
        }

        #[test]
        fn two_negative_numbers_can_pair_with_a_positive() {
            assert_eq!(
                vec![vec![-2, 0, 2], vec![-2, 1, 1]],
                normalise(three_sum(vec![-2, 0, 1, 1, 2]))
            );
        }

        #[test]
        fn duplicate_triplets_are_only_returned_once() {
            let result = normalise(three_sum(vec![-1, -1, -1, 2, 2, 2]));
            assert_eq!(vec![vec![-1, -1, 2]], result);
        }

        #[test]
        fn naive_version_agrees_on_examples() {
            let cases = vec![
                vec![-1, 0, 1, 2, -1, -4],
                vec![0, 1, 1],
                vec![0, 0, 0],
                vec![-2, 0, 1, 1, 2],
                vec![-1, 0, 1],
            ];
            for nums in cases {
                assert_eq!(
                    normalise(three_sum(nums.clone())),
                    normalise(three_sum_naive(nums.clone())),
                    "nums = {nums:?}"
                );
            }
        }

        #[test]
        fn faster_version_handles_small_inputs() {
            assert!(three_sum_faster(vec![0, 1, 1]).is_empty());
            assert_eq!(
                vec![vec![0, 0, 0]],
                normalise(three_sum_faster(vec![0, 0, 0]))
            );
            assert_eq!(
                vec![vec![-1, 0, 1]],
                normalise(three_sum_faster(vec![-1, 0, 1]))
            );
        }
    }
}
