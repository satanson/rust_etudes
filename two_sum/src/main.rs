use ::std::vec::Vec;
use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    if nums.is_empty() {
      return res;
    }
    let mut map :HashMap<i32, i32>= HashMap::new();
    map.insert(target - nums[0], 0);
    for (i, v) in nums[1..].iter().enumerate() {
      if map.contains_key(v) {
        res.push(map[v]);
        res.push(i as i32 + 1);
        return res;
      } else {
        map.insert(target - v, i  as i32 + 1);
      }
    }
    return res;
  }
}

fn main() {
  let nums = vec![1, 2, 3, 4, 5, 6, 7];
  let res = Solution::two_sum(nums, 8);
  println!("{:?}", res);
}
