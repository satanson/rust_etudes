struct Solution {}

impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    if n == 0 {
      return vec!(-1, -1);
    }
    if n == 1 {
      if nums[0] == target {
        return vec!(0, 0);
      } else {
        return vec!(-1, -1);
      }
    }
    let (mut l, mut r) = (0, n - 1);
    while l < r {
      let m = (l + r) / 2;
      let mid = nums[m];
      //println!("leftmost: left: nums[{}]={} mid: nums[{}]={} right: nums[{}]={}", l, nums[l], m, mid, r, nums[r]);
      //std::thread::sleep(std::time::Duration::from_millis(500));
      if mid == target {
        if l == m {
          break;
        } else if nums[m - 1] == target {
          r = m - 1;
        } else {
          r = m;
        }
      } else if mid < target {
        if m < r {
          l = m + 1
        } else {
          break;
        }
      } else {
        if m > l {
          r = m - 1;
        } else {
          break;
        }
      }
    }

    let leftmost = l;
    if nums[leftmost] != target {
      return vec!(-1, -1);
    }

    let (mut l, mut r) = (0, n - 1);
    while l < r {
      let m = (l + r) / 2;
      let mid = nums[m];
      //println!("rightmost: left: nums[{}]={} mid: nums[{}]={} right: nums[{}]={}", l, nums[l], m, mid, r, nums[r]);
      //std::thread::sleep(std::time::Duration::from_millis(500));
      if mid == target {
        if m == l {
          if nums[r] == target {
            break;
          } else {
            r = l;
            break;
          }
        } else if nums[m + 1] == target {
          l = m + 1;
        } else {
          l = m;
        }
      } else if mid < target {
        if m < r {
          l = m + 1;
        } else {
          break;
        }
      } else {
        if m > l {
          r = m - 1;
        } else {
          break;
        }
      }
    }
    let rightmost = r;
    // find rightmost
    return vec!(leftmost as i32, rightmost as i32);
  }
}

fn main() {
  //println!("{:?}", Solution::search_range(vec!(2, 2, 2), 2));
  println!("{:?}", Solution::search_range(vec!(2, 2, 2, 3), 2));
  println!("{:?}", Solution::search_range(vec!(0, 1, 2, 2, 2), 2));
  println!("{:?}", Solution::search_range(vec!(0, 1, 2, 2, 2, 3, 4, 5), 2));
}
