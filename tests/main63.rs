//! Tests for `src/main63.rs` — Permutations.

mod main63 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main63.rs");

    #[cfg(test)]
    mod tests {
        use super::permute;

        fn normalise(mut permutations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            permutations.sort();
            permutations
        }

        #[test]
        fn example_one() {
            assert_eq!(
                normalise(vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ]),
                normalise(permute(vec![1, 2, 3]))
            );
        }

        #[test]
        fn two_elements_have_two_permutations() {
            assert_eq!(
                normalise(vec![vec![1, 2], vec![2, 1]]),
                normalise(permute(vec![1, 2]))
            );
        }

        #[test]
        fn one_element_has_one_permutation() {
            assert_eq!(vec![vec![1]], permute(vec![1]));
        }

        #[test]
        fn empty_input_has_the_empty_permutation() {
            let expected: Vec<Vec<i32>> = vec![vec![]];
            let actual = permute(Vec::<i32>::new());
            assert_eq!(expected, actual);
        }

        #[test]
        fn number_of_permutations_is_factorial() {
            for n in 1..=6 {
                let values: Vec<i32> = (1..=n).collect();
                assert_eq!(
                    (1..=n).product::<i32>() as usize,
                    permute(values).len(),
                    "n = {n}"
                );
            }
        }

        #[test]
        fn every_permutation_contains_each_value_once() {
            let values = vec![4, 5, 6];
            for permutation in permute(values.clone()) {
                assert_eq!(values.len(), permutation.len());
                let mut sorted = permutation.clone();
                sorted.sort_unstable();
                assert_eq!(values, sorted);
            }
        }
    }
}
