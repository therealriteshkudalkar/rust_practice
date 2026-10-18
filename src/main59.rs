use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }
}

fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(root_node) => {
            match (&root_node.borrow().left, &root_node.borrow().right) {
                (None, None) => 1,
                (Some(left_node), Some(right_node)) => {
                    1 + min(
                        min_depth(Some(Rc::clone(left_node))),
                        min_depth(Some(Rc::clone(right_node))),
                    )
                }
                (Some(left_node), None) => 1 + min_depth(Some(Rc::clone(left_node))),
                (None, Some(right_node)) => 1 + min_depth(Some(Rc::clone(right_node))),
            }
        }
    }
}
