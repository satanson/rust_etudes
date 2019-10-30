pub mod list {
  // Definition for singly-linked list.
  #[derive(PartialEq, Eq, Clone, Debug)]
  pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
  }

  impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
      ListNode {
        next: None,
        val,
      }
    }
    pub fn from(vals: Vec<i32>) -> Option<Box<ListNode>> {
      let mut hdr = Box::new(ListNode { next: None, val: 0 });
      let mut curr =  &mut hdr;
      for _x in &vals {
        curr.next = Some(Box::new(ListNode { next: None, val: *_x}));
        curr = curr.next.as_mut().unwrap()
      }
      return hdr.next;
    }

    pub fn to_vec(&self)->Vec<i32> {
      let mut curr = &Some(Box::new(self.clone()));
      let mut vals = Vec::new();
      while curr.is_some() {
        vals.push(curr.as_ref().unwrap().val);
        curr = &curr.as_ref().unwrap().next;
      }
      return vals;
    }
  }

  use std::fmt::{Display, Formatter, Result};

  impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
      let xs:Vec<String> = self.to_vec().iter().map(|&x|x.to_string()).collect();
      write!(f, "List{{{}->nil}}",xs.join("->"))
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
