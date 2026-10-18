//! Tests for `src/main59.rs` — Minimum Depth of Binary Tree.

mod main59 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main59.rs");

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::{min_depth, TreeNode};

        fn node(
            value: i32,
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            Some(TreeNode::new(value, left, right))
        }

        #[test]
        fn empty_tree_has_depth_zero() {
            assert_eq!(0, min_depth(None));
        }

        #[test]
        fn single_node_has_depth_one() {
            assert_eq!(1, min_depth(node(1, None, None)));
        }

        #[test]
        fn example_one() {
            let root = node(
                3,
                node(9, None, None),
                node(20, node(15, None, None), node(7, None, None)),
            );
            assert_eq!(2, min_depth(root));
        }

        #[test]
        fn missing_left_child_does_not_count_as_a_leaf() {
            let root = node(
                2,
                None,
                node(3, None, node(4, None, node(5, None, node(6, None, None)))),
            );
            assert_eq!(5, min_depth(root));
        }

        #[test]
        fn left_skewed_tree() {
            let root = node(1, node(2, node(3, None, None), None), None);
            assert_eq!(3, min_depth(root));
        }

        #[test]
        fn balanced_tree_uses_the_shorter_side() {
            let root = node(1, node(2, node(4, None, None), None), node(3, None, None));
            assert_eq!(2, min_depth(root));
        }
    }
}
