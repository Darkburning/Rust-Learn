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

    // 当前节点插入左/右节点
    fn insert_node(&mut self, side: Side, value: i32) -> Rc<RefCell<TreeNode>> {
        let new_node = TreeNode::new(value);

        if side == Side::Left {
            self.left = Some(Rc::clone(&new_node));
        } else {
            self.right = Some(Rc::clone(&new_node));
        }
        new_node
    }

    // 当前节点插入左/右子树
    fn insert_tree(&mut self, side: Side, tree: &Rc<RefCell<TreeNode>>) {
        if side == Side::Left {
            self.left = Some(Rc::clone(tree));
        } else {
            self.right = Some(Rc::clone(tree));
        }
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
    // 插入节点的方式建树
    println!("---------insert_node method---------");
    println!("---------before modification---------");
    let root = TreeNode::new(1);
    let l1 = root.borrow_mut().insert_node(Side::Left, 100);
    let r1 = root.borrow_mut().insert_node(Side::Right, 3);

    l1.borrow_mut().insert_node(Side::Left, 4);
    l1.borrow_mut().insert_node(Side::Right, 5);
    r1.borrow_mut().insert_node(Side::Left, 6);
    in_order_traversal(&root.borrow());
    println!("---------after modification---------");

    l1.borrow_mut().modification(2);

    in_order_traversal(&root.borrow());

    // 插入子树的方式建树
    println!("---------insert_tree method---------");
    let left_root = TreeNode::new(2);
    left_root.borrow_mut().insert_node(Side::Left, 4);
    left_root.borrow_mut().insert_node(Side::Right, 5);

    let right_root = TreeNode::new(3);
    right_root.borrow_mut().insert_node(Side::Left, 6);

    let tree_root = TreeNode::new(1);
    tree_root.borrow_mut().insert_tree(Side::Left, &left_root);
    tree_root.borrow_mut().insert_tree(Side::Right, &right_root);
    in_order_traversal(&tree_root.borrow());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tree_creation() {
        let root = TreeNode::new(1);
        let l1 = root.borrow_mut().insert_node(Side::Left, 2);
        let r1 = root.borrow_mut().insert_node(Side::Right, 3);

        let l1l = l1.borrow_mut().insert_node(Side::Left, 4);
        let l1r = l1.borrow_mut().insert_node(Side::Right, 5);
        let r1l = r1.borrow_mut().insert_node(Side::Left, 6);

        assert_eq!(root.borrow().value, 1);
        assert_eq!(l1.borrow().value, 2);
        assert_eq!(r1.borrow().value, 3);
        assert_eq!(l1l.borrow().value, 4);
        assert_eq!(l1r.borrow().value, 5);
        assert_eq!(r1l.borrow().value, 6);
    }

    #[test]
    fn test_tree_modification() {
        let root = TreeNode::new(1);
        assert_eq!(root.borrow().value, 1);

        root.borrow_mut().modification(2);
        assert_eq!(root.borrow().value, 2);
    }

    #[test]
    fn test_in_order_traversal() {
        // 创建二叉树
        let root = TreeNode::new(1);
        let l1 = root.borrow_mut().insert_node(Side::Left, 2);
        let r1 = root.borrow_mut().insert_node(Side::Right, 3);

        l1.borrow_mut().insert_node(Side::Left, 4);
        l1.borrow_mut().insert_node(Side::Right, 5);
        r1.borrow_mut().insert_node(Side::Left, 6);

        // 验证中序遍历结果
        let mut output = Vec::new();
        in_order_traversal(&root.borrow(), &mut output);

        let expected_output = vec![4, 2, 5, 1, 6, 3];
        assert_eq!(output, expected_output);
    }

    // 辅助函数：中序遍历并将结果存储在向量中
    fn in_order_traversal(root: &TreeNode, output: &mut Vec<i32>) {
        if let Some(inner) = &root.left {
            in_order_traversal(&inner.borrow(), output);
        }
        output.push(root.value);

        if let Some(inner) = &root.right {
            in_order_traversal(&inner.borrow(), output);
        }
    }
}
