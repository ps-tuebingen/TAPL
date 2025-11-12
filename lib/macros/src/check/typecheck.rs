use crate::{
    literals::{check_result, lang_env},
    utils::{get_enum_variants, get_lang_attr, get_variant_type_name, map_variants},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_typecheck(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let variants = get_enum_variants(&derive_input.data);
    let check_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.check(env), }
    });
    let rule_variants = map_variants(&variants, |var| {
        let ty_name = get_variant_type_name(var);
        quote! { rules.extend(#ty_name::<#lang_val>::rules()); }
    });
    let lang_env = lang_env();
    let check_result = check_result();
    let output = quote! {
        #[automatically_derived]
        impl check::Typecheck for #ident {
            type Lang = #lang_val;
            fn check(&self,env:#lang_env) -> #check_result {
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
