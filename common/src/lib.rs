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
      let mut curr = &mut hdr;
      for _x in &vals {
        curr.next = Some(Box::new(ListNode { next: None, val: *_x }));
        curr = curr.next.as_mut().unwrap()
      }
      return hdr.next;
    }

    pub fn to_vec(&self) -> Vec<i32> {
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
      let xs: Vec<String> = self.to_vec().iter().map(|&x| x.to_string()).collect();
      write!(f, "List{{{}->nil}}", xs.join("->"))
    }
  }
}

pub mod sudoku {
  pub fn sudoku_from(d: [[&str; 9]; 9]) -> Vec<Vec<char>> {
    return d.iter().map(|r| {
      r.iter().map(|&s| {
        s.as_bytes()[0] as char
      }).collect()
    }).collect();
  }

  pub fn is_valid_sudoku(board: &Vec<Vec<char>>) -> bool {
    use std::ops::AddAssign;
    use std::collections::HashMap;
    let f = |x: char, d: &mut HashMap<char, i32>| -> bool{
      match x {
        '1'...'9' => {
          let v = d.entry(x).or_insert(0);
          v.add_assign(1);
          if *v > 1 {
            return false;
          }
        }
        '.' => {}
        _ => {
          return false;
        }
      }
      return true;
    };
    // row
    for i in 0..9 {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for j in 0..9 {
        if !f(board[i][j], &mut d) {
          println!("[{},{}]: row conflict", i, j);
          return false;
        }
      }
    }

    // column
    for i in 0..9 {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for j in 0..9 {
        if !f(board[j][i], &mut d) {
          println!("[{},{}]: column conflict", j, i);
          return false;
        }
      }
    }
    // 3x3 cell
    for i in 0..9 {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for j in 0..9 {
        let r0 = i % 3;
        let c0 = i / 3;
        let r1 = j % 3 + 3 * r0;
        let c1 = j / 3 + 3 * c0;
        let c = board[r1][c1];
        if !f(c, &mut d) {
          println!("[{},{}]: nine conflict", r1, c1);
          return false;
        }
      }
    }
    return true;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
