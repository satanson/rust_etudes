struct Solution {}

impl Solution {
  pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut min_i, mut max_i) = (0usize, 0usize);
    let (mut min, mut max) = (std::i32::MAX, std::i32::MIN);

    for (i, &x) in nums.iter().enumerate() {
      if x <= 0 {
        continue;
      }
      if x > max {
        max_i = i;
        max = x;
      }
      if x < min {
        min_i = i;
        min = x;
      }
    }

    if min > 1 {
      return 1;
    }

    let mut nums = nums;
    nums.swap(0, min_i);
    let mut i = 1;
    loop {
      if i >= n {
        break;
      }
      let num = nums[i];
      println!("i={} num={} nums={:?}", i, num, &nums);
      // ignore larger integer
      if num >= min + n as i32 || num <= 0 {
        nums[i] = -1;
        i += 1;
        continue;
      }
      // right position
      let proper_i = (num - min) as usize;
      if proper_i == i {
        i += 1;
        continue;
      }
      if nums[i] == nums[proper_i] {
        nums[i] = -1;
        i += 1;
      } else {
        nums.swap(i, proper_i);
      }
    }
    for i in 1..n {
      if nums[i] == -1 {
        return min + i as i32;
      }
    }
    return min + n as i32;
  }
}

fn main() {
  //println!("{}", Solution::first_missing_positive(vec!(1, 2, 3)));
  //println!("{}", Solution::first_missing_positive(vec!(2, 3, -1, 4)));
  //println!("{}", Solution::first_missing_positive(vec!(1, 2, 3, 7, 8, 9)));
  //println!("{}", Solution::first_missing_positive(vec!(3, 4, -1, 1)));
  //println!("{}", Solution::first_missing_positive(vec!(1, 1)));
  //println!("{}", Solution::first_missing_positive(vec!(0, 1, 2)));
  //println!("{}", Solution::first_missing_positive(vec!(1, 3, 3)));
  println!("{}", Solution::first_missing_positive(vec!(3, 4, -1, 1)));

}
