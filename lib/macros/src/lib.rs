extern crate proc_macro;
use proc_macro::TokenStream;

mod check;
pub(crate) mod literals;
pub(crate) mod utils;
use check::{
    kindcheck::{generate_kindcheck, generate_no_kindcheck},
    normalize::{generate_no_normalize, generate_normalize},
    subtypecheck::{generate_no_subtypecheck, generate_subtypecheck},
    typecheck::generate_typecheck,
};

/// Derive Typecheck for Terms
/// terms need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement Typecheck already
#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    generate_typecheck(input)
}

//// Derive Subtypecheck for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Subtypecheck
#[proc_macro_derive(Subtypecheck, attributes(Lang))]
pub fn derive_subcheck(input: TokenStream) -> TokenStream {
    generate_subtypecheck(input)
}

/// Derive Subtypecheck for Types with no subtyping
/// this always throws an error when the subtypes are checked
#[proc_macro_derive(NoSubtypes, attributes(Lang))]
pub fn derive_no_subcheck(input: TokenStream) -> TokenStream {
    generate_no_subtypecheck(input)
}

/// Derive Kindchecking for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Kindhcheck
#[proc_macro_derive(Kindcheck, attributes(Lang))]
pub fn derive_kindcheck(input: TokenStream) -> TokenStream {
    generate_kindcheck(input)
}

/// Derive Kindcheck for Types with no Kinds
/// this always throws an error when the kinds are checked
#[proc_macro_derive(NoKinds, attributes(Lang))]
pub fn derive_no_kindcheck(input: TokenStream) -> TokenStream {
    generate_no_kindcheck(input)
}

/// Derive Normalize for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Normalize
#[proc_macro_derive(Normalize, attributes(Lang))]
pub fn derive_normalize(input: TokenStream) -> TokenStream {
    generate_normalize(input)
}

/// Derive Normalize for Types with no normalizing
/// this always returns the argument when normalizing
#[proc_macro_derive(NoNorm, attributes(Lang))]
pub fn derive_no_normalize(input: TokenStream) -> TokenStream {
    generate_no_normalize(input)
}
