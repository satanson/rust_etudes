struct Solution {}

impl Solution {
  pub fn is_valid(s: String) -> bool {
    if s.is_empty() {
      return true;
    }
    let mut stk = Vec::new();
    for c in s.chars() {
      let top = stk.last();
      match (c, top) {
        ('[', _) | ('(', _) | ('{', _) => {
          stk.push(c);
        }
        (']', Some('[')) | (')', Some('(')) | ('}', Some('{')) => {
          stk.pop();
        }
        _ => {
          return false;
        }
      }
    }
    return stk.is_empty();
  }
}

fn main() {
  println!("{}", Solution::is_valid(String::from("()")));
  println!("{}", Solution::is_valid(String::from("{}")));
  println!("{}", Solution::is_valid(String::from("[]")));
  println!("{}", Solution::is_valid(String::from("[](){}[{()}()]{{{{{{}}}}}}")));
}
