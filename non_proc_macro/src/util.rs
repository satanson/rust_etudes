extern crate my_proc_macro;
use self::my_proc_macro::*;
pub fn foobar() {
  println!("hello world")
}

#[test]
fn test_foobar(){
  foobar()
}

struct Buzz;
struct BuzzIter {
  v:i32
}

impl IntoIterator for Buzz {
  type Item = i32;
  type IntoIter = BuzzIter;

  fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
    BuzzIter{v:0}
  }
}

impl Iterator for BuzzIter {
  type Item = i32;
  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    let v = self.v;
    match v {
      10 => {
        None
      }
      _ => {
        self.v+=1;
        Some(v)
      }
    }
  }
}

#[test]
fn test_buzz() {
  let buzz = Buzz;
  for x in buzz {
    println!("x={}", x)
  }
}

#[derive(Aaa, Debug)]
pub struct AAA_struct{
  #[aaa_helper] pub a:i32
}

#[cfg(feature="f0")]
pub fn is_f0_enabled() {
  println!("f0 enabled")
}

#[cfg(not(feature="f0"))]
pub fn is_f0_enabled() {
  println!("f0 not enabled")
}
