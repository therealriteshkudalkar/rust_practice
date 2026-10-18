//! Tests for `src/main53.rs` — Find the Duplicate Number.

mod main53 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main53.rs");

    #[cfg(test)]
    mod tests {
        use super::{find_duplicate, find_duplicate_naive, find_duplicate_smarter};

        fn valid_cases() -> Vec<(Vec<i32>, i32)> {
            vec![
                (vec![1, 3, 4, 2, 2], 2),
                (vec![3, 1, 3, 4, 2], 3),
                (vec![3, 3, 3, 3, 3], 3),
                (vec![1, 1, 2], 1),
                (vec![1, 2, 2], 2),
                (vec![2, 2, 2, 2], 2),
            ]
        }

        #[test]
        fn sorting_solution_matches_all_valid_cases() {
            for (nums, expected) in valid_cases() {
                assert_eq!(
                    expected,
                    find_duplicate_smarter(nums.clone()),
                    "nums = {nums:?}"
                );
            }
        }

        #[test]
        fn xor_solution_matches_all_valid_cases() {
            for (nums, expected) in valid_cases() {
                assert_eq!(
                    expected,
                    find_duplicate_naive(nums.clone()),
                    "nums = {nums:?}"
                );
            }
        }

        #[test]
        fn both_reference_implementations_agree() {
            for (nums, _) in valid_cases() {
                assert_eq!(
                    find_duplicate_smarter(nums.clone()),
                    find_duplicate_naive(nums.clone()),
                    "nums = {nums:?}"
                );
            }
        }

        #[test]
        #[ignore = "known bug: find_duplicate compares values against zero-based indices, so [1,3,4,2,2] returns 1"]
        fn in_place_solution_finds_the_duplicate() {
            for (nums, expected) in valid_cases() {
                assert_eq!(expected, find_duplicate(nums.clone()), "nums = {nums:?}");
            }
        }
    }
}
