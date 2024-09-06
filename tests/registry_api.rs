trait MyTrait {
    fn foo(&self) -> u32;
}

#[derive(Default)]
struct MyStruct;

#[traitreg::register(default)]
impl MyTrait for MyStruct {
    fn foo(&self) -> u32 {
        123
    }
}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();

enum MyEnum {
    #[allow(unused)]
    MyEnumVariant,
}

#[traitreg::register]
impl MyTrait for MyEnum {
    fn foo(&self) -> u32 {
        456
    }
}

#[test]
fn main() {
    assert_eq!(2, MYTRAIT_REGISTRY.iter().count());
    assert_eq!(1, MYTRAIT_REGISTRY.instanciate_all().count());

    let instance = MYTRAIT_REGISTRY.instanciate_all().next().unwrap();
    assert_eq!(instance.foo(), 123);
}
