extern crate common;

use common::list::ListNode;
struct Solution{}

impl Solution {
  pub fn advance(p: &Option<Box<ListNode>>, k: usize) -> &Option<Box<ListNode>> {
    let mut q = p;
    for _ in 0..k {
      if q.is_none() {
        break;
      } else {
        q = &q.as_ref().unwrap().next;
      }
    }
    return q;
  }

  pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let hdr = Some(Box::new(ListNode { next: head, val: 0 }));
    let mut p0 = &hdr;
    let mut p2 = Self::advance(p0, 2);
    let mut new_hdr = Some(Box::new(ListNode::new(0)));
    let mut p = &mut new_hdr;

    while !p2.is_none() {
      let pa = Self::advance(p0,1);
      let pb = Self::advance(p0,2);

      p.as_mut().unwrap().next = Some(Box::new(ListNode::new(pb.as_ref().unwrap().val)));
      p = &mut p.as_mut().unwrap().next;
      p.as_mut().unwrap().next = Some(Box::new(ListNode::new(pa.as_ref().unwrap().val)));
      p = &mut p.as_mut().unwrap().next;
      p0 = p2;
      p2 = Self::advance(p2,  2);
    }

    p0  = Self::advance(p0, 1);
    while !p0.is_none() {
      p.as_mut().unwrap().next = Some(Box::new(ListNode::new(p0.as_ref().unwrap().val)));
      p = &mut p.as_mut().unwrap().next;
      p0  = Self::advance(p0, 1);
    }
    return new_hdr.unwrap().next;
  }
}

fn main() {
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1))).unwrap());
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1,2))).unwrap());
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1,2,3))).unwrap());
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1,2,3,4))).unwrap());
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1,2,3,4,5))).unwrap());
  println!("{}", Solution::swap_pairs(ListNode::from(vec!(1,2,3,4,5,6))).unwrap());
}
