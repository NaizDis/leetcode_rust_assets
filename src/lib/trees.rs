//Definition Of Binary Tree
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use super::stack;
type TreeLink = Option<Rc<RefCell<TreeNode>>>;
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue: Vec<TreeLink> = vec![];
    queue.push(root.clone());
    while let Some(x) = queue.pop() {
        if let Some(n) = x {
            let TreeNode { left, right, .. } = &mut *n.borrow_mut();
            mem::swap(left, right);
            queue.push(left.clone());
            queue.push(right.clone());
        }
    }
    root
}
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.unwrap();
    let mut node = node.borrow_mut();
    let left = self::max_depth(node.left.take());
    let right = self::max_depth(node.right.take());
    left.max(right) + 1
}
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut res = vec![true];
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<bool>) -> i32 {
        match root {
            Some(root) => {
                let left = dfs(root.borrow().left.clone(), res);
                if !res[0] {
                    return 0;
                }
                let right = dfs(root.borrow().right.clone(), res);
                if (left - right).abs() > 1 {
                    res[0] = false;
                    return 0;
                }
                left.max(right) + 1
            }
            None => 0,
        }
    }
    dfs(root, res.as_mut());
    res[0]
}
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_d = vec![0];
    fn height(root: Option<Rc<RefCell<TreeNode>>>, mxd: &mut Vec<i32>) -> i32 {
        match root {
            Some(root) => {
                let left_h = height(root.borrow().left.clone(), mxd);
                let right_h = height(root.borrow().right.clone(), mxd);
                let dia = left_h + right_h;
                mxd[0] = mxd[0].max(dia);
                left_h.max(right_h) + 1
            }
            None => 0,
        }
    }
    height(root, max_d.as_mut());
    max_d[0]
}
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn balanced(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                balanced(p.borrow().left.clone(), q.borrow().left.clone())
                    && balanced(p.borrow().right.clone(), q.borrow().right.clone())
            }
        }
    }
    balanced(p, q)
}
