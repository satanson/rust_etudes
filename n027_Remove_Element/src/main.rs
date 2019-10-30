struct Solution{}
impl Solution {
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
      return n as i32;
    }
    let mut i = 0;
    let mut k = 0;
    while k < n && nums[k] == val {
      k+=1;
    }

    // if all nums eq val;
    if k==n {
      nums.resize(0, 0);
      return 0;
    }

    if k==0 { // zero-th elm ne val
      i = 0;
      k +=1;
    } else { // zero-th elm ne val
      nums[i] = nums[k];
      k+=1;
    }

    // invariant#0 0<=i < k <n
    // invariant#0 nums[0~i(inclusive)] all ne val
    while k < n {
      if nums[k] == val {
        k+=1;
      } else if i+1 == k {
        i+=1;
        k+=1;
      } else {
        i+=1;
        nums[i] = nums[k];
        k+=1;
      }
    }
    return (i+1) as i32;
  }
}

fn main() {
  println!("{}", Solution::remove_element(&mut vec!(1,1,1),1));
  println!("{}", Solution::remove_element(&mut vec!(1,2,2),2));
  println!("{}", Solution::remove_element(&mut vec!(1,2,3),3));
  println!("{}", Solution::remove_element(&mut vec!(1,2,3),4));
}
