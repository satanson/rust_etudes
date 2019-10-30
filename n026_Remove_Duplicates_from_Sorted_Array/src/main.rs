struct Solution {}

impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
      return n as i32;
    }
    let mut i = 0;
    let mut k = 1;
    while k < n {
      if nums[k] == nums[i] {
        k += 1;
      } else if i + 1 == k {
        i += 1;
        k += 1;
      } else {
        i += 1;
        nums[i] = nums[k];
        k += 1;
      }
    }
    nums.resize(i+1, 0);
    return (i+1) as i32;
    return 0;
  }
}

fn main() {
  println!("{:?}", Solution::remove_duplicates(&mut vec!()));
  println!("{:?}", Solution::remove_duplicates(&mut vec!(1)));
  println!("{:?}", Solution::remove_duplicates(&mut vec!(1,2,3)));
  println!("{:?}", Solution::remove_duplicates(&mut vec!(1,1,1)));
  println!("{:?}", Solution::remove_duplicates(&mut vec!(1,1,1,2,2,2)));
  println!("{:?}", Solution::remove_duplicates(&mut vec!(1,1,1,2,2,2,3,4,4)));
}
