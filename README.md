# Traitreg

Create a registry of implementations of a trait. Useful for all kinds of metaprogramming, but in
particular can be used for:

* Dependency injection (at runtime)
* Registration for plugins or middleware
* Any code which needs to do _something_ for a number of types

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

### Implementation Details

The registry is built during startup by methods called by the linker, before `main()` is
called. This approach is very much platform dependent but avoids issues with other approaches
which run at compile-time but are unsound.

Notably multiple crates (i.e. compilation units) can register implementations independently,
the registry will pick up all of the impls automatically at runtime. This can be useful for a
plugin system where shared libraries (`cdylib` crates) are loaded. Currently loading shared
libraries manually after `main()` is called will not update the registry.

It is possible to build a registry like this purely at compile time using procedural macros
but as far as I am aware this is unsound. Each proc macro invocation currently reuses the same
proc-macro executable in-memory without reloading it, so state _can_ be persisted in static
memory, but storing state across several independent macro calls is not supported by rustc and
this behaviour may break in the future.

### Similar / Previous Work

* <https://github.com/dtolnay/inventory>
* <https://github.com/mmastrac/rust-ctor>
* <https://github.com/DouglasDwyer/wings>
