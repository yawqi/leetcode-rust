// Definition for a binary tree node.
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
// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_values = vec![];
        let mut deque = VecDeque::new();
        if let Some(node) = root {
            deque.push_back(node);
        }
        while !deque.is_empty() {
            let len = deque.len();
            let mut max = i32::MIN;
            for _ in 0..len {
                if let Some(node) = deque.pop_front() {
                    max = std::cmp::max(max, node.borrow().val);
                    if let Some(l_node) = node.clone().borrow_mut().left.take() {
                        deque.push_back(l_node);
                    }
                    if let Some(r_node) = node.clone().borrow_mut().right.take() {
                        deque.push_back(r_node);
                    }
                }
            }
            max_values.push(max);
        }
        max_values
    }
}
// @lc code=end