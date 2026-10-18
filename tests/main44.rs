//! Tests for `src/main44.rs` — Find the Town Judge.

mod main44 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main44.rs");

    #[cfg(test)]
    mod tests {
        use super::find_judge;

        #[test]
        fn example_one() {
            assert_eq!(2, find_judge(2, vec![vec![1, 2]]));
        }

        #[test]
        fn example_two() {
            assert_eq!(3, find_judge(3, vec![vec![1, 3], vec![2, 3]]));
        }

        #[test]
        fn example_three_has_no_judge_because_the_candidate_trusts_someone() {
            assert_eq!(-1, find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]));
        }

        #[test]
        fn single_person_is_the_judge() {
            assert_eq!(1, find_judge(1, vec![]));
        }

        #[test]
        fn no_one_trusted_means_no_judge() {
            assert_eq!(-1, find_judge(3, vec![vec![1, 2], vec![2, 3]]));
        }

        #[test]
        fn mutual_trust_means_no_judge() {
            assert_eq!(-1, find_judge(2, vec![vec![1, 2], vec![2, 1]]));
        }

        #[test]
        fn invalid_person_outside_the_town_is_rejected() {
            assert_eq!(-1, find_judge(2, vec![vec![1, 3], vec![2, 3]]));
        }
    }
}
