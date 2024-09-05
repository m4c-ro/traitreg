fn main() {
    for registered in MYTRAIT_REGISTRY.iter() {
        eprintln!("{:#?}", registered);
    }
}

trait MyTrait {}

struct MyStruct;

#[traitreg::register]
impl MyTrait for MyStruct {}

#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();
