extern crate rust_etudes;
use rust_etudes::util::*;

fn main() {
  let a = Some(10)
    .map(|x| {
      x + 1
    })
    .map(|x| {
      x.to_string()
    }).unwrap();

  println!("Hello, world!{}", a);
  foobar();
  let aaa = AAA_struct{a:10};
  println!("aaa={:?}", aaa);
  is_f0_enabled();
}
