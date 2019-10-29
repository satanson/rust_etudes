struct Solution {}

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
      return String::from("");
    }
    if strs.len() == 1 {
      return strs[0].clone();
    }
    let mut min_len = std::usize::MAX;
    for s in &strs {
      min_len = std::cmp::min(min_len, s.len());
    }

    let mut k = 0usize;
    while k < min_len {
      let c = strs[0].as_bytes()[k];
      let mut ok = true;
      for i in 1..strs.len() {
        if c != strs[i].as_bytes()[k] {
          ok = false;
        }
      }
      if ok {
        k += 1;
      } else {
        break;
      }
    }
    return strs[0][0..k].to_string();
  }
}

fn main() {
  println!("{}", Solution::longest_common_prefix(vec!(
    String::from("abcd"),
    String::from("abdef")
  )));
  println!("{}", Solution::longest_common_prefix(vec!(
    String::from("abcd"),
  )));
  println!("{}", Solution::longest_common_prefix(vec!(
  )));
}
