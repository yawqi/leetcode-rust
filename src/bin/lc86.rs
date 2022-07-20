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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(-1);
        let mut dummy2 = ListNode::new(-1);
        let mut tail1 = &mut dummy1;
        let mut tail2 = &mut dummy2;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                tail1.next = Some(node);
                tail1 = tail1.next.as_deref_mut().unwrap();
                tail1.next = None;
            } else {
                tail2.next = Some(node);
                tail2 = tail2.next.as_deref_mut().unwrap();
                tail2.next = None;
            }
        }

        tail1.next = dummy2.next.take();
        dummy1.next
    }
}