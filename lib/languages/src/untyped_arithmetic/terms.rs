use super::UntypedArithmetic;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::DerivationRule;
use macros::{Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};

#[derive(
    FromVariants,
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[Lang(UntypedArithmetic)]
pub enum Term {
    True(True<UntypedArithmetic>),
    False(False<UntypedArithmetic>),
    If(If<UntypedArithmetic>),
    Num(Num<UntypedArithmetic>),
    Succ(Succ<UntypedArithmetic>),
    Pred(Pred<UntypedArithmetic>),
    IsZero(IsZero<UntypedArithmetic>),
}

impl syntax::terms::Term for Term {}

impl Typecheck for Term {
    type Lang = UntypedArithmetic;
    fn check(
        &self,
        _: Environment<UntypedArithmetic>,
    ) -> Result<Derivation<UntypedArithmetic>, CheckError> {
        Err(NoTyping::new(UntypedArithmetic.describe()).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
