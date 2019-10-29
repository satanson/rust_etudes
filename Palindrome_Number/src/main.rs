struct Solution{}
impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false;
    }
    let mut q = x;
    let mut n = 0;
    loop {
      if q == 0 {
        break;
      }
      let r = q % 10;
      q = q / 10;
      if n > (std::i32::MAX -r)/10 {
        return false;
      }
      n = n * 10 +r;
    }
    return x == n;
  }
}

fn main() {
  println!("{}", Solution::is_palindrome(121));
  println!("{}", Solution::is_palindrome(-121));
  println!("{}", Solution::is_palindrome(1210));
}
