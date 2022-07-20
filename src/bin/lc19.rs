use std::ops::{DerefMut, Deref};

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


impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head.take();
        let mut latter = &dummy.clone();
        let mut former = &mut dummy;

        for _ in 0..n {
            latter = latter.next.as_deref().unwrap();
        }
        while let Some(nxt) = &latter.next {
            latter = nxt;
            former = former.next.as_deref_mut().unwrap();
        }
        former.next = former.next.take().unwrap().next.take();
        dummy.next
    }
}