//! Tests for `src/main1.rs` — Two Sum.
//!
//! The exercise lives in a binary source file, so it is included as a module.
//! This keeps a one-to-one mapping between `src/mainN.rs` and `tests/mainN.rs`.

mod main1 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main1.rs");

    #[cfg(test)]
    mod tests {
        use super::two_sum;

        #[test]
        fn example_one_returns_matching_indices() {
            assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        }

        #[test]
        fn example_two_uses_values_not_positions() {
            assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        }

        #[test]
        fn returns_empty_vec_when_no_pair_matches() {
            assert!(two_sum(vec![1, 2, 3], 7).is_empty());
        }

        #[test]
        fn supports_negative_numbers_and_duplicates() {
            let mut result = two_sum(vec![-3, 4, 3, 90], 0);
            result.sort_unstable();
            assert_eq!(vec![0, 2], result);
        }

        #[test]
        fn returns_the_first_pair_found_for_duplicates() {
            assert_eq!(vec![0, 1], two_sum(vec![3, 3, 4, 3], 6));
        }
    }
}
