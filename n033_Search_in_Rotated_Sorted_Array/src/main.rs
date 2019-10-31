struct Solution {}

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
      return -1;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
      let m = (l + r) / 2;
      //println!("l={} m={} r={} l..m={:?} m..r={:?} nums[l]={} nums[m]={} nums[r]={}",
      //         l, m, r, &nums[l..=m], &nums[m..=r], nums[l],nums[m], nums[r]);
      if nums[m] < target {
        if m < r && (target <= nums[r] || nums[m] >= nums[r]){
          l = m + 1;
        } else if l < m {
          r = m - 1;
        } else {
          return -1;
        }
      } else if nums[m] > target {
        if l < m && (nums[l] <= target || nums[l] >= nums[m]){
          r = m - 1;
        } else if m < r {
          l = m + 1;
        } else {
          return -1;
        }
      } else {
        return m as i32;
      }
    }
    if nums[l] == target {
      return l as i32;
    } else {
      return -1;
    }
  }
}

fn main() {
  let nums = vec!(1,2);
  for target in 0..1 {
    let i = Solution::search(nums.clone(), target);
    if i == -1 {
      println!("target={} not found", target);

    } else {
      println!("target={} i={} found={}", target, i, nums[i as usize]);
    }
  }

}
