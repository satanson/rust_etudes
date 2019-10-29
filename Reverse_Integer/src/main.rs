struct Solution {}

impl Solution {
  fn reverse_pos(x: i32) -> i32 {
    assert_eq!(x > 0, true);
    const i32_max: i32 = std::i32::MAX;
    let mut n = 0;
    let mut q = x;
    loop {
      if q == 0 {
        break;
      }
      // overflow
      let r = q % 10;
      q = q / 10;
      if n > (i32_max - r) / 10 {
        return 0;
      }
      n = n * 10 + r;
    }
    return n;
  }

  fn reverse_neg(x: i32) -> i32 {
    assert_eq!(x < 0, true);
    const i32_min: i32 = std::i32::MIN;
    let mut n = 0;
    let mut q = x;
    loop {
      if q == 0 {
        break;
      }
      let r = q % 10;
      q = q / 10;
      //overflow
      if n < (i32_min - r) / 10 {
        return 0;
      }
      n = n * 10 + r;
    }
    return n;
  }

  pub fn reverse(x: i32) -> i32 {
    const i32_max: i32 = std::i32::MAX;
    const i32_min: i32 = std::i32::MIN;
    if x == 0 {
      return 0;
    } else if x > 0 {
      return Solution::reverse_pos(x);
    } else {
      return Solution::reverse_neg(x);
    }
  }
}

fn main() {
  println!("{}", Solution::reverse(123));
  println!("{}", Solution::reverse(-321));
  println!("{}", Solution::reverse(-210));
  println!("{}", Solution::reverse(1463847412));
  println!("{}", Solution::reverse(std::i32::MAX));
  println!("{}", Solution::reverse(std::i32::MIN));
  println!("{}", std::i32::MAX);
  println!("{}", std::i32::MIN);
  println!("{}", Solution::reverse(Solution::reverse(std::i32::MAX)+1));
}
