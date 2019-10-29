struct Solution {}

impl Solution {
  pub fn lcs(s1: &String, s2: &String) -> (usize, usize) {
    if s1.is_empty() || s2.is_empty() {
      return (0, 0);
    }
    let ss1 = s1.as_bytes();
    let ss2 = s2.as_bytes();
    let n1 = ss1.len();
    let n2 = ss2.len();
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut row: Vec<i32> = Vec::new();
    row.resize(n2 + 1, 0);
    mat.resize(n1 + 1, row);
    let mut max_len = 0i32;
    let mut max_off = 0i32;

    for i in 1..n1 + 1 {
      for j in 1..n2 + 1 {
        if ss1[i - 1] == ss2[j - 1] {
          let len = mat[i - 1][j - 1] + 1;
          mat[i][j] = len;
          if len > max_len {
            max_len = len;
            max_off = i as i32;
          }
        } else {
          mat[i][j] = 0;
        }
      }
    }
    return ((max_off - max_len) as usize, max_len as usize);
  }
  pub fn longest_palindrome(s: String) -> String {
    //let rev_s = s.chars().rev().collect();
    //let (off, len) = Solution::lcs(&s, &rev_s);
    //return s[off..off+len].to_string();
    let ss = s.as_bytes();
    let n = ss.len();
    if n < 2 {
      return s;
    }
    let mut off = 0;
    let mut len = 1;

    let mut d: Vec<Vec<bool>> = Vec::new();
    let mut row: Vec<bool> = Vec::new();
    row.resize(n, false);
    d.resize(n, row);

    // len=1
    for i in 0..n {
      d[0][i] = true;
    }

    // len=2
    for i in 0..n - 1 {
      if ss[i] == ss[i + 1] {
        d[1][i] = true;
        len = 2;
        off = i;
      }
    }

    // len>2

    for l in 2..n {
      for i in 0..n - l {
        if ss[i] == ss[i + l] && d[l - 2][i + 1] {
          d[l][i]=true;
          len = l + 1;
          off = i;
        }
      }
    }
    return s[off..off + len].to_string();
  }
}

fn main() {
  let s1 = String::from("eabaf");
  let s2 = String::from("abcba");
  println!("{}", Solution::longest_palindrome(s2));
}
