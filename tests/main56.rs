//! Tests for `src/main56.rs` — Maximum Depth of Binary Tree.

mod main56 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main56.rs");

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::{max_depth, TreeNode};

        fn node(
            value: i32,
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            Some(TreeNode::new(value, left, right))
        }

        #[test]
        fn empty_tree_has_depth_zero() {
            assert_eq!(0, max_depth(None));
        }

        #[test]
        fn single_node_has_depth_one() {
            assert_eq!(1, max_depth(node(1, None, None)));
        }

        #[test]
        fn example_tree() {
            let root = node(
                3,
                node(9, None, None),
                node(20, node(15, None, None), node(7, None, None)),
            );
            assert_eq!(3, max_depth(root));
        }

        #[test]
        fn left_skewed_tree() {
            let root = node(1, node(2, node(3, node(4, None, None), None), None), None);
            assert_eq!(4, max_depth(root));
        }

        #[test]
        fn right_skewed_tree() {
            let root = node(1, None, node(2, None, node(3, None, node(4, None, None))));
            assert_eq!(4, max_depth(root));
        }

        #[test]
        fn tree_node_constructor_exposes_its_value() {
            let root = TreeNode::new(42, None, None);
            assert_eq!(42, root.borrow().val);
            assert!(root.borrow().left.is_none());
            assert!(root.borrow().right.is_none());
        }
    }
}
