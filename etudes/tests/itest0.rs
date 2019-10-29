extern crate etudes;
use etudes::util;
#[test]
fn test_add_itest0(){
  assert_eq!(5, util::add(2,3));
}

#[test]
fn test_foobar0(){
  util::foobar0();
}

#[test]
fn test_string_slice(){
  util::string_slice();
}

#[test]
fn lifetime_subtyping_test(){
  util::lifetime_subtyping();
}

#[test]
fn lifetime_bound_test() {
  util::lifetime_bound();
}

#[test]
fn diverging_function_test() {
  util::diverging_function();
}