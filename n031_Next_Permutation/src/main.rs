struct Solution{}
impl Solution {
  pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    if n < 2 {
      return;
    }
    //seek for minimum ascend pair;
    let mut ii = 0;
    let mut jj = 0;
    for i in 0..n {
      for j in i+1..n{
        if nums[i] < nums[j] {
          ii = i;
          jj = j;
        }
      }
    }
    //println!("ii={} jj={}", ii, jj);

    if ii == jj {
      nums.sort();
    }
    nums.swap(ii,jj);
    nums[ii+1..].sort();
  }
}

fn main() {
  let mut nums = vec!(1,2,3,4);
  let mut i  = 0;
  loop {
    println!("{:?}", nums);
    Solution::next_permutation(&mut nums);
    i+=1;
    if i > 30 {
      break;
    }
  }
}
