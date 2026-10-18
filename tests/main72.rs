//! Tests for `src/main72.rs` — Find Subsequence of Length K With the Largest Sum.

mod main72 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main72.rs");

    #[cfg(test)]
    mod tests {
        use super::max_subsequence;

        #[test]
        fn example_one() {
            assert_eq!(vec![3, 3], max_subsequence(vec![2, 1, 3, 3], 2));
        }

        #[test]
        fn example_two_negative_values() {
            assert_eq!(vec![-1, 3, 4], max_subsequence(vec![-1, -2, 3, 4], 3));
        }

        #[test]
        fn example_three_ties_keep_original_order() {
            assert_eq!(vec![3, 4], max_subsequence(vec![3, 4, 3, 3], 2));
        }

        #[test]
        fn k_equals_one_selects_the_maximum() {
            assert_eq!(vec![3], max_subsequence(vec![1, 3, 2], 1));
        }

        #[test]
        fn k_equals_length_returns_the_original_array() {
            assert_eq!(vec![5, 4, 3, 2, 1], max_subsequence(vec![5, 4, 3, 2, 1], 5));
        }

        #[test]
        fn increasing_array_selects_the_tail() {
            assert_eq!(vec![3, 4, 5], max_subsequence(vec![1, 2, 3, 4, 5], 3));
        }

        #[test]
        fn all_negative_values_choose_the_largest_k() {
            assert_eq!(vec![-1, -2], max_subsequence(vec![-5, -1, -2, -9], 2));
        }
    }
}
