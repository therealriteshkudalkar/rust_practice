use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    return match root {
        None => 0,
        Some(root_node) => {
            let left_depth = match &root_node.borrow().left {
                None => 0,
                Some(left_node) => max_depth(Some(Rc::clone(left_node)))
            };
            let right_depth = match &root_node.borrow().right {
                None => 0,
                Some(right_node) => max_depth(Some(Rc::clone(right_node)))
            };
            1 + max(left_depth, right_depth)
        }
    };
}

pub fn main56() {
    let tree_root = TreeNode::new(
        1,
        Some(TreeNode::new(
            2,
            Some(TreeNode::new(4, None, None)),
            Some(TreeNode::new(5, None, Some(TreeNode::new(6, None, None)))),
        )),
        Some(TreeNode::new(3, Some(TreeNode::new(7, None, None)), None)),
    );

    println!("max_depth: {}", max_depth(Some(tree_root)));
}
