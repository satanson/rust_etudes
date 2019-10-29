struct Solution {}

impl Solution {
  pub fn roman_to_int_helper(
    s: &[u8],
    one: u8,
    five: u8,
    ten: u8) -> (i32, usize) {
    if s.len() >= 4 {
      let a = s[0..4].to_vec();
      if a == vec!(five, one, one, one) {
        return (8, 4);
      }
    }

    if s.len() >= 3 {
      let a = s[0..3].to_vec();
      if a == vec!(one, one, one) {
        return (3, 3);
      } else if a == vec!(five, one, one) {
        return (7, 3);
      }
    }

    if s.len() >= 2 {
      let a = s[0..2].to_vec();
      if a == vec!(one, one) {
        return (2, 2);
      } else if a == vec!(one, five) {
        return (4, 2);
      } else if a == vec!(five, one) {
        return (6, 2);
      } else if a == vec!(one, ten) {
        return (9, 2);
      }
    }
    if s.len() >= 1 {
      let a = s[0..1].to_vec();
      if a == vec!(one) {
        return (1, 1);
      } else if a == vec!(five) {
        return (5, 1);
      }
    }
    return (0, 0);
  }

  pub fn roman_to_int(s: String) -> i32 {
    assert!(!s.is_empty());
    let bs = s.as_bytes();
    let mut i = 0;
    let mut n = 0;
    while i<bs.len() && bs[i] as char == 'M' {
      i += 1;
    }
    n += 1000 * i as i32;

    if i >= bs.len() {
      return n;
    }
    let c = bs[i] as char;
    if c == 'C' || c == 'D' {
      let (k, advance) = Self::roman_to_int_helper(
        &bs[i..],
        'C' as u8,
        'D' as u8,
        'M' as u8);
      n += 100 * k;
      i += advance;
    }


    if i >= bs.len() {
      return n;
    }
    let c = bs[i] as char;
    if c == 'X' || c == 'L' {
      let (k, advance) = Self::roman_to_int_helper(
        &bs[i..],
        'X' as u8,
        'L' as u8,
        'C' as u8);
      n += 10 * k;
      i += advance;
    }

    if i >= bs.len() {
      return n;
    }
    let c = bs[i] as char;
    if c == 'I' || c == 'V' {
      let (k, advance) = Self::roman_to_int_helper(
        &bs[i..],
        'I' as u8,
        'V' as u8,
        'X' as u8);
      n += k;
      i += advance;
    }
    return n;
  }
}

fn main() {
  //let a = vec!(1,2,3);
  //let b = vec!(1,2,3);
  //println!("{}",a==b);
  //println!("{}", Solution::roman_to_int(String::from("I")));
  //println!("{}", Solution::roman_to_int(String::from("MMMCMVIII")));
  println!("{}", Solution::roman_to_int(String::from("MDLXX")));
}
