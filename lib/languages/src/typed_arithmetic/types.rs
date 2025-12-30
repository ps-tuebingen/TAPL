use super::TypedArithmetic;
use macros::{
    FreeTypeVars, FromVariants, GenerateConstraintsType, GrammarDescribe, LangDisplay, LatexFmt,
    NoKinds, NoNorm, NoSubtypes, SolveConstraint, Spanned, SubstType,
};
use syntax::types::{Bool, Nat, Type as TypeTrait, TypeGroup};

#[derive(
    SolveConstraint,
    GenerateConstraintsType,
    FreeTypeVars,
    Spanned,
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    Debug,
    PartialEq,
    Eq,
    Clone,
)]
#[Lang(TypedArithmetic)]
pub enum Type {
    Nat(Nat<TypedArithmetic>),
    Bool(Bool<TypedArithmetic>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = TypedArithmetic;
    fn into_nat(self) -> Option<Nat<TypedArithmetic>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<TypedArithmetic>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }
}
