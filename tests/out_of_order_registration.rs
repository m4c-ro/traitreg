#[test]
fn main() {
    assert_eq!(2, MYTRAIT_REGISTRY.iter().count());
}

trait MyTrait {}

struct MyStruct;

#[traitreg::register]
impl MyTrait for MyStruct {}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();

enum MyEnum {
    #[allow(unused)]
    MyEnumVariant,
}

#[traitreg::register]
impl MyTrait for MyEnum {}
