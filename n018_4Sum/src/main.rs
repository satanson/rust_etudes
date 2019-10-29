use std::{thread,time};
struct Solution {}

impl Solution {
  pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 4 {
      return vec!();
    }
    let mut nums = nums.clone();
    nums.sort();
    let nums = nums;

    let mut ans: Vec<Vec<i32>> = Vec::new();
    for ai in 0..nums.len() - 3 {
      if ai > 0 && nums[ai] == nums[ai - 1] {
        continue;
      }
      let nums2 = &nums[ai + 1..];
      let a = nums[ai];
      let target2 = target - a;
      if nums2.len() < 2 {
        continue;
      }
      for bi in 0..nums2.len() - 2 {
        if bi > 0 && nums2[bi] == nums2[bi - 1] {
          continue;
        }
        let b = nums2[bi];
        let mut ci = bi + 1;
        let mut di = nums2.len() - 1;
        while ci < di {
          let c = nums2[ci];
          let d = nums2[di];
          let sum = b + c + d;
          if sum < target2 {
            ci += 1;
          } else if sum > target2 {
            di -= 1;
          } else {
            //println!("num={:?} ai={} bi={} ci={} di={}",nums, ai, bi, ci, di,);
            ans.push(vec!(a,b,c,d));
            if ci + 1 < nums2.len() {
              ci += 1;
            }

            if di >=1 {
              di -=1;
            }

            while ci < nums2.len() && nums2[ci] == nums2[ci-1] {
              ci +=1;
            }
            while di >=1 && nums2[di] == nums2[di+1] {
              di -= 1;
            }
          }
        }
      }
    }
    return ans;
  }
}

fn main() {
  println!("{:?}", Solution::four_sum(vec!(-1,0,-5,-2,-2,-4,0,1,-2),-9));
}
