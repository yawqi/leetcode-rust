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
      right: None
    }
  }
}

struct Solution;
fn main() {}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut deque = VecDeque::new();
        let mut depth = 0;
        if let Some(node) = root {
            deque.push_back(node);
        }

        while !deque.is_empty() {
            let len = deque.len();
            depth += 1;
            for _ in 0..len {
                if let Some(node) = deque.pop_front() {
                    if node.borrow().right.is_none() && node.borrow().left.is_none() {
                        return depth;
                    }
                    if let Some(left) = node.clone().borrow_mut().left.take() {
                        deque.push_back(left);
                    }
                    if let Some(right) = node.clone().borrow_mut().right.take() {
                        deque.push_back(right);
                    }
                }
            }
        }
        depth
    }
}