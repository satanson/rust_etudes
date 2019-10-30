extern crate common;

use common::list::ListNode;

struct Solution {}

use std::collections::BinaryHeap;
struct ListNodeDescentOrder<'a>(&'a Box<ListNode>);

use std::cmp::{PartialEq, Ord, Ordering};

impl<'a> PartialEq for ListNodeDescentOrder<'a> {
  fn eq(&self, other: &Self) -> bool {
    return self.0.val == other.0.val;
  }
}

impl<'a> Eq for ListNodeDescentOrder<'a> {}

impl<'a> Ord for ListNodeDescentOrder<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    let diff = self.0.val - other.0.val;
    if diff > 0 {
      return Ordering::Less;
    } else if diff < 0 {
      return Ordering::Greater;
    } else {
      return Ordering::Equal;
    }
  }
}

impl<'a> PartialOrd for ListNodeDescentOrder<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}

impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let n = lists.len();
    if n == 0 {
      return None;
    }

    let mut hdr = Some(Box::new(ListNode::new(0)));
    let mut p = &mut hdr;
    let mut pri_queue = BinaryHeap::with_capacity(n);

    for l in &lists {
      if let Some(n) = l {
        pri_queue.push(ListNodeDescentOrder(n));
      }
    }
    while !pri_queue.is_empty() {
      if let Some(n) = pri_queue.pop() {
        p.as_mut().unwrap().next = Some(Box::new(ListNode::new((n.0).val)));
        p = &mut p.as_mut().unwrap().next;

        if let Some(ref next) = (n.0).as_ref().next {
          pri_queue.push(ListNodeDescentOrder(next))
        }
      } else {
        break;
      }
    }
    return hdr.unwrap().next;
  }
}

fn main() {
  /*
  let a = Box::new(ListNode::new(10));
  let b = Box::new(ListNode::new(9));
  let c = Box::new(ListNode::new(2));
  let mut priq = BinaryHeap::new();
  println!("a<b={} b<c={} a<c={}",
           ListNodeDescentOrder(&a) == ListNodeDescentOrder(&a),
           ListNodeDescentOrder(&b) == ListNodeDescentOrder(&c),
           ListNodeDescentOrder(&a) == ListNodeDescentOrder(&c));
  priq.push(ListNodeDescentOrder(&a));
  priq.push(ListNodeDescentOrder(&b));
  priq.push(ListNodeDescentOrder(&c));
  loop {
    let top = priq.pop();
    if let Some(t) = &top {
      println!("{}",t.0.val)
    } else {
      break;
    }
  }
  */
  let sort_runs = vec!(
    ListNode::from(vec!(3, 5, 10)),
    ListNode::from(vec!(2)),
    ListNode::from(vec!(6, 7, 8, 9, 10))
  );
  println!("{}", Solution::merge_k_lists(sort_runs).unwrap());
}
