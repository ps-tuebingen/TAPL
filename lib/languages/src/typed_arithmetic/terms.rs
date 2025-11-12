use super::TypedArithmetic;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{False, If, IsZero, Num, Pred, Succ, True};

#[derive(
    FromVariants,
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Typecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(TypedArithmetic)]
pub enum Term {
    True(True<TypedArithmetic>),
    False(False<TypedArithmetic>),
    If(If<TypedArithmetic>),
    Num(Num<TypedArithmetic>),
    Succ(Succ<TypedArithmetic>),
    Pred(Pred<TypedArithmetic>),
    IsZero(IsZero<TypedArithmetic>),
}

impl syntax::terms::Term for Term {}
