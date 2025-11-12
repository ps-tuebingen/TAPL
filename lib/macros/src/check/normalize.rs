use crate::{
    literals::{lang_env, new_set, rule_set},
    utils::{get_enum_variants, get_lang_attr, get_variant_type_name, map_variants},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_normalize(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let lang_env = lang_env();
    let rule_set = rule_set();
    let new_set = new_set();
    let variants = get_enum_variants(&derive_input.data);
    let norm_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.normalize(env),}
    });
    let rule_variants = map_variants(&variants, |var| {
        let ty_name = get_variant_type_name(&var);
        quote! {
            rules.extend(<#ty_name::<#lang_val> as check::Normalize>::rules());
        }
    });

    let output = quote! {
        impl check::Normalize for #ident{
            type Lang = #lang_val;
            fn normalize(self,env:#lang_env) -> derivations::Derivation<Self::Lang>{
                match self{
                    #(#norm_variants)*
                }
            }

            fn rules() -> #rule_set{
                #new_set
                #(#rule_variants)*
                rules
            }

        }
    };
    output.into()
}

pub fn generate_no_normalize(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let lang_env = lang_env();

    let output = quote! {
        impl check::Normalize for #ident{
            type Lang = #lang_val;

            fn normalize(self,env:#lang_env) -> derivations::Derivation<Self::Lang>{
                derivations::NormalizingDerivation::empty(self).into()
            }

            fn rules() {
                std::collections::HashSet::new()
            }
        }
    };
    output.into()
}
