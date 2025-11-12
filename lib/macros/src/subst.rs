use crate::{
    literals::lang_term,
    utils::{get_enum_variants, get_lang_attr, map_variants},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_subst_term(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let lang_term = lang_term();
    let variants = get_enum_variants(&derive_input.data);
    let variant_subst = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.subst(v,t).into(),}
    });

    let output = quote! {
        impl syntax::subst::SubstTerm for #ident{
            type Lang = #lang_val;
            type Target = Self;

            fn subst(self,v:&Var,t:&#lang_term) -> Self::Target{
                match self{
                    #(#variant_subst)*
                }
            }
        }
    };
    output.into()
}
