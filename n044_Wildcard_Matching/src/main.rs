struct Solution {}

impl Solution {
  // D(n,m):
  // p[m-1]='*'=>D(n,m-1)|| D(n-1, m)
  // p[m-1]='?'=>D(n-1,m-1)
  // p[m-1]='c'=>if p[n-1]==s[m-1] {D(n-1, m-1)} else {false}

  // D(0,0) = true
  // D(0,m) && m>=1 => if (all p[0..n] == '*'){true}else {false};
  // D(n,0) && n>=1 => false;
  pub fn is_match(s: String, p: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let p = p.chars().collect::<Vec<char>>();
    let n = s.len();
    let m = p.len();
    // initialize 2d vector with single value
    let mut d = vec![vec![false;m+1];n+1];
    d[0][0] = true;
    for i in 1..=n {
      d[i][0] = false;
    }

    for j in 1..=m {
      d[0][j] = d[0][j-1] && p[j-1] == '*';
    }
    for i in 1..=n {
      for j in 1..=m {
        match p[j-1] {
          '?'=>{
            d[i][j] = d[i-1][j-1];
          }
          '*'=>{
            d[i][j] = d[i-1][j]||d[i][j-1];
          }
          c =>{
            d[i][j] = s[i-1] == c && d[i-1][j-1];
          }
        }
      }
    }
    return d[n][m];
  }
}

fn main() {
  //[format!("a?b*"),format!("abcd"), format!("a*"), format!("a???")].iter().for_each(|x|{
  //  println!("{},{},{}", format!("abcd"), x, Solution::is_match(format!("abcd"), x.clone()));
  //  println!("{},{},{}", format!("abc"), x, Solution::is_match(format!("abc"), x.clone()));
  //});
  //println!("{}", Solution::is_match(format!("adceb"),format!("*a*b")));
  println!("{}", Solution::is_match(format!("a"),format!("a*")));
}
