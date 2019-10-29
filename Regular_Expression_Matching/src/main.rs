struct Solution {}

impl Solution {
  pub fn is_match_rev(s: &str, p: &str) -> bool {
    // p's len == 0
    if p.is_empty() {
      return s.is_empty();
    }

    let p_len = p.len();
    let s_len = s.len();
    let p_bytes = p.as_bytes();
    let s_bytes = s.as_bytes();

    // p_len == 1
    if p_len == 1 {
      if s_len != 1 {
        return false;
      }
      let s_c0 = s_bytes[0] as char;
      let p_c0 = p_bytes[0] as char;
      if p_c0 == '.' || p_c0 == s_c0 {
        return true;
      } else {
        return false;
      }
    }

    let p_c0 = p_bytes[0] as char;
    let p_c1 = p_bytes[1] as char;

    // p_len ==2 and p[1] == '*'
    if p_len == 2 && p_c1 == '*' {
      if p_c0 == '.' {
        return true;
      } else {
        for &s_c in s_bytes {
          if s_c as char != p_c0 {
            return false;
          }
        }
        return true;
      }
    }

    if p_c1 == '*' {
      if p_c0 == '.' {
        for i in 0..s_len + 1 {
          if Self::is_match_rev(&s[i..], &p[2..]) {
            return true;
          }
        }
        return false;
      } else {
        for i in 0..s_len {
          let s_c = s_bytes[i] as char;
          if s_c == p_c0 {
            if Self::is_match_rev(&s[i + 1..], &p[2..]) {
              return true;
            }
          } else {
            break;
          }
        }
        return Self::is_match_rev(&s, &p[2..]);
      }
    } else {
      if s_len == 0 {
        return false;
      }
      let s_c0 = s_bytes[0] as char;
      if p_c0 == '.' || p_c0 == s_c0 {
        return Self::is_match_rev(&s[1..], &p[1..]);
      } else {
        return false;
      }
    }

    return false;
  }

  pub fn is_match(s: String, p: String) -> bool {
    return Solution::is_match_rev(&s, &p);
  }
}

fn main() {
  println!("{}", Solution::is_match(String::from("abc"), String::from("abc")));
  println!("{}", Solution::is_match(String::from("abc"), String::from("ab.d")));
  println!("{}", Solution::is_match(String::from("aab"), String::from("c*a*b")));
  println!("{}", Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")));
  println!("{}", Solution::is_match(String::from("mississippi"), String::from("mis*is*ip*.")));
  println!("{}", Solution::is_match(String::from("a"), String::from("ab*")));
  println!("{}", Solution::is_match(String::from("a"), String::from(".*..a*")));
}
