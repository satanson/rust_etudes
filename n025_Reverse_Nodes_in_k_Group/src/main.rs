extern crate common;

use common::list::ListNode;

struct Solution {}

impl Solution {
  pub fn advance(p: &Option<Box<ListNode>>, k: usize) -> &Option<Box<ListNode>> {
    let mut q = p;
    for _ in 0..k {
      if q.is_none() {
        break;
      }
      q = &q.as_ref().unwrap().next;
    }
    return q;
  }


  pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
      return head;
    }
    let hdr = Some(Box::new(ListNode { next: head, val: 0 }));
    let mut new_hdr = Some(Box::new(ListNode::new(0)));
    let mut p0 = &hdr;
    let mut pk = Self::advance(p0, k as usize);
    let mut ph = &mut new_hdr;

    while pk.is_some() {
      let mut group_hdr = Some(Box::new(ListNode::new(0)));
      p0 = Self::advance(&p0, 1);
      group_hdr.as_mut().unwrap().next = Some(Box::new(ListNode::new(p0.as_ref().unwrap().val)));
      while p0 != pk {
        p0 = Self::advance(&p0, 1);
        let n = &group_hdr.as_ref().unwrap().next;
        group_hdr.as_mut().unwrap().next = Some(Box::new(
          ListNode { next: n.clone(), val: p0.as_ref().unwrap().val }));
      }
      let mut pg = &group_hdr.as_ref().unwrap().next;
      while pg.is_some() {
        ph.as_mut().unwrap().next = Some(Box::new(ListNode::new(pg.as_ref().unwrap().val)));
        ph = &mut ph.as_mut().unwrap().next;
        pg = &pg.as_ref().unwrap().next;
      }
      pk = Self::advance(pk, k as usize);
    }

    p0 = Self::advance(&p0, 1);
    while p0.is_some() {
      ph.as_mut().unwrap().next = Some(Box::new(ListNode::new(p0.as_ref().unwrap().val)));
      ph = &mut ph.as_mut().unwrap().next;
      p0 = &p0.as_ref().unwrap().next;
    }
    return new_hdr.unwrap().next;
  }
}

fn main() {
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1)), 0).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1)), 1).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1)), 2).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1,2)), 0).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1,2)), 1).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1,2)), 2).unwrap());
  println!("{}", Solution::reverse_k_group(ListNode::from(vec!(1,2)), 3).unwrap());
}
