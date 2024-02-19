use std::cmp;

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    // Constructing a binary tree
    let mut root = TreeNode::new(3);
    let mut left_child = TreeNode::new(9);
    let right_child = TreeNode::new(20);
    let mut right_child_left = TreeNode::new(15);
    let right_child_right = TreeNode::new(7);

    right_child_left.right = Some(Box::new(right_child_right));
    right_child.left = Some(Box::new(right_child_left));
    root.left = Some(Box::new(left_child));
    root.right = Some(Box::new(right_child));

    // Finding the maximum depth of the binary tree
    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the binary tree: {}", depth); // Output: 3
}
