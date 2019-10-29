#![crate_type="proc-macro"]
extern crate proc_macro;
use proc_macro::*;

#[proc_macro_derive(Aaa, attributes(aaa_helper))]
pub fn derive_aaa(tokens:TokenStream)->TokenStream {

  for tk in tokens.clone(){
    println!("tk={}", tk);
  }
  TokenStream::new()
}