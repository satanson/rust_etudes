struct Solution {}

impl Solution {
  pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut stk: Vec<(usize, char)> = Vec::new();
    let mut max = 0;
    for (i, c) in s.chars().enumerate() {
      //println!("before {:?}, i={} c={}", stk, i, c);
      match (stk.last(), c) {
        (Some(&(j, '(')), ')') => {
          stk.pop();
          let mut tmp = i + 1 - j;
          stk.push((tmp, 'M'));
          if tmp > max {
            max = tmp;
          }
        }
        (Some(&(k, 'M')), ')') => {
          stk.pop();
          match stk.last() {
            Some(&(l, '(')) => {
              stk.pop();
              let mut tmp = i + 1 - l;
              stk.push((tmp, 'M'));
              if tmp > max {
                max = tmp;
              }
            }
            _ => {
              stk.push((i, c));
            }
          }
        }
        _ => {
          stk.push((i, c));
        }
      }

      if let Some(&(k, 'M')) = stk.last() {
        stk.pop();
        let mut tmp = k;
        loop {
          if let Some(&(k, 'M')) = stk.last() {
            tmp += k;
            stk.pop();
          } else {
            break;
          }
        }
        stk.push((tmp, 'M'));
        if tmp > max {
          max = tmp;
        }
      }
    }

    if let Some(&(k, 'M'))= stk.last() {
      if k > max {
        max = k;
      }
    }
    return max as i32;
  }
}

fn main() {
  //println!("{}", Solution::longest_valid_parentheses(String::from("(()")));
  //println!("{}", Solution::longest_valid_parentheses(String::from(")()())")));
  //println!("{}", Solution::longest_valid_parentheses(String::from(")()())")));
  println!("{}", Solution::longest_valid_parentheses(String::from("(())))((())()))))")));
}
