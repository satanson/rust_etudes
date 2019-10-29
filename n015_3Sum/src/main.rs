struct Solution {}

impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();
    let mut sum3: Vec<Vec<i32>> = Vec::new();
    let len = nums.len();
    if len < 3 {
      return sum3;
    }
    for i in 0..len {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }
      let mut l = (i + 1) as i32;
      let mut r = (len - 1) as i32;
      while l < r {
        let s = nums[i] + nums[l as usize] + nums[r as usize];
        if s > 0 {
          r -= 1;
        } else if s < 0 {
          l += 1;
        } else {
          sum3.push(vec!(nums[i], nums[l as usize], nums[r as usize]));
          while l + 1 < len  as i32 && nums[l as usize] == nums[(l + 1) as usize] {
            l += 1;
          }
          while r - 1 >= 0 && nums[r as usize] == nums[(r - 1) as usize] {
            r -= 1;
            println!("r={}", r);
          }
          l += 1;
          r -= 1;
        }
      }
    }
    return sum3;
  }
}

fn main() {
  //println!("{:?}", Solution::three_sum(vec!(-1, 0, 1, 2, -1, -4)));
  println!("{:?}", Solution::three_sum(vec!(0, 0, 0)));
}
