# TraitReg

### API

Register a trait implementation

```rust
trait MyTrait {}
struct MyType;

#[traitreg::register]
impl MyTrait for MyType {}
```

Optionally: register with a constructor

```rust
trait MyTrait {}

#[derive(Default)]
struct MyType;

#[traitreg::register(default)]
impl MyTrait for MyType {}

struct MyOtherType;
impl MyOtherType {
    fn new() -> Self { Self }
}

#[traitreg::register(new)]
impl MyTrait for MyOtherType {}
```

Build a static registry of all registered trait implementations at compile-time.

```rust
#[traitreg::registry(MyTrait)]
static MYTRAIT_REGISTRY: () = ();
```

Access registry contents.

```rust
// Enumerate impls
for reg in MYTRAIT_REGISTRY.iter() {
    println!("{reg:#?}");

    // Instanciate 
    let instance: Option<Box<dyn MyTrait>> = reg.instanciate();
}
```
