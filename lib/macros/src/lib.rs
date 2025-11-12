extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_meta = derive_input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("Lang"))
        .expect("Type `Lang` needs to be provided")
        .clone()
        .meta;
    let lang_val = match lang_meta {
        Meta::List(ls) => ls.tokens,
        _ => panic!("Wrong format for attribute `Lang`, use `#[Lang(Type)]`"),
    };
    let enum_data = match derive_input.data {
        Data::Struct(_) => panic!("Cannot derive Typecheck for structs"),
        Data::Union(_) => panic!("Cannot derive Typecheck for unions"),
        Data::Enum(en) => en,
    };
    let variants = enum_data
        .variants
        .iter()
        .map(|var| {
            let ident = &var.ident;
            quote! {Self::#ident(inner) => inner.check(env), }
        })
        .collect::<Vec<_>>();
    let output = quote! {
        use check::Typecheck;
        use errors::check_error::CheckError;
        use syntax::env::Environment;
        use derivations::{Derivation};
        use grammar::DerivationRule;
        use std::collections::HashSet;

        impl Typecheck for #ident {
            type Lang = #lang_val;
            fn check(&self,env:Environment<Self::Lang>) -> Result<Derivation<Self::Lang>,CheckError>{
                match self {
                #(#variants),*
                }
            }
            fn rules() -> HashSet<DerivationRule>{
                todo!()
            }
        }
    };
    output.into()
}
