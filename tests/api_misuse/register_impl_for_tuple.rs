fn main() {}



trait MyTrait {}

#[traitreg::register]
impl MyTrait for (u32,) {}
