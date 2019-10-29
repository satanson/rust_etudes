pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

//pub fn invoke_fn_mut<T>(mut f: T)->String
//  where T: FnMut(&'static str) -> String {
//  f("def")
//}

pub fn invoke_fn_mut<T: Fn(&'static str) -> String>(f: T) -> String {
  f("def")
}

pub fn foobar0() {
  let y = String::from("abc");
  let z = String::from("xyz");
  let inc1 = |x| {
    y.clone() + &z + x
  };
  assert_eq!(String::from("abcxyzdef"), invoke_fn_mut(inc1));
}

#[derive(Debug)]
pub struct MyBox<T> {
  pub v: T,
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &<Self as Deref>::Target {
    return &self.v;
  }
}

pub fn pass_by_ref(r: &str) {
  println!("pass_by_ref: {}", r);
}

use std::fmt::Debug;

pub fn pass_by_value<T: Debug>(v: T) {
  println!("pass_by_value: {:?}", v);
}

pub fn string_slice() {
  let a = MyBox { v: String::from("abc") };
  pass_by_ref(&a);
  pass_by_ref(a.deref());
  let a_ref = &a;
  println!("{:?}", (*a_ref));
  println!("{:?}", a);
  //pass_by_value(*a_ref);
  use std::rc::Rc;
  let rc = Rc::new(String::from("abc"));
  let rc2 = rc.clone();
  let rc3 = Rc::clone(&rc);
  println!("rc refcount={}", Rc::strong_count(&rc));
  println!("rc2 refcount={}", Rc::strong_count(&rc2));
  println!("rc3 refcount={}", Rc::strong_count(&rc3));
  let weak_rc = Rc::downgrade(&rc);
  {
    let maybe_rc4 = weak_rc.upgrade();
    match maybe_rc4 {
      Some(rc4) => {
        println!("{}", rc4);
      }
      None => {
        println!("None");
      }
    }
    drop(rc);
    drop(rc2);
    drop(rc3);
  }

  let maybe_rc4 = weak_rc.upgrade();
  match maybe_rc4 {
    Some(rc4) => {
      println!("{}", rc4);
    }
    None => {
      println!("None");
    }
  }
}

pub struct Leaf<'a> {
  val: &'a String
}

pub struct IntermediateNode<'a, 'b: 'a> {
  leaf: &'a Leaf<'b>
}

pub struct RootNode<'a, 'b: 'a, 'c: 'b> {
  pub intermediate: &'a IntermediateNode<'b, 'c>
}

impl<'a, 'b, 'c> RootNode<'a, 'b, 'c> {
  fn get_leaf_val(&self) -> &'c String {
    return self.intermediate.leaf.val;
  }
}

fn get_leaf_val<'b, 'c>(intermediate: IntermediateNode<'b, 'c>) -> &'c str {
  return &RootNode { intermediate: &intermediate }.get_leaf_val()[1..];
}

pub fn lifetime_subtyping() {
  let val = String::from("hello world");
  let leaf = Leaf { val: &val };
  let intermediate_node = IntermediateNode { leaf: &leaf };
  let leaf_val = get_leaf_val(intermediate_node);
  println!("{}", leaf_val);
}

struct Wrapper<'a, T> (&'a T);

pub fn lifetime_bound() {
  let s = String::from("abcd");
  let mut a = Wrapper(&s);
  let r = &a;
  //let s1 = String::from("efgh");
  //a.0 = &s1;
  println!("{}", r.0);
}

pub fn diverging_function()  {
  let number :Option<i32>= None;
  loop {
    let num: i32 = match number {
      Some(num) => num,
      None => break,
    };
    println!("{}", num);
  }
}