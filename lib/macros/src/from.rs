use crate::utils::{get_enum_variants, get_lang_attr, get_variant_type_name};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

pub fn generate_from_variants(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let lang_name = get_lang_attr(&derive_input.attrs);
    let ident = derive_input.ident;
    let variants = get_enum_variants(&derive_input.data);
    let mut streams = Vec::with_capacity(variants.len());
    for variant in variants {
        let ty_name = get_variant_type_name(&variant);
        streams.push(generate_from(&ty_name, &lang_name, &ident, &variant.ident));
    }
    quote! { #(#streams)* }.into()
}

fn generate_from(
    ty_name: &Ident,
    lang_name: &proc_macro2::TokenStream,
    ident: &Ident,
    variant_ident: &Ident,
) -> proc_macro2::TokenStream {
    let ty_applied = quote! {#ty_name<#lang_name>};
    quote! {
        #[automatically_derived]
        impl From<#ty_applied> for #ident{
            fn from(t:#ty_applied) -> #ident{
                #ident::#variant_ident(t)
            }
        }
    }
    .into()
}
