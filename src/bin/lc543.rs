// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + Self::max_depth(&node.borrow().left.clone()) + Self::max_depth(&node.borrow().right.clone())
        } else {
            0
        }
    }

    fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let r_max = Self::max_depth(&node.borrow().right);
            let l_max = Self::max_depth(&node.borrow().left);
            std::cmp::max(1 + r_max, 1 + l_max)
        } else {
            0
        }
    }
}
// @lc code=end