use super::UntypedArithmetic;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::DerivationRule;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};

#[derive(
    SubstType, SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Clone, Debug, PartialEq, Eq,
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

impl From<True<UntypedArithmetic>> for Term {
    fn from(tru: True<UntypedArithmetic>) -> Term {
        Term::True(tru)
    }
}

impl From<False<UntypedArithmetic>> for Term {
    fn from(fls: False<UntypedArithmetic>) -> Term {
        Term::False(fls)
    }
}

impl From<If<UntypedArithmetic>> for Term {
    fn from(ift: If<UntypedArithmetic>) -> Term {
        Term::If(ift)
    }
}

impl From<Num<UntypedArithmetic>> for Term {
    fn from(num: Num<UntypedArithmetic>) -> Term {
        Term::Num(num)
    }
}

impl From<Succ<UntypedArithmetic>> for Term {
    fn from(succ: Succ<UntypedArithmetic>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<UntypedArithmetic>> for Term {
    fn from(pred: Pred<UntypedArithmetic>) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero<UntypedArithmetic>> for Term {
    fn from(isz: IsZero<UntypedArithmetic>) -> Term {
        Term::IsZero(isz)
    }
}
