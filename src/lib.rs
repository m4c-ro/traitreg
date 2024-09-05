//!
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
#![forbid(missing_docs)]

// Refs:
//
// https://maskray.me/blog/2021-11-07-init-ctors-init-array
// https://github.com/mmastrac/rust-ctor
// https://docs.rs/bevy_type_registry/0.3.0/bevy_type_registry/
// https://github.com/DouglasDwyer/wings/tree/master

// TODO:
//      - Deconflict type/trait names (get full path?)
//      - Return custom iter type for iter_constructors method
//      - Use linker section priority to ensure trait_registry runs after register_impl
//      - Remove unsafe & static mut for sync



pub use traitreg_macros::{register, registry};



static mut __REGISTRY: Vec<RegisteredImplWrapper<Box<u32>>> = vec![];



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

    let wrapper: RegisteredImplWrapper<Box<u32>> = unsafe {
        std::mem::transmute(wrapper)
    };
    
    unsafe {
        __REGISTRY.push(wrapper);
    }
}

#[doc(hidden)]
pub fn __enumerate_impls<Trait>(trait_: &'static str) -> RegisteredImplIter<Trait> {
    RegisteredImplIter::<Trait> {
        inner: unsafe { __REGISTRY.iter() },
        trait_,
        _t: std::marker::PhantomData::<Trait>
    }
}

/// Trait registry storage. Contains methods to access the registry.
#[derive(Default)]
pub struct TraitRegStorage<Trait> {
    impls: Vec<RegisteredImplWrapper<Trait>>,
}

impl<Trait> TraitRegStorage<Trait> {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            impls: vec![]
        }
    }

    #[doc(hidden)]
    pub fn __register_impl(&mut self, type_: RegisteredImplWrapper<Trait>) {
        self.impls.push(type_);
    }

    /// Iterate over registered implementations
    pub fn iter(&self) -> std::slice::Iter<RegisteredImplWrapper<Trait>> {
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

impl<Trait> std::fmt::Debug for RegisteredImplWrapper<Trait> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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



#[doc(hidden)]
pub struct RegisteredImplIter<Trait> {
    inner: std::slice::Iter<'static, RegisteredImplWrapper<Box<u32>>>,
    trait_: &'static str,
    _t: std::marker::PhantomData<Trait>,
}

impl<Trait> Iterator for RegisteredImplIter<Trait> {
    type Item = RegisteredImplWrapper<Trait>;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.inner.by_ref() {
            let item: Self::Item = unsafe {
                std::mem::transmute((*item).clone())
            };

            if item.trait_name == self.trait_ {
                return Some(item);
            }
        }

        None
    }
}
