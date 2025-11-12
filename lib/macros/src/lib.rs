extern crate proc_macro;
use proc_macro::TokenStream;

mod check;
mod eval;
mod format;
mod from;
mod grammar;
pub(crate) mod literals;
mod subst;
pub(crate) mod utils;
use check::{
    kindcheck::{generate_kindcheck, generate_no_kindcheck},
    normalize::{generate_no_normalize, generate_normalize},
    subtypecheck::{generate_no_subtypecheck, generate_subtypecheck},
    typecheck::generate_typecheck,
};
use eval::generate_eval;
use format::{generate_display, generate_latexfmt};
use from::generate_from_variants;
use grammar::generate_grammar_describe;
use subst::{generate_subst_term, generate_subst_type};

/// Derive Typecheck for Terms
/// terms need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement Typecheck
#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    generate_typecheck(input)
}

//// Derive Subtypecheck for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Subtypecheck
#[proc_macro_derive(Subtypecheck, attributes(Lang))]
pub fn derive_subcheck(input: TokenStream) -> TokenStream {
    generate_subtypecheck(input)
}

/// Derive Subtypecheck for Types with no subtyping
/// this always throws an error when the subtypes are checked
#[proc_macro_derive(NoSubtypes, attributes(Lang))]
pub fn derive_no_subcheck(input: TokenStream) -> TokenStream {
    generate_no_subtypecheck(input)
}

/// Derive Kindchecking for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Kindhcheck
#[proc_macro_derive(Kindcheck, attributes(Lang))]
pub fn derive_kindcheck(input: TokenStream) -> TokenStream {
    generate_kindcheck(input)
}

/// Derive Kindcheck for Types with no Kinds
/// this always throws an error when the kinds are checked
#[proc_macro_derive(NoKinds, attributes(Lang))]
pub fn derive_no_kindcheck(input: TokenStream) -> TokenStream {
    generate_no_kindcheck(input)
}

/// Derive Normalize for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement Normalize
#[proc_macro_derive(Normalize, attributes(Lang))]
pub fn derive_normalize(input: TokenStream) -> TokenStream {
    generate_normalize(input)
}

/// Derive Normalize for Types with no normalizing
/// this always returns the argument when normalizing
#[proc_macro_derive(NoNorm, attributes(Lang))]
pub fn derive_no_normalize(input: TokenStream) -> TokenStream {
    generate_no_normalize(input)
}

/// Derive Eval for Terms
/// Terms need to have the form `enum Term { T1(T1),T2(T2),... }`
/// where each `Ti` has `Eval` implemneted
#[proc_macro_derive(Eval, attributes(Lang))]
pub fn derive_eval(input: TokenStream) -> TokenStream {
    generate_eval(input)
}

/// Derive Grammar Describe for Terms
//// Terms need to have the form `enum Term { T1(T1),T2(T2),... }`
/// where each `Ti` has `RuleDescribe` implemneted
#[proc_macro_derive(GrammarDescribe, attributes(Lang))]
pub fn derive_grammar_describe(input: TokenStream) -> TokenStream {
    generate_grammar_describe(input)
}

/// Derive fmt::Display for enums
/// enums must have the form `enum T { T1(T1),T2(T2),...}`
/// implementation calls `fmt` on all variants
#[proc_macro_derive(LangDisplay)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    generate_display(input)
}

/// Derive LatexFmt for enums
/// enums must have the form `enum T { T1(T1),T2(T2),...}`
/// implementation calls to_latex on all variants
#[proc_macro_derive(LatexFmt)]
pub fn derive_latexfmt(input: TokenStream) -> TokenStream {
    generate_latexfmt(input)
}

/// Derive SubstTerm for terms
/// terms need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement SubstTerm
#[proc_macro_derive(SubstTerm, attributes(Lang))]
pub fn derive_substterm(input: TokenStream) -> TokenStream {
    generate_subst_term(input)
}

/// Derive SubstType for enums
/// enums need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement SubstTerm
#[proc_macro_derive(SubstType, attributes(Lang))]
pub fn derive_substtype(input: TokenStream) -> TokenStream {
    generate_subst_type(input)
}

/// Derive From<> for enums
/// enums must have the form `enum T { T1(T1),T2(T2),....}`
/// each implementation is then `T::Ti(ti)`
#[proc_macro_derive(FromVariants, attributes(Lang))]
pub fn derive_from_variants(input: TokenStream) -> TokenStream {
    generate_from_variants(input)
}
