#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn test0(){
  assert_eq!(4,3, "{}", "failure");
}

#[test]
fn test_add(){
  use super::util;
  assert_eq!(10, util::add(5,5));
}