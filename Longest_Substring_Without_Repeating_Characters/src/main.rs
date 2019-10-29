use std::collections::HashMap;

struct Solution {}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    if s.as_bytes().len() < 2 {
      return s.as_bytes().len() as i32;
    }

    let mut max = -1;
    let mut hash: HashMap<u8, i32> = HashMap::new();
    let mut k = -1;
    for (i, b) in s.as_bytes().iter().enumerate() {

      match hash.get(b) {
        None => {
          hash.insert(*b, i as i32);
        }
        Some(ii) => {
          if *ii <= k {
            hash.insert(*b, i as i32);
            continue;
          }
          let curr_max = i as i32 - k -1;
          //println!("k={},i={},ii={},hash={:?}, curr_max={}, max={}", k, i, ii, &hash, curr_max, max);
          k = *ii;
          if curr_max > max {
            max = curr_max;
          }
          hash.insert(*b, i as i32);
        }
      }
    }
    let curr_max = s.as_bytes().len() as i32 - k -1;
    if curr_max > max {
      max = curr_max;
    }
    return max;
  }
}

fn main() {
  let s1 = "abcdefg";
  let s2 = "aaaaaaa";
  let s3 = "abcdaabc";
  let s4 = "cdd";
  //println!("{}: {}", s1, Solution::length_of_longest_substring(String::from(s1)));
  //println!("{}: {}", s2, Solution::length_of_longest_substring(String::from(s2)));
  //println!("{}: {}", s3, Solution::length_of_longest_substring(String::from(s3)));
  println!("{}: {}", s4, Solution::length_of_longest_substring(String::from(s4)));
}
