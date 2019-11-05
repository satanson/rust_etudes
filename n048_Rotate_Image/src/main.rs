struct Solution {}

impl Solution {
  // n = 2k+1; i in (0..k+1), j in (0..k)
  // n = 2k; in in (0..k), j in (0..k)
  // (i,j) => (j, n-1-i)

  pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..(n+1)/2 {
      for j in 0..n/2 {
        let mut i0 = i;
        let mut j0 = j;
        let mut prev = matrix[i][j];
        loop {
          let i1 = j0;
          let j1 = n - 1 -i0;
          let tmp = matrix[i1][j1];
          matrix[i1][j1] = prev;
          prev = tmp;
          i0 = i1;
          j0 = j1;
          if i1 == i && j1 == j {
            break;
          }
        }
      }
    }
  }
}

fn main() {
  let mut matrix =  vec!(vec!(1,2,3),vec!(4,5,6),vec!(7,8,9));
  Solution::rotate(&mut matrix);
  println!("{:?}", &matrix);
}
