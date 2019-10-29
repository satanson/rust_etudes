use std::cmp;

struct Solution {}

impl Solution {
  pub fn seek_kth(ax: &Vec<i32>, bx: &Vec<i32>, k: usize) -> i32 {
    let a_len = ax.len();
    let b_len = bx.len();

    assert_eq!(a_len > 0 || b_len > 0, true);
    assert_eq!(0 <= k && k < a_len + b_len, true);

    if a_len == 0 {
      return bx[k];
    }
    if b_len == 0 {
      return ax[k];
    }

    if k < b_len && ax[0] >= bx[k] {
      return bx[k];
    }

    if k < a_len && bx[0] >= ax[k] {
      return ax[k];
    }

    let mut l = 0;
    let mut h = k - 1;
    while h - l > 1 {
      let am = (l + h) / 2;
      let bm = k - am - 1;
      if am >= a_len {
        h = a_len - 1;
      } else if bm >= b_len {
        l = am + 1;
      } else {
        if ax[am] > bx[bm] {
          h = am;
        } else if ax[am] < bx[bm] {
          l = am;
        } else {
          return ax[am];
        }
      }
    }

    if h == l {
      return cmp::max(ax[h], bx[k - h - 1]);
    } else {
      let h1 = k - l - 1;
      let l1 = k - h - 1;
      if h >= a_len {
        return cmp::max(ax[l], bx[h1]);
      } else if h1 >= b_len {
        return cmp::max(ax[h], bx[l1]);
      } else {
        return cmp::min(
          cmp::max(ax[l], bx[k - l - 1]),
          cmp::max(ax[h], bx[k - h - 1]),
        );
      }
    }
  }

  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let a_len = nums1.len();
    let b_len = nums2.len();
    let k0 = (a_len + b_len - 1) / 2;
    let k1 = (a_len + b_len) / 2;
    let v0 = Solution::seek_kth(&nums1, &nums2, k0);
    let v1 = Solution::seek_kth(&nums1, &nums2, k1);
    return (v0 + v1) as f64 /2.0f64;
  }
}

fn main() {
  let ax = vec!(1, 3, 5, 10, 11, 12);
  let bx = vec!(2, 4, 6, 7, 8, 9);
  for i in 0..12 {
    println!("{}", Solution::seek_kth(&ax, &bx, i))
  }
  println!("{}", Solution::find_median_sorted_arrays(ax,bx))
}
