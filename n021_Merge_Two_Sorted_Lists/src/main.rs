struct Solution {}

extern crate common;

use common::list::ListNode;

impl Solution {
  pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
      return l2;
    }
    if l2.is_none() {
      return l1;
    }
    let mut hdr = Some(Box::new(ListNode::new(0)));
    let mut p0 = &l1;
    let mut p1 = &l2;
    let mut p = &mut hdr;

    while p0.is_some() && p1.is_some() {
      if p0.as_ref().unwrap().val <= p1.as_ref().unwrap().val {
        p.as_mut().unwrap().next = p0.clone();
        p0 = &p0.as_ref().unwrap().next;
        p = &mut p.as_mut().unwrap().next;
      } else {
        p.as_mut().unwrap().next = p1.clone();
        p1 = &p1.as_ref().unwrap().next;
        p = &mut p.as_mut().unwrap().next;
      }
    }

    while p0.is_some() {
      p.as_mut().unwrap().next = p0.clone();
      p0 = &p0.as_ref().unwrap().next;
      p = &mut p.as_mut().unwrap().next;
    }

    while p1.is_some() {
      p.as_mut().unwrap().next = p1.clone();
      p1 = &p1.as_ref().unwrap().next;
      p = &mut p.as_mut().unwrap().next;
    }
    return hdr.unwrap().next;
  }
}

fn main() {
  println!("{}", Solution::merge_two_lists(
    ListNode::from(vec!(1,2,3)),
    ListNode::from(vec!(4,5,6))).unwrap());
  println!("{}", Solution::merge_two_lists(
    ListNode::from(vec!()),
    ListNode::from(vec!(4,5,6))).unwrap());
  println!("{}", Solution::merge_two_lists(
    ListNode::from(vec!(1,2,3)),
    ListNode::from(vec!())).unwrap());
  println!("{}", Solution::merge_two_lists(
    ListNode::from(vec!(1)),
    ListNode::from(vec!(2,3,4))).unwrap());
}
