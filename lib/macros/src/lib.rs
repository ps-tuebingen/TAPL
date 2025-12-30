extern crate proc_macro;
use proc_macro::TokenStream;

mod check;
mod eq;
mod eval;
mod format;
mod free_vars;
mod from;
mod grammar;
mod inference;
pub(crate) mod literals;
mod span;
mod subst;
pub(crate) mod utils;

use check::{
    kindcheck::{generate_kindcheck, generate_no_kindcheck},
    normalize::{generate_no_normalize, generate_normalize},
    subtypecheck::{generate_no_subtypecheck, generate_subtypecheck},
    typecheck::generate_typecheck,
};
use eq::generate_eq_no_span;
use eval::generate_eval;
use format::{generate_display, generate_latexfmt};
use free_vars::{generate_free_type_vars, generate_free_vars};
use from::{generate_from_variants, generate_into_term};
use grammar::generate_grammar_describe;
use inference::{
    generate_generate_constraints_term, generate_generate_constraints_type,
    generate_solve_constraint,
};
use span::generate_spanned;
use subst::{generate_subst_term, generate_subst_type};

/// Derive `Typecheck` for Terms
/// terms need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement `Typecheck`
#[proc_macro_derive(Typecheck, attributes(Lang))]
pub fn derive_check(input: TokenStream) -> TokenStream {
    generate_typecheck(input)
}

/// Derive `Subtypecheck` for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement `Subtypecheck`
#[proc_macro_derive(Subtypecheck, attributes(Lang))]
pub fn derive_subcheck(input: TokenStream) -> TokenStream {
    generate_subtypecheck(input)
}

/// Derive `Subtypecheck` for Types with no subtyping
/// this always throws an error when the subtypes are checked
#[proc_macro_derive(NoSubtypes, attributes(Lang))]
pub fn derive_no_subcheck(input: TokenStream) -> TokenStream {
    generate_no_subtypecheck(input)
}

/// Derive `Kindcheck` for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement `Kindhcheck`
#[proc_macro_derive(Kindcheck, attributes(Lang))]
pub fn derive_kindcheck(input: TokenStream) -> TokenStream {
    generate_kindcheck(input)
}

/// Derive `Kindcheck` for Types with no Kinds
/// this always throws an error when the kinds are checked
#[proc_macro_derive(NoKinds, attributes(Lang))]
pub fn derive_no_kindcheck(input: TokenStream) -> TokenStream {
    generate_no_kindcheck(input)
}

/// Derive `Normalize` for Types
/// types need to have the form `enum T { Ty1(Ty1),Ty2(Ty2),...}`
/// where all `Tyi` implement `Normalize`
#[proc_macro_derive(Normalize, attributes(Lang))]
pub fn derive_normalize(input: TokenStream) -> TokenStream {
    generate_normalize(input)
}

/// Derive `Normalize` for Types with no normalizing
/// this always returns the argument when normalizing
#[proc_macro_derive(NoNorm, attributes(Lang))]
pub fn derive_no_normalize(input: TokenStream) -> TokenStream {
    generate_no_normalize(input)
}

/// Derive `Eval` for Terms
/// Terms need to have the form `enum Term { T1(T1),T2(T2),... }`
/// where each `Ti` has `Eval` implemneted
#[proc_macro_derive(Eval, attributes(Lang))]
pub fn derive_eval(input: TokenStream) -> TokenStream {
    generate_eval(input)
}

/// Derive `GrammarDescribe` for Terms
/// Terms need to have the form `enum Term { T1(T1),T2(T2),... }`
/// where each `Ti` has `RuleDescribe` implemneted
#[proc_macro_derive(GrammarDescribe, attributes(Lang))]
pub fn derive_grammar_describe(input: TokenStream) -> TokenStream {
    generate_grammar_describe(input)
}

/// Derive [`fmt::Display`] for enums
/// enums must have the form `enum T { T1(T1),T2(T2),...}`
/// implementation calls `fmt` on all variants
#[proc_macro_derive(LangDisplay)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    generate_display(input)
}

/// Derive `LatexFmt` for enums
/// enums must have the form `enum T { T1(T1),T2(T2),...}`
/// implementation calls `to_latex` on all variants
#[proc_macro_derive(LatexFmt)]
pub fn derive_latexfmt(input: TokenStream) -> TokenStream {
    generate_latexfmt(input)
}

/// Derive `SubstTerm` for terms
/// terms need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement `SubstTerm`
#[proc_macro_derive(SubstTerm, attributes(Lang))]
pub fn derive_substterm(input: TokenStream) -> TokenStream {
    generate_subst_term(input)
}

/// Derive `SubstType`for enums
/// enums need to have the form `enum T { T1(T1),T2(T2),...}`
/// where all `Ti` implement `SubstTerm`
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

/// Derive `From<Value> for Term` for values
/// values must have the form `enum T { T1(T1),T2(T2),....}`
/// and each `Ti` must implement `Into<Term>`
#[proc_macro_derive(IntoTerm, attributes(Lang))]
pub fn derive_into_term(input: TokenStream) -> TokenStream {
    generate_into_term(input)
}

/// Derive [`syntax::span::Spanned`] for enum types
/// Requires that each enum variant has implemented `Spanned`
#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: TokenStream) -> TokenStream {
    generate_spanned(input)
}

/// Derive [`std::cmp::PartialEq`] and [`std::cmp::Eq`] for structs ignoring [`syntax::span::Span`] fields
/// requires all struct fields to be named and have both implemented
#[proc_macro_derive(EqNoSpan)]
pub fn derive_eq_no_span(input: TokenStream) -> TokenStream {
    generate_eq_no_span(input)
}

/// Derive [`syntax::free_vars::FreeVars`] for enum types
/// requires all variants to implement FreeVars and calls that implementation
#[proc_macro_derive(FreeVars)]
pub fn derive_free_vars(input: TokenStream) -> TokenStream {
    generate_free_vars(input)
}

/// Derive [`syntax::free_vars::FreeTypeVars`] for enum types
/// requires all variants to implement FreeTypeVars and calls that implementation
#[proc_macro_derive(FreeTypeVars)]
pub fn derive_free_type_vars(input: TokenStream) -> TokenStream {
    generate_free_type_vars(input)
}

/// Derive [`inference::GenerateConstraints`] for enum types of terms
/// requires all variants to implement GenerateConstraints and calls that implementation
#[proc_macro_derive(GenerateConstraintsTerm, attributes(Lang))]
pub fn derive_generate_constraints_term(input: TokenStream) -> TokenStream {
    generate_generate_constraints_term(input)
}

/// Derive [`inference::GenerateConstraints`] for enum types of types
/// requires all variants to implement GenerateConstraints and calls that implementation
#[proc_macro_derive(GenerateConstraintsType, attributes(Lang))]
pub fn derive_generate_constraints_type(input: TokenStream) -> TokenStream {
    generate_generate_constraints_type(input)
}

#[proc_macro_derive(SolveConstraint, attributes(Lang))]
pub fn derive_solve_constraint(input: TokenStream) -> TokenStream {
    generate_solve_constraint(input)
}
