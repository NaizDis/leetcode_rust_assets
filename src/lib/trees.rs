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
use std::collections::VecDeque;
use std::num::NonZero;
use std::rc::Rc;
use std::{mem, usize};

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
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn same(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                same(p.borrow().left.clone(), q.borrow().right.clone())
                    && same(p.borrow().right.clone(), q.borrow().left.clone())
            }
        }
    }
    same(root.clone(), root)
}
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    if let Some(n) = root {
        let node = n.borrow();
        if node.left.is_none() && node.right.is_none() && node.val == sum {
            return true;
        }
        return has_path_sum(node.left.clone(), sum - node.val)
            || has_path_sum(node.right.clone(), sum - node.val);
    }
    false
}
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if root == sub_root {
        return true;
    }
    if let Some(node) = root {
        let node = node.borrow();
        is_subtree(node.left.clone(), sub_root.clone())
            || is_subtree(node.right.clone(), sub_root.clone())
    } else {
        return false;
    }
}
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut que = VecDeque::from([root.clone()]);
    let mut res: Vec<Vec<i32>> = vec![];
    while !que.is_empty() {
        let mut level = vec![];
        let n = que.len();
        for _ in 0..n {
            let crr = que.pop_front().unwrap();
            if let Some(node) = crr {
                let node = node.borrow();
                level.push(node.val);
                if node.left.is_some() {
                    que.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    que.push_back(node.right.clone());
                }
            }
        }
        res.push(level);
    }
    res
}
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut res_arr = vec![];
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(p) = root {
            let node = p.borrow();
            dfs(node.left.clone(), arr);
            arr.push(node.val);
            dfs(node.right.clone(), arr);
        }
    }
    dfs(root, res_arr.as_mut());
    res_arr[(k - 1) as usize]
}
pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut prev = vec![-1];
    let mut min_dist = vec![i32::MAX];
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, min: &mut Vec<i32>, prev: &mut Vec<i32>) {
        if let Some(p) = root {
            let node = p.borrow();
            dfs(node.left.clone(), min, prev);
            if !prev[0] != -1 {
                min[0] = min[0].min((node.val - prev[0]).abs());
            }
            prev[0] = node.val;
            dfs(node.right.clone(), min, prev);
        }
    }
    dfs(root, min_dist.as_mut(), prev.as_mut());
    min_dist[0]
}
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(p) = root {
            let node = p.borrow();
            if node.val as i64 <= min || node.val as i64 >= max {
                return false;
            }
            return dfs(node.left.clone(), min, node.val as i64)
                && dfs(node.right.clone(), node.val as i64, max);
        }
        return true;
    }
    dfs(root, i64::MIN, i64::MAX)
}
use std::cmp::Ordering;
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;
    let p = p.unwrap().borrow().val;
    let q = q.unwrap().borrow().val;

    while let Some(n) = cur {
        let val = n.borrow().val;
        match (val.cmp(&p), val.cmp(&q)) {
            (Ordering::Greater, Ordering::Greater) => cur = n.borrow().left.clone(),
            (Ordering::Less, Ordering::Less) => cur = n.borrow().right.clone(),
            _ => return Some(n.clone()),
        };
    }
    None
}
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| node.children.entry(c).or_default())
            .is_leaf = true;
    }

    fn get(&self, word: String) -> Option<&Trie> {
        word.chars().try_fold(self, |node, c| node.children.get(&c))
    }

    fn search(&self, word: String) -> bool {
        self.get(word).map_or(false, |node| node.is_leaf)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get(prefix).is_some()
    }
}
