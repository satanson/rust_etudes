struct Solution {}

impl Solution {
  pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    match n {
      0 => { return vec!(); }
      1 => { return vec!(nums); }
      _ => {}
    }
    let mut fact_n = 1;
    for i in 1..=n {
      assert!(fact_n < std::usize::MAX / i, "overflow!");
      fact_n *= i;
    }
    const SENTINEL :i32 = std::i32::MIN;
    let mut xs = vec![0; n];
    let mut ans = Vec::with_capacity(fact_n);
    for _ in 0..fact_n {
      let mut perm = vec![SENTINEL; n];
      for (k, &p) in xs.iter().enumerate().rev() {
        let k = n - 1 - k;
        let mut skip = 0;
        let mut m = 0;
        while skip <= p {
          if perm[m] == SENTINEL {
            skip+=1;
          }
          m+=1;
        }
        perm[m-1] = nums[k];
        //println!("xs={:?} perm={:?} nums={:?} p={} k={} m={} perm[{}]={} nums[{}]={}", &xs, &perm,&nums, p, k, m, m, perm[m],k, nums[k]);
      }

      let mut carry = 1;
      for i in 0..n {
        xs[i] += carry;
        carry = xs[i]/(i+1);
        xs[i] %= (i+1);
        if carry==0 {
          break;
        }
      }
      ans.push(perm);
    }
    return ans;
  }
}

fn main() {
  println!("{:?}", Solution::permute(vec!(1,2,3,4)));
}
