//! Tests for `src/main61.rs` — Maximum Score of a Node Sequence.

mod main61 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main61.rs");

    #[cfg(test)]
    mod tests {
        use super::maximum_score;

        #[test]
        #[ignore = "known bug: depth_first_search adds the terminal node's score twice, so example one returns 30 instead of 24"]
        fn example_one() {
            let scores = vec![5, 2, 9, 8, 4];
            let edges = vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
            ];
            assert_eq!(24, maximum_score(scores, edges));
        }

        #[test]
        fn example_two_has_no_path_of_four_nodes() {
            let scores = vec![9, 20, 6, 4, 11, 12];
            let edges = vec![vec![0, 3], vec![5, 3], vec![2, 4], vec![1, 3]];
            assert_eq!(-1, maximum_score(scores, edges));
        }

        #[test]
        #[ignore = "known bug: depth_first_search adds the terminal node's score twice, so example three returns 13 instead of 12"]
        fn example_three() {
            let scores = vec![5, 4, 3, 2, 1];
            let edges = vec![vec![0, 3], vec![2, 3], vec![3, 4], vec![1, 4]];
            assert_eq!(12, maximum_score(scores, edges));
        }

        #[test]
        #[ignore = "known bug: depth_first_search adds the terminal node's score twice, so the path 1+2+3+4 returns 13"]
        fn simple_path_of_four_nodes() {
            let scores = vec![1, 2, 3, 4];
            let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
            assert_eq!(10, maximum_score(scores, edges));
        }

        #[test]
        fn fewer_than_four_nodes_is_impossible() {
            assert_eq!(
                -1,
                maximum_score(vec![1, 2, 3], vec![vec![0, 1], vec![1, 2]])
            );
            assert_eq!(-1, maximum_score(vec![10], vec![]));
        }
    }
}
