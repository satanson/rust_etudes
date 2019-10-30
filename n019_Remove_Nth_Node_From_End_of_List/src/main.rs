extern crate common;
use common::list::{ListNode};
struct Solution();

impl Solution {
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
      return head;
    }
    let hdr = Some(Box::new(ListNode{next:head, val:0}));
    let mut hdr2 = Some(Box::new(ListNode::new(0)));
    let mut p0 = &hdr;
    let mut p1 = &hdr;
    let mut p2 = &mut hdr2;

    for _ in 0..n {
      p1 = &p1.as_ref().unwrap().next;
    }
    while !p1.as_ref().unwrap().next.is_none() {
      p2.as_mut().unwrap().next = p0.as_ref().unwrap().next.clone();
      p0 = &p0.as_ref().unwrap().next;
      p1 = &p1.as_ref().unwrap().next;
      p2 = &mut p2.as_mut().unwrap().next;
    }

    p0 = &p0.as_ref().unwrap().next;
    while !p0.as_ref().unwrap().next.is_none() {
      p2.as_mut().unwrap().next = p0.as_ref().unwrap().next.clone();
      p2 = &mut p2.as_mut().unwrap().next;
      p0 = &p0.as_ref().unwrap().next;
    }
    p2.as_mut().unwrap().next = None;
    return hdr2.unwrap().next;
  }
}

fn main() {
  let l = ListNode::from(vec!(1,3,4,5,2));
  //println!("{}",l.unwrap());
  println!("{}",Solution::remove_nth_from_end(ListNode::from(vec!(1,2,3,4,5)),2).unwrap());
  println!("{}",Solution::remove_nth_from_end(ListNode::from(vec!(1,2,3,4,5)),1).unwrap());
  println!("{}",Solution::remove_nth_from_end(ListNode::from(vec!(1,2,3,4,5)),5).unwrap());
}
