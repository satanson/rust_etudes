struct Solution {}

impl Solution {
  pub fn multiply(num1: String, num2: String) -> String {
    let mut d: Vec<u8> = Vec::new();
    d.resize(num1.len() + num2.len(), 0);
    let mut a = &num1;
    let mut b = &num2;
    if a.len() < b.len() {
      std::mem::swap(&mut a, &mut b);
    }

    for (k, i) in b.chars().rev().enumerate() {
      let mut carry = 0;
      //println!("{:?}", &d);
      for (l, j) in a.chars().rev().enumerate() {
        let ij = (i as u8 - '0' as u8) * (j as u8 - '0' as u8) + carry;
        let x = ij % 10;
        carry = ij / 10;
        d[k + l] += x;
        carry += d[k+l] /10;
        d[k + l] %= 10;
        //println!("i={} j={} ij={} carry={} d[{}]={}", i, j, ij, carry, k+l, d[k+l]);
      }
      let mut m = k + a.len();
      loop {
        if carry == 0 {
          break;
        }
        d[m] += carry;
        carry = d[m] / 10;
        d[m] %= 10;
      }
    }
    let mut m = d.len();
    while m > 0 && d[m - 1] == 0 {
      m -= 1;
    }
    d.resize(std::cmp::max(1, m), 0);
    return d.iter().rev().map(|x| {
      (*x + '0' as u8) as char
    }).collect::<String>();
  }
}

fn main() {
  println!("{}", Solution::multiply(format!("123456789123456789"), format!("123456789123456789")));
  println!("{}", Solution::multiply(format!("0"), format!("124")));
  println!("{}", Solution::multiply(format!("0"), format!("0")));
  println!("{}", Solution::multiply(format!("0"), format!("12345")));
}
