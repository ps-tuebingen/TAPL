use crate::utils::{get_enum_variants, map_variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_free_vars(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let variants = get_enum_variants(&derive_input.data);
    let var_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.free_vars(vars), }
    });
    let output = quote! {
        #[automatically_derived]
        impl syntax::free_vars::FreeVars for #ident {
            fn free_vars(&self,vars:&mut std::collections::HashSet<syntax::Var>) {
                match self {
                #(#var_variants)*
                }
            }
        }
    };
    output.into()
}

pub fn generate_free_type_vars(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let variants = get_enum_variants(&derive_input.data);
    let var_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.free_type_vars(vars), }
    });
    let output = quote! {
        #[automatically_derived]
        impl syntax::free_vars::FreeTypeVars for #ident {
            fn free_type_vars(&self,vars:&mut std::collections::HashSet<syntax::TypeVar>) {
                match self {
                #(#var_variants)*
                }
            }
        }
    };
    output.into()
}
