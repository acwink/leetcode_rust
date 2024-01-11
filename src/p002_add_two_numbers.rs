struct Solution;

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
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut dummy, mut next) = (Box::new(ListNode::new(-1)), 0);
        let mut curr = dummy.as_mut();
        while l1.is_some() || l2.is_some() {
            let (l1_val, l2_val) = (
                if l1.is_some() { l1.as_ref().unwrap().val } else {0},
                if l2.is_some() { l2.as_ref().unwrap().val } else {0}, 
            );
            let sum = l1_val + l2_val + next;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            next = sum / 10;

            l1 = if l1.is_some() { l1.unwrap().next.take() } else { None };
            l2 = if l2.is_some() { l2.unwrap().next.take() } else { None };
            curr = curr.next.as_mut().unwrap();
        }
        if next == 1 { 
            curr.next = Some(Box::new(ListNode::new(1)));
        }
        dummy.next
    }
}