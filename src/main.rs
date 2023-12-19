#![allow(dead_code)]
#![allow(unused_variables)]

use fun1::TreeNode;

fn main() {
    let mut root = Box::new(TreeNode::new(10));
    root.left = Some(Box::new(TreeNode::new(20)));
    root.right = Some(Box::new(TreeNode::new(13)));
    if let Some(ref mut left) = root.left {
        left.left = Some(Box::new(TreeNode::new(34)));
    }
    if let Some(ref mut right) = root.right {
        right.left = Some(Box::new(TreeNode::new(23)));
        right.right = Some(Box::new(TreeNode::new(56)));
    }
    root.traverse();
}
