use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

/// Register an implementation of a trait on a concrete type.
///
/// ```rust
/// trait MyTrait {}
/// struct MyType;
///
/// #[traitreg::register]
/// impl MyTrait for MyType {}
/// ```
///
/// Supports registration of a constructor, which can be any associated method with the signature
/// `fn() -> Self`. For Example:
///
/// ```rust
/// trait MyTrait {}
///
/// #[derive(Default)]
/// struct MyType;
///
/// #[traitreg::register(default)]
/// impl MyTrait for MyType {}
///
/// struct MyOtherType;
/// impl MyOtherType {
///     fn new() -> Self { Self }
/// }
///
/// #[traitreg::register(new)]
/// impl MyTrait for MyOtherType {}
/// ```
#[proc_macro_attribute]
pub fn register(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Read custom / default constructor from attribute if it exists
    let constructor_fn = if attr.is_empty() {
        None
    } else {
        Some(syn::parse_macro_input!(attr as RegisterAttribute))
    };

    let constructor_fn_call_str = if let Some(cfn) = constructor_fn {
        let ident = cfn.constructor_fn_ident;
        quote! {
            Some(Box::new(Self::#ident()))
        }
    } else {
        quote! {
            None
        }
    };

    let item_clone = item.clone();

    let parsed_item = syn::parse_macro_input!(item as RegisterItem);
    let item_impl = parsed_item.item;

    let (trait_not, trait_path, _) = item_impl
        .trait_
        .expect("Can only register an implementation of a trait, 'impl <Trait> for <Type>'.");
    assert!(
        trait_not.is_none(),
        "Cannot register inverted impl trait: 'impl !Trait for Type'."
    );

    let trait_ident = trait_path
        .require_ident()
        .expect("Expected trait in impl block to have an identifier.");
    let trait_name = format!("{trait_ident}");

    let type_path = get_self_type_path(&item_impl.self_ty);
    let type_ident = type_path
        .require_ident()
        .expect("Expected type in impl block to have an identifier.");
    let type_name = format!("{type_ident}");

    let register_static_ident =
        syn::parse_str::<syn::Ident>(format!("{}_{}__Register", type_ident, trait_ident).as_ref())
            .expect("Unable to create identifier");
    let register_static_fn_ident = syn::parse_str::<syn::Ident>(
        format!("{}_{}__RegisterFn", type_ident, trait_ident).as_ref(),
    )
    .expect("Unable to create identifier");

    let mut result: proc_macro::TokenStream = quote! {
        impl traitreg::RegisteredImpl<Box<dyn #trait_path>> for #type_path {
            const INSTANCIATE: fn() -> Option<Box<dyn #trait_path>> = || { #constructor_fn_call_str };
            const NAME: &'static str = #type_name;
            const PATH: &'static str = stringify!(#type_path);
            const FILE: &'static str = core::file!() ;
            const MODULE_PATH: &'static str = core::module_path!();
            const TRAIT_NAME: &'static str = #trait_name;
        }

        #[used]
        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "freebsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "netbsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "openbsd", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "dragonfly", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "illumos", link_section = ".init_array.10000")]
        #[cfg_attr(target_os = "haiku", link_section = ".init_array.10000")]
        #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(windows, link_section = ".CRT$XCT")]
        static #register_static_ident: extern fn() = {
            extern fn #register_static_fn_ident() {
                traitreg::__register_impl::<Box<dyn #trait_path>, #type_path>();
            }
            #register_static_fn_ident
        };
    }.into();

    result.extend(item_clone.clone());

    result
}

/// Create a registry of implementations of a trait
///
/// ```rust
/// trait MyTrait {}
///
/// #[traitreg::registry(MyTrait)]
/// static MYTRAIT_REGISTRY: () = ();
/// ```
#[proc_macro_attribute]
pub fn registry(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let registry_attr = syn::parse_macro_input!(attr as RegistryAttribute);
    let registry_item = syn::parse_macro_input!(item as RegistryItem);

    let trait_ident = registry_attr.trait_ident;
    let item = registry_item.item;

    let trait_name = format!("{trait_ident}");
    let item_ident = item.ident;
    let storage_ident = syn::parse_str::<syn::Ident>(format!("{}__STORAGE", item_ident).as_ref())
        .expect("Unable to create identifier");
    let wrapper_struct_ident =
        syn::parse_str::<syn::Ident>(format!("{}__TraitReg", item_ident).as_ref())
            .expect("Unable to create identifier");
    let build_static_ident =
        syn::parse_str::<syn::Ident>(format!("{}__Build", item_ident).as_ref())
            .expect("Unable to create identifier");
    let build_static_fn_ident =
        syn::parse_str::<syn::Ident>(format!("{}__BuildFn", item_ident).as_ref())
            .expect("Unable to create identifier");

    quote! {
        static mut #storage_ident: Option<traitreg::TraitRegStorage<Box<dyn #trait_ident>>> = None;

        static #item_ident: #wrapper_struct_ident = #wrapper_struct_ident {};

        struct #wrapper_struct_ident;

        impl ::core::ops::Deref for #wrapper_struct_ident {
            type Target = traitreg::TraitRegStorage<Box<dyn #trait_ident>>;
            fn deref(&self) -> &'static traitreg::TraitRegStorage<Box<dyn #trait_ident>> {
                unsafe {
                    #storage_ident.as_ref().unwrap()
                }
            }
        }

        #[used]
        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "freebsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "netbsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "openbsd", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "dragonfly", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "illumos", link_section = ".init_array.20000")]
        #[cfg_attr(target_os = "haiku", link_section = ".init_array.20000")]
        #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(windows, link_section = ".CRT$XCU")]
        static #build_static_ident: extern fn() = {
            extern fn #build_static_fn_ident() {
                let mut storage = traitreg::TraitRegStorage::<Box<dyn #trait_ident>>::new();
                for registered_impl in traitreg::__enumerate_impls(#trait_name) {
                    storage.__register_impl(registered_impl);
                }

                unsafe {
                    #storage_ident = Some(storage)
                }
            }
            #build_static_fn_ident
        };
    }.into()
}

#[derive(Debug)]
struct RegisterAttribute {
    constructor_fn_ident: Ident,
}

impl Parse for RegisterAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            constructor_fn_ident: Ident::parse(input)?,
        })
    }
}

struct RegisterItem {
    item: syn::ItemImpl,
}

impl Parse for RegisterItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: syn::ItemImpl::parse(input)?,
        })
    }
}

#[derive(Debug)]
struct RegistryAttribute {
    trait_ident: Ident,
}

impl Parse for RegistryAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_ident: Ident::parse(input)?,
        })
    }
}

struct RegistryItem {
    item: syn::ItemStatic,
}

impl Parse for RegistryItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: syn::ItemStatic::parse(input)?,
        })
    }
}

fn get_self_type_path(self_ty: &syn::Type) -> &syn::Path {
    if let syn::Type::Path(type_path) = self_ty {
        return &type_path.path;
    }

    let error_type = match self_ty {
        syn::Type::Array(_) => "n array",
        syn::Type::BareFn(_) => " function",
        syn::Type::Group(_) => " group",
        syn::Type::ImplTrait(_) => " trait impl",
        syn::Type::Infer(_) => "n inferred type (_)",
        syn::Type::Macro(_) => " macro",
        syn::Type::Never(_) => " never type",
        syn::Type::Paren(_) => " parenthesis",
        syn::Type::Ptr(_) => " pointer",
        syn::Type::Reference(_) => " reference",
        syn::Type::Slice(_) => " slice",
        syn::Type::TraitObject(_) => " trait object",
        syn::Type::Tuple(_) => " tuple",
        syn::Type::Verbatim(_) => "n unknown syntax",
        _ => unreachable!(),
    };

    panic!(
        "Cannot register implementation on a{}, expected a struct, enum, union or type alias.",
        error_type
    );
}
