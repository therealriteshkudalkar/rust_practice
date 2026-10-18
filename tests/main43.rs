//! Tests for `src/main43.rs` — Find Eventual Safe States.

mod main43 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main43.rs");

    #[cfg(test)]
    mod tests {
        use super::eventual_safe_nodes;

        #[test]
        fn example_one() {
            let graph = vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![],
            ];
            assert_eq!(vec![2, 4, 5, 6], eventual_safe_nodes(graph));
        }

        #[test]
        fn example_two() {
            let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
            assert_eq!(vec![4], eventual_safe_nodes(graph));
        }

        #[test]
        fn every_node_on_a_path_to_a_terminal_is_safe() {
            let graph = vec![vec![1], vec![2], vec![3], vec![]];
            assert_eq!(vec![0, 1, 2, 3], eventual_safe_nodes(graph));
        }

        #[test]
        fn all_nodes_are_terminal_or_lead_to_terminal_nodes() {
            let graph = vec![vec![], vec![], vec![]];
            assert_eq!(vec![0, 1, 2], eventual_safe_nodes(graph));
        }

        #[test]
        fn single_cycle_has_no_safe_nodes() {
            let graph = vec![vec![1], vec![0]];
            assert!(eventual_safe_nodes(graph).is_empty());
        }

        #[test]
        fn empty_graph_has_no_safe_nodes() {
            assert!(eventual_safe_nodes(vec![]).is_empty());
        }
    }
}
