//! Tests for `src/main46.rs` — House Robber.

mod main46 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main46.rs");

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;

        use super::{rob, rob_houses, rob_houses_memoized};

        #[test]
        fn example_one() {
            assert_eq!(4, rob(vec![1, 2, 3, 1]));
        }

        #[test]
        fn example_two() {
            assert_eq!(12, rob(vec![2, 7, 9, 3, 1]));
        }

        #[test]
        fn two_houses_that_cannot_both_be_robbed() {
            assert_eq!(2, rob(vec![2, 1]));
        }

        #[test]
        fn single_house() {
            assert_eq!(1, rob(vec![1]));
            assert_eq!(0, rob(vec![0]));
        }

        #[test]
        fn empty_street_earns_nothing() {
            assert_eq!(0, rob(vec![]));
        }

        #[test]
        fn memoized_and_plain_recursive_versions_agree() {
            let cases = vec![
                vec![1, 2, 3, 1],
                vec![2, 7, 9, 3, 1],
                vec![2, 1, 1, 2],
                vec![2, 1],
                vec![1],
            ];
            for houses in cases {
                let recursive = rob_houses(&houses, 0, false);
                let memoized = rob_houses_memoized(&houses, &mut HashMap::new(), 0, false);
                assert_eq!(recursive, memoized, "houses = {houses:?}");
                assert_eq!(recursive, rob(houses.clone()));
            }
        }
    }
}
