struct Solution {}

impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::ops::AddAssign;
    use std::collections::HashMap;
    let f = |x: char, d: &mut HashMap<char, i32>| -> bool{
      match x {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          let v = d.entry(x).or_insert(0);
          v.add_assign(1);
          if *v > 1 {
            return false;
          }
        }
        '.' => {}
        _ => {
          return false;
        }
      }
      return true;
    };
    // row
    for row in &board {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for &c in row {
        if !f(c, &mut d) {
          return false;
        }
      }
    }

    // column
    for i in 0..9 {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for j in 0..9 {
        if !f(board[j][i], &mut d) {
          return false;
        }
      }
    }
    // 3x3 cell
    for i in 0..9 {
      let mut d: HashMap<char, i32> = std::collections::HashMap::new();
      for j in 0..9 {
        let r0 = i % 3;
        let c0 = i / 3;
        let r1 = j % 3 + 3 * r0;
        let c1 = j / 3 + 3 * c0;
        let c = board[r1][c1];
        if !f(c, &mut d) {
          return false;
        }
      }
    }
    return true;
  }
}

fn main() {
  let b0 = [
    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"]
  ];
  let b0: Vec<Vec<char>> = b0.iter().map(
    |r| {
      r.iter().map(
        |&c| {
          c.as_bytes()[0] as char
        }).collect()
    }).collect();
  let b1 = [
    ["8", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"]
  ];

  let b1: Vec<Vec<char>> = b1.iter().map(
    |r| {
      r.iter().map(
        |&c| {
          c.as_bytes()[0] as char
        }).collect()
    }).collect();
  println!("{:?}", Solution::is_valid_sudoku(b0));
  println!("{:?}", Solution::is_valid_sudoku(b1));
}
