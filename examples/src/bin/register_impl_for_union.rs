#![allow(unused)]
// If it compiles it works
fn main() {}



trait MyTrait {}

union MyUnion {
    f: f32,
    u: u32,
}

#[traitreg::register]
impl MyTrait for MyUnion {}
