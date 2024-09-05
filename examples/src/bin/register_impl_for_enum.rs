#![allow(unused)]
// If it compiles it works
fn main() {}



trait MyTrait {}

enum MyEnum {}

#[traitreg::register]
impl MyTrait for MyEnum {}
