struct Solution {}

impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
      return 0;
    }
    let mut l = 0;
    let mut r = n;
    while l < r {
      let m = (l + r) / 2;
      let mid = nums[m];
      if mid == target {
        return m as i32;
      } else if mid < target {
        l = m + 1;
      } else {
        r = m;
      }
    }
    return l as i32;
  }
}

fn main() {
  println!("{}", Solution::search_insert(vec!(1), 2));
  println!("{}", Solution::search_insert(vec!(1), 0));
  println!("{}", Solution::search_insert(vec!(1), 1));

  println!("{}", Solution::search_insert(vec!(1, 3), 0));
  println!("{}", Solution::search_insert(vec!(1, 3), 1));
  println!("{}", Solution::search_insert(vec!(1, 3), 2));
  println!("{}", Solution::search_insert(vec!(1, 3), 3));
  println!("{}", Solution::search_insert(vec!(1, 3), 4));
}
