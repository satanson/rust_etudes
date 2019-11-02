struct Solution {}

extern crate common;

use common::sudoku;

use std::u16;
use std::cmp::Ordering;
use std::collections::HashSet;

struct CellAndMissing((usize, usize), u16);

pub fn sudoku_from(d: [[&str; 9]; 9]) -> Vec<Vec<char>> {
  return d.iter().map(|r| {
    r.iter().map(|&s| {
      s.as_bytes()[0] as char
    }).collect()
  }).collect();
}

impl PartialEq for CellAndMissing {
  fn eq(&self, other: &Self) -> bool {
    return (self.1).count_ones() == (other.1).count_ones();
  }
}

impl Eq for CellAndMissing {}

impl Ord for CellAndMissing {
  fn cmp(&self, other: &Self) -> Ordering {
    return (other.1).count_ones().cmp(&(self.1).count_ones());
  }
}

impl PartialOrd for CellAndMissing {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(other.cmp(self));
  }
}

use std::fmt::{Debug, Formatter, Result};

impl Debug for CellAndMissing {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "CellAndMissing:[{},{}],{:b},{:?}\n",
           (self.0).0, (self.0).1, self.1, which_bit(self.1))
  }
}

fn which_bit(x: u16) -> HashSet<char> {
  (1..=9)
    .filter(|&i| ((1 & !(x >> i)) == 1))
    .map(|x| ('0' as u8 + x as u8) as char).collect::<HashSet<char>>()
}

impl Solution {
  pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    const ROW: usize = 0;
    const COL: usize = 1;
    const NINE: usize = 2;
    let mut back_and_forth: Vec<((usize, usize), HashSet<char>)> = Vec::new();
    loop {
      let mut d: [[u16; 9]; 3] = [[0; 9]; 3];

      for i in 0..9 {
        for j in 0..9 {
          let c0 = board[i][j];
          let c1 = board[j][i];
          let c2 = board[(i / 3) * 3 + j / 3][(i % 3) * 3 + j % 3];
          for (k, &c) in vec!(c0, c1, c2).iter().enumerate() {
            match c {
              '0'...'9' => {
                let m = c as u8 - '0' as u8;
                d[k][i] |= 1 << m;
              }
              _ => {}
            }
          }
        }
      }
      use std::collections::BinaryHeap;
      //let mut dd: BinaryHeap<CellAndMissing> = BinaryHeap::with_capacity(81);
      let mut dd: Vec<CellAndMissing> = Vec::with_capacity(81);
      for i in 0..9 {
        for j in 0..9 {
          let c = board[i][j];
          if c != '.' {
            continue;
          }
          let c0 = d[ROW][i];
          let c1 = d[COL][j];
          let c2 = d[NINE][(i / 3) * 3 + j / 3];
          let b = c0 | c1 | c2;
          dd.push(CellAndMissing((i, j), b));
        }
      }
      if dd.is_empty() {
        return;
      }
      dd.sort();
      loop {
        //println!("in-doubt: {}", dd.len());
        //std::thread::sleep(std::time::Duration::from_millis(10));
        match dd.last() {
          Some(&CellAndMissing((i, j), bits)) => {
            let mut cset = which_bit(bits);
            //println!("in-doubt={} [{},{}]: {:b}, ones={}, c={:?}\nback_and_forth={:?}\ndd={:?}",
            //         dd.len(), i, j, bits, bits.count_ones(), &cset, &back_and_forth,
            //         dd.iter().rev().collect::<Vec<&CellAndMissing>>());
            //std::thread::sleep(std::time::Duration::from_millis(100));
            match bits.count_ones() {
              9 => {
                loop {
                  match back_and_forth.pop() {
                    Some(((i, j), mut cset)) => {
                      board[i][j] = '.';
                      if !cset.is_empty() {
                        let c = *cset.iter().next().unwrap();
                        cset.remove(&c);
                        board[i][j] = c;
                        back_and_forth.push(((i, j), cset));
                        break;
                      }
                    }
                    None => {
                      break;
                    }
                  }
                }
                break;
              }
              8 => {
                let mut cset = which_bit(bits);
                let c = *cset.iter().next().unwrap();
                cset.remove(&c);
                //println!("[{},{}] choose {}, cset={:?}", i, j, c, &cset);
                back_and_forth.push(((i, j), cset));
                board[i][j] = c;
                dd.pop();
                break;
              }
              _ => {
                let c = *cset.iter().next().unwrap();
                cset.remove(&c);
                //println!("[{},{}] choose {}, cset={:?}", i, j, c, &cset);
                back_and_forth.push(((i, j), cset));
                board[i][j] = c;
                dd.pop();
                break;
              }
            }
          }
          None => {
            break;
          }
        }
      }
    }
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
  let b1 = [
    [".", ".", "9", "7", "4", "8", ".", ".", "."],
    ["7", ".", ".", ".", ".", ".", ".", ".", "."],
    [".", "2", ".", "1", ".", "9", ".", ".", "."],
    [".", ".", "7", ".", ".", ".", "2", "4", "."],
    [".", "6", "4", ".", "1", ".", "5", "9", "."],
    [".", "9", "8", ".", ".", ".", "3", ".", "."],
    [".", ".", ".", "8", ".", "3", ".", "2", "."],
    [".", ".", ".", ".", ".", ".", ".", ".", "6"],
    [".", ".", ".", "2", "7", "5", "9", ".", "."]
  ];

  let b2 = [
    [".", ".", "9", "7", "4", "8", ".", ".", "."],
    ["7", ".", ".", ".", ".", ".", ".", ".", "."],
    [".", "2", ".", "1", ".", "9", ".", ".", "."],
    [".", ".", "7", ".", ".", ".", "2", "4", "."],
    [".", "6", "4", ".", "1", ".", "5", "9", "."],
    [".", "9", "8", ".", ".", ".", "3", ".", "."],
    [".", ".", ".", "8", ".", "3", ".", "2", "."],
    [".", ".", ".", ".", ".", ".", ".", ".", "6"],
    [".", ".", ".", "2", "7", "5", "9", ".", "."]
  ];
  let mut b0 = sudoku_from(b0);
  let mut b1 = sudoku_from(b1);
  let mut b2 = sudoku_from(b2);
  //Solution::solve_sudoku(&mut b0);
  Solution::solve_sudoku(&mut b2);
  sudoku::is_valid_sudoku(&b2);
  println!("is_valid={} {:?}", sudoku::is_valid_sudoku(&b2), b2);
}
