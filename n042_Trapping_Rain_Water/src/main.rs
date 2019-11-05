struct Solution {}

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 3 {
      return 0;
    }
    let mut volume = 0;
    let mut l = 0;
    let mut r = n - 1;
    let mut left_max  = height[l];
    let mut right_max = height[n-1];

    while l < r {
      if left_max < right_max {
        let left_next = height[l+1];
        if left_next < left_max {
          volume += left_max - left_next;
        } else {
          left_max = left_next;
        }
        l+=1;
      } else {
        let right_prev = height[r-1];
        if right_prev < right_max {
          volume += right_max - right_prev;
        } else {
          right_max = right_prev;
        }
        r-=1;
      }
    }
    return volume;
  }
}

fn main() {
  println!("{}", Solution::trap(vec!()));
  println!("{}", Solution::trap(vec!(1)));
  println!("{}", Solution::trap(vec!(1, 2)));
  println!("{}", Solution::trap(vec!(1, 2, 3)));
  println!("{}", Solution::trap(vec!(3, 2, 1)));
  println!("{}", Solution::trap(vec!(1, 0, 1)));
  println!("{}", Solution::trap(vec!(1, 0, 2)));
  println!("{}", Solution::trap(vec!(3, 0, 2)));
  println!("{}", Solution::trap(vec!(1, 2, 3, 0, 2, 1, 0)));
  println!("{}", Solution::trap(vec!(1, 1, 2, 2, 3, 3)));
  println!("{}", Solution::trap(vec!(0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1)));
}
