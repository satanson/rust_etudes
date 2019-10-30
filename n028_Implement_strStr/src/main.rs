struct Solution {}

impl Solution {
  pub fn kmp_table(t: &mut Vec<i32>, m: &[u8]) {
    let n = m.len() as i32;
    assert!(n > 0);
    t.resize((n + 1) as usize, 0);
    t[0] = -1;
    let mut p: i32 = 1;
    let mut c: i32 = 0;
    while p < n {
      if m[p as usize] == m[c as usize] {
        t[p as usize] = t[c as usize]
      } else {
        t[p as usize] = c;
        c = t[c as usize];
        while c >= 0 && m[c as usize] != m[p as usize] {
          c = t[c as usize];
        }
      }
      p += 1;
      c += 1;
    }
    t[p as usize] = c;
  }

  pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let n = needle.len();
    if n == 0 {
      return 0;
    }
    let mut kmp: Vec<i32> = Vec::new();
    Self::kmp_table(&mut kmp, needle);

    let mut k = 0;
    let mut i = 0;
    //println!("kmp:{:?}", kmp);
    while k + i < haystack.len() {
      //println!("k={} i={} c={}", k, i, kmp[i]);
      //std::thread::sleep(std::time::Duration::from_millis(100));
      if haystack[k + i] == needle[i] {
        i += 1;
        if i == n {
          return k as i32;
        }
      } else {
        match kmp[i] {
          -1 => {
            //println!("br#0: k={} i={} c={}", k, i, kmp[i]);
            //std::thread::sleep(std::time::Duration::from_millis(100));
            k += i + 1;
            i = 0;
          }
          0 => {
            //println!("br#1: k={} i={} c={}", k, i, kmp[i]);
            //std::thread::sleep(std::time::Duration::from_millis(100));
            k += i;
            i = 0;
          }
          l if l > 0 => {
            //println!("br#2: k={} i={} c={}", k, i, kmp[i]);
            //std::thread::sleep(std::time::Duration::from_millis(100));
            k = k + i - l as usize;
            i = l as usize;
          }
          _ => {
            panic!("l must be >= -1");
          }
        }
      }
    }
    return -1;
  }
}

fn main() {
  println!("{}", Solution::str_str(String::from("abcdefg"), String::from("cde")));
  println!("{}", Solution::str_str(String::from("ABC ABCDAB ABCDABCDABDE"), String::from("ABCDABD")));
}
