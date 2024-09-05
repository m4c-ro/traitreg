fn main() {}



trait MyTrait {}

struct Dummy;

type MyType = Dummy;

#[traitreg::register]
impl MyTrait for MyType {}
