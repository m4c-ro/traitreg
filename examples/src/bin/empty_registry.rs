#![allow(unused)]
// If it compiles it works
fn main() {}



trait MyTrait {}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: _ = _;
