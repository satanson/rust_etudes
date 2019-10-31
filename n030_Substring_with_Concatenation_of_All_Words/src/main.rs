struct Solution {}

impl Solution {
  pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut dict: HashMap<&str, i32> = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    if s.is_empty() || words.is_empty() {
      return ans;
    }
    for w in &words {
      *dict.entry(w).or_insert(0) += 1;
    }
    let l = words[0].len();
    let m = words.len();
    let L = l * m;
    let n = s.len();
    for i in 0..n - L + 1 {
      let mut ok = true;
      let mut dict2: HashMap<&str, i32> = HashMap::new();
      for j in 0..m {
        let t = &s[i + j * l..i + (j + 1) * l];
        //println!("i={} dict2={:?}, t={}", i, dict2, t);
        //println!("i={} dict={:?}", i, dict);
        let v2 = dict2.entry(t).or_insert(0);
        *v2 += 1;
        match dict.get(t) {
          Some(v) if *v2 > *v => {
            ok = false;
            break;
          }
          None => {
            ok = false;
            break;
          }
          _ => {}
        }
      }
      if ok {
        ans.push(i as i32);
      }
    }
    return ans;
  }
}

fn main() {
  //use std::collections::HashMap;
  //let mut dict: HashMap<&str, i32> = HashMap::new();
  //for w in vec!("a", "a", "b", "b", "c") {
  //  *dict.entry(w).or_insert(0) += 1;
  //}
  println!("{:?}", Solution::find_substring(
    String::from("wordgoodgoodgoodbestword"),
    vec!(String::from("word"),
         String::from("good"),
         String::from("best"),
         String::from("good")
    )));
}
