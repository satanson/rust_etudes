struct Solution {}

impl Solution {
  pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n < 3 {
      return 0;
    }

    if n == 3 {
      return nums[0] + nums[1] + nums[2];
    }

    let mut nums = nums.clone();
    nums.sort();
    let mut diff = std::i32::MAX;
    let mut sum = target;
    for i in 0..n {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }
      let mut l = i + 1;
      let mut r = n - 1;
      while l < r {
        //println!("{:?} i={} l={} r={} nums[i]={} nums[l]={} nums[r]={}", &nums, i, l,r, nums[i],nums[l], nums[r]);
        let tmp_sum = nums[i] + nums[l] + nums[r];
        let tmp_diff = tmp_sum - target;
        if i32::abs(tmp_diff) < diff {
          diff = i32::abs(tmp_diff);
          sum = tmp_sum;
        }
        if tmp_diff < 0 {
          l += 1;
        } else if tmp_diff > 0 {
          r -= 1;
        } else {
          return sum;
        }
      }
    }
    return sum;
  }
}

fn main() {

  println!("{}", Solution::three_sum_closest(vec!(-1,0,1,1,55),3));
}
