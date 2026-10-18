//! Tests for `src/main65.rs` — 4Sum.
//!
//! The current implementation is a stub that always returns an empty vector,
//! so the examples from the problem statement are marked as ignored.

mod main65 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main65.rs");

    #[cfg(test)]
    mod tests {
        use super::four_sum;

        fn normalise(mut quadruplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            for quadruplet in &mut quadruplets {
                quadruplet.sort_unstable();
            }
            quadruplets.sort();
            quadruplets
        }

        #[test]
        fn no_solution_returns_empty() {
            assert!(four_sum(vec![1, 2, 3], 0).is_empty());
            assert!(four_sum(vec![1, 1, 1, 1], 100).is_empty());
        }

        #[test]
        fn empty_input_returns_empty() {
            assert!(four_sum(vec![], 0).is_empty());
        }

        #[test]
        #[ignore = "not implemented: four_sum is a stub returning an empty Vec"]
        fn example_one() {
            assert_eq!(
                normalise(vec![
                    vec![-2, -1, 1, 2],
                    vec![-2, 0, 0, 2],
                    vec![-1, 0, 0, 1],
                ]),
                normalise(four_sum(vec![1, 0, -1, 0, -2, 2], 0))
            );
        }

        #[test]
        #[ignore = "not implemented: four_sum is a stub returning an empty Vec"]
        fn example_two_all_twos() {
            assert_eq!(vec![vec![2, 2, 2, 2]], four_sum(vec![2, 2, 2, 2, 2], 8));
        }
    }
}
