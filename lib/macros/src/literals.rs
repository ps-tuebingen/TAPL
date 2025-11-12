use quote::quote;

pub fn lang_ty() -> proc_macro2::TokenStream {
    quote! {<Self::Lang as syntax::language::Language>::Type}
}

pub fn lang_env() -> proc_macro2::TokenStream {
    quote! {syntax::env::Environment<Self::Lang>}
}

pub fn check_result() -> proc_macro2::TokenStream {
    quote! { Result<derivations::Derivation<Self::Lang>,errors::check_error::CheckError> }
}
