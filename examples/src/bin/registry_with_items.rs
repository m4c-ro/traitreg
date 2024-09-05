#![allow(unused)]
// If it compiles it works
fn main() {
    for registered in MYTRAIT_REGISTRY.iter() {
        eprintln!("{:#?}", registered);
    }
}



trait MyTrait: std::fmt::Debug {}

#[derive(Debug)]
struct MyStruct;

#[derive(Debug)]
enum MyEnum { MyEnumVariant }
impl MyEnum {
    pub fn new() -> Self { Self::MyEnumVariant }
}

#[derive(Default, Debug)]
struct Dummy;
type MyType = Dummy;

union MyUnion {
    f: f32,
    u: u32,
}
impl MyUnion {
    pub fn new() -> Self { Self { f: 0.0 } }
}
impl std::fmt::Debug for MyUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "MyUnion")
    }
}



#[traitreg::register]
impl MyTrait for MyStruct {}
#[traitreg::register(new)]
impl MyTrait for MyEnum {}
#[traitreg::register(default)]
impl MyTrait for MyType {}
#[traitreg::register(new)]
impl MyTrait for MyUnion {}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();
