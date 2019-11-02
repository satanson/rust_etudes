struct Solution {}

impl Solution {
  pub fn count_and_say(n: i32) -> String {
    assert!(1 <= n && n <= 30);
    let mut s = String::from("1");
    if n == 1 {
      return s;
    }
    let mut s2 = String::new();
    let (mut a, mut b) = (&mut s, &mut s2);
    for _ in 1..n {
      let mut prev = 'x';
      let mut count = 0;
      for c in a.chars() {
        if c == prev {
          count += 1;
        } else if prev == 'x' {
          prev = c;
          count = 1;
        } else {
          b.push_str(&count.to_string());
          b.push(prev);
          prev = c;
          count = 1;
        }
      }
      b.push_str(&count.to_string());
      b.push(prev);
      a.clear();
      a.push_str(b);
      b.clear();
    }
    return s;
  }
}

fn main() {
  use std::iter::Iterator;
  (1..=30).into_iter().for_each(|x|{
    println!("{}", Solution::count_and_say(x));
  });
}
