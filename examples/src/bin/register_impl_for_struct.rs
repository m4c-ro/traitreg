#![allow(unused)]
// If it compiles it works
fn main() {}



trait MyTrait {}

struct MyStruct;

#[traitreg::register]
impl MyTrait for MyStruct {}
