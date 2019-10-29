struct Solution{}
impl Solution {
  pub fn int_to_roman_helper(
    n: usize,
    one: &'static str,
    five: &'static str,
    ten: &'static str) -> String {
    let mut s = String::new();
    if 0 < n && n < 4 {
      s = s + &String::from(one).repeat(n);
    } else if n == 4 {
      s.push_str(one);
      s.push_str(five);
    } else if n == 5 {
      s.push_str(five);
    } else if 5 < n && n < 9 {
      s.push_str(five);
      s = s + &String::from(one).repeat(n - 5);
    } else if n == 9 {
      s.push_str(one);
      s.push_str(ten);
    }
    return s;
  }
  pub fn int_to_roman(num: i32) -> String {
    assert!(0 < num && num < 4000);
    let thousands = num / 1000;
    let hundreds = num % 1000 / 100;
    let tens = num % 100 / 10;
    let n = num % 10;

    let mut s = String::new();
    s.reserve(100);

    if thousands != 0 {
      s += &String::from("M").repeat(thousands as usize);
    }

    s += &Self::int_to_roman_helper(hundreds as usize, "C", "D", "M");
    s += &Self::int_to_roman_helper(tens as usize, "X", "L", "C");
    s += &Self::int_to_roman_helper(n as usize, "I", "V", "X");
    return s;
  }
}

fn main() {
  println!("{}", Solution::int_to_roman(3999));
  println!("{}", Solution::int_to_roman(1994));
}
