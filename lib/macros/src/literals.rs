use quote::quote;

/// `<Self::Lang as Language>::Type`
pub fn lang_ty() -> proc_macro2::TokenStream {
    quote! {<Self::Lang as syntax::language::Language>::Type}
}

//// `Environment<Self::Lang>`
pub fn lang_env() -> proc_macro2::TokenStream {
    quote! {syntax::env::Environment<Self::Lang>}
}

/// `Result<Derivation<Self::Lang>,CheckError>`
pub fn check_result() -> proc_macro2::TokenStream {
    quote! { Result<derivations::Derivation<Self::Lang>,errors::check_error::CheckError> }
}

/// `HashSet<DerivationRule>`
pub fn rule_set() -> proc_macro2::TokenStream {
    quote! { ::std::collections::HashSet<grammar::DerivationRule> }
}

/// let mut rules = HashSet::new();
pub fn new_set() -> proc_macro2::TokenStream {
    quote! {let mut rules = ::std::collections::HashSet::new();}
}

/// <Self::Lang as Language>::Term
pub fn lang_term() -> proc_macro2::TokenStream {
    quote! { <Self::Lang as syntax::language::Language>::Term }
}
