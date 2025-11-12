extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod utils;
use utils::{get_enum_variants, get_lang_attr, get_variant_type_name};

#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let variants = get_enum_variants(&derive_input.data);
    let check_variants = variants
        .iter()
        .map(|var| {
            let ident = &var.ident;
            quote! {Self::#ident(inner) => inner.check(env), }
        })
        .collect::<Vec<_>>();
    let rule_variants = variants
        .iter()
        .map(|var| {
            let ty_name = get_variant_type_name(var);
            quote! { rules.extend(#ty_name::<#lang_val>::rules()); }
        })
        .collect::<Vec<_>>();
    let output = quote! {
        #[automatically_derived]
        impl check::Typecheck for #ident {
            type Lang = #lang_val;
            fn check(&self,env:syntax::env::Environment<Self::Lang>) -> Result<derivations::Derivation<Self::Lang>,errors::check_error::CheckError>{
                match self {
                #(#check_variants)*
                }
            }
            fn rules() -> ::std::collections::HashSet<grammar::DerivationRule>{
                let mut rules = ::std::collections::HashSet::new();
                #(#rule_variants)*
                rules
            }
        }
    };
    output.into()
}
