#![allow(unused)]
// If it compiles it _doesn't_ work
fn main() {}



trait MyTrait {}

#[traitreg::register]
impl MyTrait for (u32,) {}
