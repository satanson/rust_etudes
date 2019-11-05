struct Solution{}
impl Solution {
  pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.is_empty(){
      return Vec::new();
    }
    use std::collections::HashMap;
    let mut dict = HashMap::new();
    for s in &strs {
      let mut key = s.clone();
      unsafe {
        key.as_bytes_mut().sort();
      }
      dict.entry(key).or_insert(vec!()).push(s.clone());
    }
    return dict.iter().map(|(_,v)|{v.clone()}).collect();
  }
}

fn main() {
  println!("{:?}", Solution::group_anagrams(vec!("eat", "tea", "tan", "ate", "nat", "bat")
    .iter().map(|&x|{format!("{}", x)}).collect()));
}
