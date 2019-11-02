struct Solution {}

impl Solution {
  pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if candidates.is_empty() {
      return Vec::new();
    }
    let mut cand = candidates.clone();
    cand.sort();
    return Self::combination_sum_help(&cand, cand.len(), target);
  }

  pub fn combination_sum_help(cand: &Vec<i32>, k: usize, target: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    if k == 1 {
      let a0 = cand[0];
      if target % a0 == 0 {
        ans.push(vec!());
        ans.last_mut().unwrap().resize((target / a0) as usize, a0);
      }
      return ans;
    }

    let ak_1 = cand[k - 1];
    if ak_1 > target {
      ans.extend(Self::combination_sum_help(cand, k - 1, target));
    } else if ak_1 == target {
      ans.push(vec!(ak_1));
      ans.extend(Self::combination_sum_help(cand, k - 1, target));
    } else {
      let mut ans2 = Self::combination_sum_help(cand, k, target - ak_1);
      ans2.iter_mut().for_each(|x| x.push(ak_1));
      ans.extend(ans2);
      ans.extend(Self::combination_sum_help(cand, k - 1, target));
    }
    return ans;
  }
}

fn main() {
  println!("{:?}", Solution::combination_sum(vec!(2, 3, 6, 7), 7));
  println!("{:?}", Solution::combination_sum(vec!(2, 3, 5), 8));
}
