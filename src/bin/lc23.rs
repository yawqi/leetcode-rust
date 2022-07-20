// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {}
struct Solution;


use std::collections::BinaryHeap;
use std::cmp::{PartialOrd, Ord, PartialEq};
use std::ops::DerefMut;


impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut priority_queue = BinaryHeap::new();
        let mut dummy = ListNode::new(-1);
        let mut curr = &mut dummy;
        lists.iter_mut()
                .for_each(|a| {
                    if let Some(node) = a.take() {
                        priority_queue.push(node);
                    }
                });
        while let Some(mut node) = priority_queue.pop() {
            if let Some(nxt) = node.next.take() {
                priority_queue.push(nxt);
            }
            curr.next = Some(node);
            curr = curr.next.as_deref_mut().unwrap();
        }

        dummy.next
    }
}