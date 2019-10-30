struct Solution();
impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n==0 {
      return vec!(String::from(""));
    }
    if n==1 {
      return vec!(String::from("()"));
    }
    let mut ans:Vec<String> = Vec::new();
    for k in 0..n {
      let l = n-1-k;
      let inner = Self::generate_parenthesis(k);
      let outer = Self::generate_parenthesis(l);
      for is in &inner {
        for os in &outer {
          ans.push(format!("({}){}", is, os));
        }
      }
    }
    return ans;
  }
}

fn main() {
  println!("{:?}", Solution::generate_parenthesis(3));
  println!("{:?}", Solution::generate_parenthesis(4));
}
