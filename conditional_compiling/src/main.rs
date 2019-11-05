#[cfg(feature = "enable_foobar")]
fn foo() {
  println!("enable foobar");
}

#[cfg(not(feature = "enable_foobar"))]
fn foo() {
  println!("disable foobar");
}

fn bar(){
  if cfg!(feature="enable_foobar"){
    println!("{}-{}: enable bar", file!(), line!(),)
  } else {
    println!("{}-{}: disable bar", file!(), line!(),)
  }
}
fn main() {
  foo();
  bar();
  println!("Hello, world!");
}
