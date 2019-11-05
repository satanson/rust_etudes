struct Solution {}

impl Solution {
  pub fn my_pow_pos(x:f64, n:i32)->f64{
    let mut ans = 1.0f64;
    let mut pow2 = x;
    for k in 0..31 {
      let kth_bit = 1<<k;
      if kth_bit & n == kth_bit {
        ans *= pow2;
      }
      pow2 *= pow2;
    }
    ans
  }
  pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == std::i32::MIN {
      let mut ans = x;
      for _ in 0..31 {
        ans *= ans;
      }
      1.0f64/ans
    } else if n >= 0 {
      Self::my_pow_pos(x, n)
    } else {
      1.0f64/Self::my_pow_pos(x, -n)
    }
  }
}

fn main() {
  println!("{}", Solution::my_pow(2.0, -1));
  println!("{}", Solution::my_pow(2.0, -2));
  println!("{}", Solution::my_pow(2.0, -0));
  println!("{}", Solution::my_pow(2.0, 1));
  println!("{}", Solution::my_pow(2.0, 2));
  println!("{}", Solution::my_pow(2.0, 3));
  println!("{}", Solution::my_pow(2.0, 4));
  println!("{}", Solution::my_pow(2.0, 5));
}
