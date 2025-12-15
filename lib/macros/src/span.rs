use crate::utils::{get_enum_variants, map_variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_spanned(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let variants = get_enum_variants(&derive_input.data);
    let span_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.span(),}
    });

    let output = quote! {
        #[automatically_derived]
        impl syntax::span::Spanned for #ident{
            fn span(&self) -> syntax::span::Span {
                match self{
                    #(#span_variants)*
                }
            }
        }
    };
    output.into()
}
