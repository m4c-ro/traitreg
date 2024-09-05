fn main() {
    for registered in MYTRAIT_REGISTRY.iter() {
        println!("{registered:#?}");

        // Get metadata of registered impl
        let _ = registered.name();
        let _ = registered.path();
        let _ = registered.file();
        let _ = registered.trait_name();
        let _ = registered.module_path();

        // Create an instance of the type if a constructor is registered
        if let Some(instance) = registered.instanciate() {
            // Use Debug, a supertrait of 'MyTrait'
            println!("Instance: {instance:#?}");

            // Call a trait method
            println!("FOO: {}", instance.foo());
        }
    }
}

trait MyTrait: std::fmt::Debug {
    fn foo(&self) -> &'static str;
}

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    pub fn new() -> Self {
        Self
    }
}

#[traitreg::register(new)]
impl MyTrait for MyStruct {
    fn foo(&self) -> &'static str {
        "BAR"
    }
}

#[derive(Debug)]
enum MyEnum {
    #[allow(unused)]
    MyEnumVariant,
}

#[traitreg::register]
impl MyTrait for MyEnum {
    fn foo(&self) -> &'static str {
        "BAZ"
    }
}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();
