struct Solution {}

impl Solution {
  pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
      return vec!();
    }

    let dict = vec!("", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz");
    let dict: Vec<&[u8]> = dict.iter().map(|&x| x.as_bytes()).collect();
    let digits: Vec<usize> = digits.as_bytes().iter().map(|&x| (x - '0' as u8) as usize).collect();
    let mut n = 1;
    for &d in &digits {
      n *= dict[d].len();
    }
    let mut ans: Vec<String> = Vec::new();
    for k in 0..n {
      let mut s = String::new();
      let mut q = k;
      for &d in &digits {
        let len = dict[d].len();
        let r = q % len;
        q = q / len;
        s.push(dict[d][r] as char);
      }
      ans.push(s);
    }
    return ans;
  }
}

fn main() {
  println!("{:?}", Solution::letter_combinations(String::from("239")));
}
