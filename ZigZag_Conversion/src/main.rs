struct Solution {}

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows==1 {
      return s;
    }
    let ss = s.as_bytes();
    let len = ss.len();
    let mut s2 = String::new();
    s2.reserve(len);
    //line 0
    let mut i = 0;
    loop {
      let k = (2 * (num_rows - 1) * i) as usize;
      if k  >= len {
        break;
      }
      s2.push(ss[k] as char);
      i += 1;
    }

    //line 1~n-2
    for r in 1..num_rows - 1 {
      let mut i = 0;
      let mut down = true;
      loop {
        let k = if down {
          2 * (num_rows - 1) * i + r
        } else {
          2 * (num_rows - 1) * i - r
        } as usize;
        if k >= len {
          break;
        }
        s2.push(ss[k] as char);
        if down {
          i+=1;
        }
        // down!=down; // WRONG
        down=!down;
      }
    }
    //line n-1
    let mut i = 0;
    loop {
      let k = (2 * (num_rows - 1) * i + num_rows - 1) as usize;
      if k >= len {
        break;
      }
      s2.push(ss[k] as char);
      i += 1;
    }
    return s2;
  }
}

fn main() {
  let s1 = String::from("PAYPALISHIRING");
  let s2 = String::from("A");
  //println!("{}", Solution::convert(s1.clone(), 3));
  //println!("{}", Solution::convert(s1.clone(), 4));
  println!("#{}#", Solution::convert(s2.clone(), 1));
}
