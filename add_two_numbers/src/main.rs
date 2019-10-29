#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val,
    }
  }
}
struct Solution{}
impl Solution {

  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
      return l2;
    }

    if l2.is_none() {
      return l1;
    }

    let mut l1_curr = &l1;
    let mut l2_curr = &l2;

    let mut ans = Box::new(ListNode { next: None, val: 0 });
    let mut curr = &mut ans;
    let mut carry = 0;
    while l1_curr.is_some() && l2_curr.is_some() {
      let l1_node = l1_curr.as_ref().unwrap();
      let l2_node = l2_curr.as_ref().unwrap();
      let mut val = l1_node.val + l2_node.val + carry;
      carry = val / 10;
      val %= 10;
      curr.next = Some(Box::new(ListNode { next: None, val }));
      curr = curr.next.as_mut().unwrap();
      l1_curr = &l1_curr.as_ref().unwrap().next;
      l2_curr = &l2_curr.as_ref().unwrap().next;
    }

    while l1_curr.is_some() {
      let mut val = l1_curr.as_ref().unwrap().val + carry;
      carry = val / 10;
      val %= 10;
      curr.next = Some(Box::new(ListNode { next: None, val }));
      curr = curr.next.as_mut().unwrap();
      l1_curr = &l1_curr.as_ref().unwrap().next;
    }

    while l2_curr.is_some() {
      let mut val = l2_curr.as_ref().unwrap().val + carry;
      carry = val / 10;
      val %= 10;
      curr.next = Some(Box::new(ListNode { next: None, val }));
      curr = curr.next.as_mut().unwrap();
      l2_curr = &l2_curr.as_ref().unwrap().next;
    }
    if carry > 0 {
      curr.next = Some(Box::new(ListNode { next: None, val: 1 }));
    }
    return ans.next;
  }
}


fn vec2List(vals: Vec<i32>) -> Option<Box<ListNode>> {
  let mut hdr = Box::new(ListNode { next: None, val: 0 });
  let mut curr =  &mut hdr;
  for _x in &vals {
    curr.next = Some(Box::new(ListNode { next: None, val: *_x}));
    curr = curr.next.as_mut().unwrap()
  }
  return hdr.next;
}

fn print_list(l: &Option<Box<ListNode>>) {
  let mut curr = l;
  while curr.is_some() {
    print!("{}, ", curr.as_ref().unwrap().val);
    curr = &curr.as_ref().unwrap().next;
  }
  println!()
}

fn main() {
  let l1 = vec2List(vec!(1,2,3,6,5,4));
  let l2 = vec2List(vec!(9,7,6));
  print_list(&l1);
  print_list(&l2);
  let l = Solution::add_two_numbers(l1, l2);
  print_list(&l);
}
