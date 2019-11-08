struct Solution {}
type Pos = (i32, i32);

struct StkElm {
  pos: Pos,
  alternatives: Vec<Pair>
}
impl StkElm {
  fn new()->Box<StkElm> {
    Box::new(StkElm{pos:(0,0), alternatives:vec!()})
  }
}

impl Solution {
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    match n {
      0 | 2 | 3 => {
        return vec!();
      }
      1 => {
        return vec!(vec!(format!("Q")));
      }
      x if x < 0 => {
        return vec!();
      }
      _ => {}
    }

    let hn = (n + 1) / 2;
    let mut stk = vec!(StkElm::new());
    stk.last_mut().unwrap().pos = (0, hn-1);
    stk.last_mut().unwrap().alternatives.extend((0..hn).map(|x|(0, x)));

    
    let mut path =
    return vec!();
  }
}

fn main() {
  println!("Hello, world!");
}
