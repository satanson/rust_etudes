struct Solution {}

impl Solution {
  pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 2 {
      return vec!(nums);
    }
    let mut nums = nums.clone();
    nums.sort();
    let mut ans = Vec::new();
    loop {
      ans.push(nums.clone());

      let (mut ii, mut jj) = (0, 0);
      for i in 0..n {
        for j in i+1..n {
          if nums[i] < nums[j] {
            ii = i;
            jj = j;
          }
        }
      }
      if ii==jj {
        break;
      }
      nums.swap(ii, jj);
      nums[ii+1..].sort();
    }
    return ans;
  }
}

fn main() {
  println!("{:?}",Solution::permute_unique(vec!(1,2,2)));
  println!("{:?}",Solution::permute_unique(vec!(1,1,2,2,2,3,3,3,3)));
}
