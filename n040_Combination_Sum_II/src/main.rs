struct Solution {}

impl Solution {
  pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if candidates.is_empty() {
      return vec!();
    }
    let mut cand = candidates;
    cand.sort();
    return Self::p(&cand, cand.len(), target);
  }
  pub fn p(cand: &Vec<i32>, k: usize, target: i32) -> Vec<Vec<i32>> {
    if k == 1 {
      let a0 = cand[0];
      if a0 == target {
        return vec!(vec!(a0));
      } else {
        return vec!();
      }
    }
    let mut ans: Vec<Vec<i32>> = vec!();
    let ak_1 = cand[k - 1];
    if ak_1 > target {
      ans.extend(Self::p(cand, k - 1, target));
    } else if ak_1 == target {
      ans.push(vec!(ak_1));
      ans.extend(Self::p(cand, k - 1, target));
    } else {
      ans.extend(Self::p(cand, k - 1, target));
      let mut ans2 = Self::p(cand, k-1, target-ak_1);
      ans2.iter_mut().for_each(|x|x.push(ak_1));
      ans.extend(ans2);
    }
    use std::collections::HashSet;
    let mut dedup:HashSet<Vec<i32>> = HashSet::new();
    for a in ans {
      dedup.insert(a);
    }
    return dedup.iter().map(|x|x.clone()).collect::<Vec<Vec<i32>>>();
  }
}

fn main() {
  println!("{:?}", Solution::combination_sum2(vec!(10, 1, 2, 7, 6, 1, 5), 8));
}
