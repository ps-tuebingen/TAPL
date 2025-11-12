use crate::utils::{get_enum_variants, map_variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_term_display(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let variants = get_enum_variants(&derive_input.data);
    let variants_display = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {
            Self::#ident(inner) => inner.fmt(f),
        }
    });

    let output = quote! {
        impl ::std::fmt::Display for #ident{
            fn fmt(&self,f:&mut ::std::fmt::Formatter) -> ::std::fmt::Result{
                match self{
                    #(#variants_display)*
                }
            }
        }
    };
    output.into()
}
