use crate::utils::{get_enum_variants, get_lang_attr, get_variant_type_name, map_variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_grammar_describe(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let variants = get_enum_variants(&derive_input.data);
    let variant_rules = map_variants(&variants, |var| {
        let ty_name = get_variant_type_name(&var);
        quote! {
            <#ty_name::<#lang_val> as grammar::GrammarRuleDescribe>::rule(),
        }
    });

    let output = quote! {
        #[automatically_derived]
        impl grammar::GrammarDescribe for #ident{
            fn grammar() -> grammar::Grammar{
                grammar::Grammar::term(vec![
                    #(#variant_rules)*
                ])
            }
        }
    };
    output.into()
}
