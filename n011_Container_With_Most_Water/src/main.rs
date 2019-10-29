struct Solution {}

impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 2 {
      return 0;
    }

    let mut area = std::i32::MIN;
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
      let hl = height[l];
      let hr = height[r];
      let tmp = (r - l) as i32 * std::cmp::min(hl, hr);
      if tmp > area {
        area = tmp;
      }
      if hl < hr {
        l += 1;
      } else {
        r -= 1;
      }
    }
    return area;
  }
}

fn main() {
  println!("{}", Solution::max_area(vec!(1,8,6,2,5,4,8,3,7)));
}
