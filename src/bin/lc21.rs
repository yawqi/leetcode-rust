use std::ops::DerefMut;

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
    pub fn merge_two_lists(mut node1: Option<Box<ListNode>>, mut node2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut curr = &mut dummy;
        while node1.is_some() && node2.is_some() {
            let n1 = node1.as_deref_mut().unwrap();
            let n2 = node2.as_deref_mut().unwrap();

            if n1.val <= n2.val {
                let tmp = n1.next.take();
                curr.next = node1;
                node1 = tmp;
            } else {
                let tmp = n2.next.take();
                curr.next = node2;
                node2 = tmp;
            }
            curr = curr.next.as_deref_mut().unwrap();
        }

        curr.next = node1.or(node2);
        dummy.next
    }
}