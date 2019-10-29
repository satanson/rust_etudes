struct Solution {}

impl Solution {
  fn my_atoi_pos(s: &[u8]) -> i32 {
    if s.is_empty() {
      return 0;
    }
    let mut n = 0;
    for c in s {
      if !(*c as char).is_ascii_digit() {
        break;
      }
      let r = *c as i32 - ('0' as i32);
      if n > (std::i32::MAX - r) / 10 {
        return std::i32::MAX;
      }
      n = n * 10 + r;
    }
    return n;
  }
  fn my_atoi_neg(s: &[u8]) -> i32 {
    if s.is_empty() {
      return 0;
    }
    let mut n = 0;
    for c in s {
      if !(*c as char).is_ascii_digit() {
        break;
      }
      let r = *c as i32 - ('0' as i32);
      if n < (std::i32::MIN + r) / 10 {
        return std::i32::MIN;
      }
      n = n * 10 - r;
    }
    return n;
  }

  pub fn my_atoi(str: String) -> i32 {
    let ss = str.as_bytes();
    for (i, c) in ss.iter().enumerate() {
      if (*c as char).is_ascii_whitespace() {
        continue;
      }
      if (*c as char) == '-' {
        return Solution::my_atoi_neg(&ss[i + 1..]);
      } else if (*c as char) == '+' {
        return Solution::my_atoi_pos(&ss[i + 1..]);
      }
      if (*c as char).is_ascii_digit() {
        return Solution::my_atoi_pos(&ss[i..]);
      } else {
        return 0;
      }
    }
    return 0;
  }
}

fn main() {
  println!("{}", Solution::my_atoi(String::from("-124")));
  println!("{}", Solution::my_atoi(String::from("+124")));
  println!("{}", Solution::my_atoi(String::from("124")));
  println!("{}", Solution::my_atoi(String::from("   +124")));
  println!("{}", Solution::my_atoi(String::from("   -124")));
  println!("{}", Solution::my_atoi(String::from("   124")));
  println!("{}", Solution::my_atoi(String::from("   -91283472332")));
  println!("{}", Solution::my_atoi(String::from("   -2147483648")));
  println!("{}", Solution::my_atoi(String::from("   -2147483647")));
  println!("{}", Solution::my_atoi(String::from("   +2147483647")));
  println!("{}", Solution::my_atoi(String::from("   +2147483648adfaf")));

}
