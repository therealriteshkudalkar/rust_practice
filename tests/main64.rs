//! Tests for `src/main64.rs` — Permutations II.

mod main64 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main64.rs");

    #[cfg(test)]
    mod tests {
        use super::permute_unique;

        fn normalise(mut permutations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            permutations.sort();
            permutations
        }

        #[test]
        fn example_one_has_three_unique_permutations() {
            assert_eq!(
                normalise(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]),
                normalise(permute_unique(vec![1, 1, 2]))
            );
        }

        #[test]
        fn all_distinct_values_produce_all_permutations() {
            let result = normalise(permute_unique(vec![1, 2, 3]));
            assert_eq!(6, result.len());
            assert_eq!(
                normalise(vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ]),
                result
            );
        }

        #[test]
        fn all_equal_values_produce_one_permutation() {
            assert_eq!(vec![vec![1, 1, 1]], permute_unique(vec![1, 1, 1]));
        }

        #[test]
        fn two_equal_values() {
            assert_eq!(vec![vec![1, 1]], permute_unique(vec![1, 1]));
        }

        #[test]
        fn two_of_one_value_and_one_of_another() {
            assert_eq!(
                normalise(vec![vec![1, 2, 2], vec![2, 1, 2], vec![2, 2, 1]]),
                normalise(permute_unique(vec![1, 2, 2]))
            );
        }

        #[test]
        fn single_value() {
            assert_eq!(vec![vec![1]], permute_unique(vec![1]));
        }
    }
}
