//! # Traitreg
//!
//! Create a registry of implementations of a trait. Useful for all kinds of metaprogramming, but in
//! particular can be used for:
//!
//! * Dependency injection (at runtime)
//! * Registration for plugins or middleware
//! * Any code which needs to do _something_ for a number of types
//!
//! ### API
//!
//! Register a trait implementation
//!
//! ```rust
//! trait MyTrait {}
//! struct MyType;
//!
//! #[traitreg::register]
//! impl MyTrait for MyType {}
//! ```
//!
//! Optionally: register with a constructor
//!
//! ```rust
//! trait MyTrait {}
//!
//! #[derive(Default)]
//! struct MyType;
//!
//! #[traitreg::register(default)]
//! impl MyTrait for MyType {}
//!
//! struct MyOtherType;
//! impl MyOtherType {
//!     fn new() -> Self { Self }
//! }
//!
//! #[traitreg::register(new)]
//! impl MyTrait for MyOtherType {}
//! ```
//!
//! Build a static registry of all registered trait implementations at compile-time.
//!
//! ```rust
//! # trait MyTrait {}
//! #[traitreg::registry(MyTrait)]
//! static MYTRAIT_REGISTRY: () = ();
//! ```
//!
//! Access registry contents. See [TraitRegStorage].
//!
//! ```rust
//! # trait MyTrait {}
//! # #[traitreg::registry(MyTrait)]
//! # static MYTRAIT_REGISTRY: () = ();
//! // Enumerate impls
//! for reg in MYTRAIT_REGISTRY.iter() {
//!     println!("{reg:#?}");
//!
//!     // Instanciate
//!     let instance: Option<Box<dyn MyTrait>> = reg.instanciate();
//! }
//! ```
//!
//! ### Implementation Details
//!
//! The registry is built during startup by methods called by the linker, before `main()` is
//! called. This approach is very much platform dependent but avoids issues with other approaches
//! which run at compile-time but are unsound.
//!
//! Notably multiple crates (i.e. compilation units) can register implementations independently,
//! the registry will pick up all of the impls automatically at runtime. This can be useful for a
//! plugin system where shared libraries (`cdylib` crates) are loaded. Currently loading shared
//! libraries manually after `main()` is called will not update the registry.
//!
//! It is possible to build a registry like this purely at compile time using procedural macros
//! but as far as I am aware this is unsound. Each proc macro invocation currently reuses the same
//! proc-macro executable in-memory without reloading it, so state _can_ be persisted in static
//! memory, but storing state across several independent macro calls is not supported by rustc and
//! this behaviour may break in the future.
//!
//! ### Similar / Previous Work
//!
//! * <https://github.com/dtolnay/inventory>
//! * <https://github.com/mmastrac/rust-ctor>
//! * <https://github.com/DouglasDwyer/wings>
#![forbid(missing_docs)]
// Refs:
//
// https://maskray.me/blog/2021-11-07-init-ctors-init-array
// https://github.com/mmastrac/rust-ctor
// https://docs.rs/bevy_type_registry/0.3.0/bevy_type_registry/
// https://github.com/DouglasDwyer/wings/tree/master

// TODO:
//      - Initialization order is not guaranteed on apple platforms
//      - Deconflict type/trait names (get full path?)
//      - Return custom iter type for iter_constructors method

pub use traitreg_macros::{register, registry};

static __REGISTRY: std::sync::Mutex<Vec<RegisteredImplWrapper<Box<u32>>>> =
    std::sync::Mutex::new(vec![]);

#[doc(hidden)]
pub trait RegisteredImpl<Trait> {
    const INSTANCIATE: fn() -> Option<Trait>;
    const NAME: &'static str;
    const PATH: &'static str;
    const FILE: &'static str;
    const MODULE_PATH: &'static str;
    const TRAIT_NAME: &'static str;
}

#[doc(hidden)]
pub fn __register_impl<Trait, Type: RegisteredImpl<Trait>>() {
    let wrapper = RegisteredImplWrapper::<Trait> {
        instanciate: Type::INSTANCIATE,
        name: Type::NAME,
        path: Type::PATH,
        file: Type::FILE,
        module_path: Type::MODULE_PATH,
        trait_name: Type::TRAIT_NAME,
    };

    // Safety: Access to this type would be UB, but we only access this value after transmuting it
    // back to the original type. In the mean time storing a fn ptr with a different signature will
    // not modify the memory layout of RegisteredImplWrapper, so it is safe to store in a Vec.
    let wrapper: RegisteredImplWrapper<Box<u32>> = unsafe { core::mem::transmute(wrapper) };

    let mut registry_ref = __REGISTRY.lock().expect("Traitreg internal mutex poisoned");
    registry_ref.push(wrapper);
}

/// Trait registry storage. Contains methods to access the registry.
pub struct TraitRegStorage<Trait> {
    impls: Vec<RegisteredImplWrapper<Trait>>,
}

impl<Trait> TraitRegStorage<Trait> {
    #[doc(hidden)]
    pub fn __new(trait_: &'static str) -> Self {
        let registry_ref = __REGISTRY.lock().expect("Traitreg internal mutex poisoned");

        let impls = registry_ref
            .iter()
            .filter(|item| item.trait_name == trait_)
            .cloned()
            .map(|item| {
                // Safety: Since we check the trait name before transmuting back we cannot accidentally
                // construct a trait object pointing to a different vtable in memory
                let item: RegisteredImplWrapper<Trait> = unsafe { core::mem::transmute(item) };
                item
            })
            .collect();

        Self { impls }
    }

    /// Iterate over registered implementations
    pub fn iter(&self) -> core::slice::Iter<RegisteredImplWrapper<Trait>> {
        self.impls.iter()
    }
}

/// Registered implementation
#[derive(Clone)]
pub struct RegisteredImplWrapper<Trait> {
    instanciate: fn() -> Option<Trait>,
    name: &'static str,
    path: &'static str,
    file: &'static str,
    module_path: &'static str,
    trait_name: &'static str,
}

impl<Trait> RegisteredImplWrapper<Trait> {
    /// Instanciate type if a constructor has been registered
    ///
    /// Returns a heap allocated trait object, `Box<dyn Trait>`, rather than a
    /// concrete type.
    pub fn instanciate(&self) -> Option<Trait> {
        (self.instanciate)()
    }

    /// The type name
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The type path. This differs from name when the implementation block is in a different crate
    /// of module than the type itself. e.g. `MyType` vs `other::module::OtherType`.
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The file containing the implementation of the trait
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// The module containing the implementation of the trait
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The trait name
    pub fn trait_name(&self) -> &'static str {
        self.trait_name
    }
}

impl<Trait> core::fmt::Debug for RegisteredImplWrapper<Trait> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        f.debug_struct("RegisteredImpl")
            .field("Type Name", &self.name)
            .field("Type Path", &self.path)
            .field("Trait Name", &self.trait_name)
            .field("Has Constructor", &(self.instanciate)().is_some())
            .field("Module Path", &self.module_path)
            .field("File", &self.file)
            .finish()
    }
}
