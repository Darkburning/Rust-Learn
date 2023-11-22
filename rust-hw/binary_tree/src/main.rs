use std::cell::RefCell;
use std::rc::Rc;
#[derive(PartialEq)]
enum Side {
    Left,
    Right,
}
#[derive(PartialEq, Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }

    // 插入左右子树
    fn insert(&mut self, side: Side, value: i32) -> Rc<RefCell<TreeNode>> {
        let new_node = TreeNode::new(value);

        if side == Side::Left {
            self.left = Some(Rc::clone(&new_node));
        } else {
            self.right = Some(Rc::clone(&new_node));
        }
        new_node
    }

    fn modification(&mut self, value: i32) {
        self.value = value;
    }
}

fn in_order_traversal(root: &TreeNode) {
    if let Some(inner) = &root.left {
        in_order_traversal(&inner.borrow());
    }
    println!("{} ", root.value);

    if let Some(inner) = &root.right {
        in_order_traversal(&inner.borrow());
    }
}

fn main() {
    let root = TreeNode::new(1);
    let l1 = root.borrow_mut().insert(Side::Left, 100);
    let r1 = root.borrow_mut().insert(Side::Right, 3);

    l1.borrow_mut().insert(Side::Left, 4);
    l1.borrow_mut().insert(Side::Right, 5);
    r1.borrow_mut().insert(Side::Left, 6);
    in_order_traversal(&root.borrow());
    println!("--- after change ---");
    l1.borrow_mut().modification(2);
    in_order_traversal(&root.borrow());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tree_creation() {
        let root = TreeNode::new(1);
        assert_eq!(root.borrow().value, 1);
        assert_eq!(root.borrow().left, None);
        assert_eq!(root.borrow().right, None);
    }

    #[test]
    fn test_tree_modification() {
        let root = TreeNode::new(1);
        root.borrow_mut().modification(2);
        assert_eq!(root.borrow().value, 2);
    }

    #[test]
    fn test_in_order_traversal() {}
}
