extern crate proc_macro;
use proc_macro::TokenStream;

mod check;
pub(crate) mod literals;
pub(crate) mod utils;
use check::{
    subtypecheck::{generate_no_subtypecheck, generate_subtypecheck},
    typecheck::generate_typecheck,
};

#[proc_macro_derive(Subtypecheck, attributes(Lang))]
pub fn derive_subcheck(input: TokenStream) -> TokenStream {
    generate_subtypecheck(input)
}

#[proc_macro_derive(NoSubtypes, attributes(Lang))]
pub fn derive_no_subcheck(input: TokenStream) -> TokenStream {
    generate_no_subtypecheck(input)
}

#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    generate_typecheck(input)
}
