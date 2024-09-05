#![allow(unused)]
// If it compiles it _doesn't_ work
fn main() {}



trait MyTrait {}

struct MyStruct;

#[traitreg::register]
impl !MyTrait for MyStruct {}
