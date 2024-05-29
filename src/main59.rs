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

pub fn main59() {
    let tree_root = TreeNode::new(
        3,
        Some(TreeNode::new(9, None, None)),
        Some(TreeNode::new(
            20,
            Some(TreeNode::new(15, None, None)),
            Some(TreeNode::new(7, None, None)),
        )),
    );
    println!("min_depth: {}", min_depth(Some(tree_root)));

    let tree_root = TreeNode::new(
        2,
        None,
        Some(TreeNode::new(
            3,
            None,
            Some(TreeNode::new(
                4,
                None,
                Some(TreeNode::new(5, None, Some(TreeNode::new(6, None, None)))),
            )),
        )),
    );
    println!("min_depth: {}", min_depth(Some(tree_root)));
}
