struct Solution {}

impl Solution {
  pub fn divide_neg_neg(dividend: i32, divisor: i32) -> i32 {
    assert!(dividend < 0 && divisor < 0);
    let mut q = 0;
    let mut k = 0;
    let mut a = dividend;
    let mut b = divisor;
    const LIMIT: i32 = std::i32::MIN >> 1;

    while a - b <= 0 && b >= LIMIT {
      b <<= 1;
      k += 1;
    }

    if a - b > 0 && k > 0 {
      b >>= 1;
      k -= 1;
    }

    while k > 0 {
      if a - b <= 0 {
        q += 1 << k;
        a -= b;
      }
      k -= 1;
      b >>= 1;
    }
    if a - b <= 0 {
      q += 1;
    }
    return q;
  }
  pub fn divide_neg_pos(dividend: i32, divisor: i32) -> i32 {
    assert!(dividend < 0 && divisor > 0);
    let mut q = 0;
    let mut k = 0;
    let mut a = dividend;
    let mut b = divisor;
    const LIMIT: i32 = std::i32::MAX >> 1;

    while a + b <= 0 && b <= LIMIT {
      b <<= 1;
      k += 1;
    }

    if a + b > 0 && k > 0 {
      b >>= 1;
      k -= 1;
    }
    //println!("a={} b={} k={}",a, b, k);
    while k > 0 {
      if a + b <= 0 {
        q -= 1 << k;
        a += b;
      }
      k -= 1;
      b >>= 1;
      //println!("a={} b={} k={}",a, b, k);
    }
    while a + b <= 0 {
      q -= 1;
      a += b;
    }
    //println!("a={} b={} k={}",a, b, k);
    return q;
  }
  pub fn divide_pos_neg(dividend: i32, divisor: i32) -> i32 {
    assert!(dividend > 0 && divisor < 0);
    let mut q = 0;
    let mut k = 0;
    let mut a = dividend;
    let mut b = divisor;
    const LIMIT: i32 = std::i32::MIN >> 1;

    while a + b >= 0 && b >= LIMIT {
      b <<= 1;
      k += 1;
    }

    if a + b < 0 && k > 0 {
      b >>= 1;
      k -= 1;
    }

    while k > 0 {
      if a + b >= 0 {
        q -= 1 << k;
        a += b;
      }
      k -= 1;
      b >>= 1;
    }
    if a + b >= 0 {
      q -= 1;
    }
    return q;
  }

  pub fn divide_pos_pos(dividend: i32, divisor: i32) -> i32 {
    assert!(dividend > 0 && divisor > 0);
    let mut q = 0;
    let mut k = 0;
    let mut a = dividend;
    let mut b = divisor;
    const LIMIT: i32 = std::i32::MAX >> 1;

    while a >= b && b <= LIMIT {
      b <<= 1;
      k += 1;
    }

    if a < b && k > 0 {
      b >>= 1;
      k -= 1;
    }

    while k > 0 {
      if a >= b {
        q += 1 << k;
        a -= b;
      }
      k -= 1;
      b >>= 1;
    }
    if a >= b {
      q += 1;
    }
    return q;
  }


  pub fn divide(dividend: i32, divisor: i32) -> i32 {
    assert_ne!(divisor, 0);
    if dividend == std::i32::MIN {
      if divisor == -1 {
        return std::i32::MAX;
      } else if divisor == 1 {
        return std::i32::MIN;
      }
    }
    if dividend == 0 {
      return 0;
    }

    if dividend > 0 && divisor > 0 {
      return Self::divide_pos_pos(dividend, divisor);
    } else if dividend > 0 && divisor < 0 {
      return Self::divide_pos_neg(dividend, divisor);
    } else if dividend < 0 && divisor > 0 {
      return Self::divide_neg_pos(dividend, divisor);
    } else {
      return Self::divide_neg_neg(dividend, divisor);
    }
  }
}

fn main() {
  // println!("{}", Solution::divide(10, 3));
  // println!("{}", Solution::divide(10, -3));
  // println!("{}", Solution::divide(-10, 3));
  // println!("{}", Solution::divide(-10, -3));
  // println!("{}", Solution::divide(7, -3));
  // println!("{}", Solution::divide(std::i32::MIN, -1));
  // println!("{}", Solution::divide(std::i32::MIN, 1));
  // println!("{}", Solution::divide(5, 6));
  println!("{}", Solution::divide(std::i32::MIN, 2));
}
