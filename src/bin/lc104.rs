/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
 *
 * https://leetcode.cn/problems/maximum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (77.08%)
 * Likes:    1292
 * Dislikes: 0
 * Total Accepted:    780.7K
 * Total Submissions: 1M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，找出其最大深度。
 * 
 * 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
 * 
 * 说明: 叶子节点是指没有子节点的节点。
 * 
 * 示例：
 * 给定二叉树 [3,9,20,null,null,15,7]，
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * 
 * 返回它的最大深度 3 。
 * 
 */

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        let mut deque = VecDeque::new();
        if let Some(node) = root {
            deque.push_back(node.clone());
        }
        while !deque.is_empty() {
            let len = deque.len();
            depth += 1;
            for _ in 0..len {
                let node = deque.pop_front().unwrap();
                if let Some(r_child) = node.clone().borrow_mut().right.take() {
                    deque.push_back(r_child);
                }
                if let Some(l_child) = node.clone().borrow_mut().left.take() {
                    deque.push_back(l_child);
                }
            }
        }
        depth
    }
}
// @lc code=end

