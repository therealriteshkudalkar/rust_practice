//! Tests for `src/main33.rs` — Majority Element.

mod main33 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main33.rs");

    #[cfg(test)]
    mod tests {
        use super::{majority_element, majority_element_optimized};

        #[test]
        fn example_one() {
            assert_eq!(3, majority_element(vec![3, 2, 3]));
        }

        #[test]
        fn example_two() {
            assert_eq!(2, majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
        }

        #[test]
        fn single_element_is_the_majority() {
            assert_eq!(1, majority_element(vec![1]));
        }

        #[test]
        fn optimized_version_matches_hash_map_version() {
            let cases = vec![
                vec![3, 2, 3],
                vec![2, 2, 1, 1, 1, 2, 2],
                vec![1],
                vec![1, 1, 2],
                vec![1, 2, 1, 3, 1, 4, 1],
            ];
            for nums in cases {
                assert_eq!(
                    majority_element(nums.clone()),
                    majority_element_optimized(nums.clone()),
                    "nums = {nums:?}"
                );
            }
        }

        #[test]
        fn optimized_version_handles_a_simple_majority() {
            assert_eq!(1, majority_element_optimized(vec![1, 1, 2]));
            assert_eq!(7, majority_element_optimized(vec![7, 7, 7, 2]));
        }
    }
}
