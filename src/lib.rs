use std::rc::Rc;

pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
    fn _traverse<'a>(root: Option<&'a TreeNode<T>>, items: &mut Vec<&'a T>) {
        if let Some(root) = root {
            TreeNode::_traverse(root.left.as_deref(), items);
            let val = &root.val;
            items.push(val);
            TreeNode::_traverse(root.right.as_deref(), items);
        }
    }

    pub fn traverse(&self) -> Vec<&T> {
        let root = Some(self);
        let mut items: Vec<&T> = Vec::new();
        TreeNode::_traverse(root, &mut items);
        items
    }
}

struct Node<T> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(self, val: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                val,
                next: self.head,
            })),
            tail: self.tail,
        }
    }

    // build a list by repeatedly pushing
    pub fn from(vec: Vec<T>) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        for item in vec.into_iter().rev() {
            list = list.push(item);
        }
        list
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inorder_traversal_test() {
        let mut root = Box::new(TreeNode::new(10));
        root.left = Some(Box::new(TreeNode::new(20)));
        root.right = Some(Box::new(TreeNode::new(13)));
        if let Some(left) = &mut root.left {
            left.left = Some(Box::new(TreeNode::new(34)));
        }
        if let Some(right) = &mut root.right {
            right.left = Some(Box::new(TreeNode::new(23)));
            if let Some(right) = &mut right.left {
                right.left = Some(Box::new(TreeNode::new(77)));
                right.right = Some(Box::new(TreeNode::new(100)));
            }
            right.right = Some(Box::new(TreeNode::new(56)));
        }
        let items = root.traverse();
        assert_eq!(vec![&34, &20, &10, &77, &23, &100, &13, &56], items);
    }
}
