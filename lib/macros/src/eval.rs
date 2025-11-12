use crate::{
    literals::{new_set, rule_set},
    utils::{get_enum_variants, get_lang_attr, get_variant_type_name, map_variants},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_eval(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let rule_set = rule_set();
    let new_set = new_set();
    let variants = get_enum_variants(&derive_input.data);
    let eval_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.eval(env),}
    });
    let rule_variants = map_variants(&variants, |var| {
        let ty_name = get_variant_type_name(var);
        quote! {
            rules.extend(<#ty_name::<#lang_val> as eval::Eval>::rules());
        }
    });

    let output = quote! {
        #[automatically_derived]
        impl eval::Eval for #ident{
            type Lang = #lang_val;
            fn eval(self, env: &mut syntax::eval_context::EvalContext<Self::Lang>) -> Result<trace::EvalTrace<Self::Lang>, errors::eval_error::EvalError> {
                match self{
                    #(#eval_variants)*
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
