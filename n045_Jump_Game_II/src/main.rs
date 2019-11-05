struct Solution {}

impl Solution {
  pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
      return -1;
    }

    let mut visited = vec![false; n];
    let mut cost = vec![std::i32::MAX; n];
    cost[0] = 0;
    while !visited[n - 1] {
      let mut min_cost = std::i32::MAX;
      let mut min_i = 0;
      for (i, &c) in cost.iter().enumerate() {
        if !visited[i] && c < min_cost {
          min_i = i;
          min_cost = c;
          visited[i] = true;
        }
      }

      for i in (min_i + 1)..=std::cmp::min(min_i + nums[min_i] as usize, n - 1) {
        if !visited[i] && min_cost + 1 < cost[i] {
          cost[i] = min_cost + 1;
        }
      }
    }
    return cost[n - 1];
  }
}

fn main() {
  println!("{}", Solution::jump(vec!(2,3,1,1, 4)));
}
