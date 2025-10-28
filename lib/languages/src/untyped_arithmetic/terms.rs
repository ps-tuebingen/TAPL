use super::UntypedArithmetic;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    env::Environment,
    language::Language,
    subst::{SubstTerm, SubstType},
    terms::{False, If, IsZero, Num, Pred, Succ, True},
    untyped::Untyped,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            True::<UntypedArithmetic>::rule(),
            False::<UntypedArithmetic>::rule(),
            If::<UntypedArithmetic>::rule(),
            Num::<UntypedArithmetic>::rule(),
            Succ::<UntypedArithmetic>::rule(),
            Pred::<UntypedArithmetic>::rule(),
            IsZero::<UntypedArithmetic>::rule(),
        ])
    }
}

impl SubstTerm for Term {
    type Lang = UntypedArithmetic;
    type Target = Term;
    fn subst(self, _: &Var, _: &Term) -> Self::Target {
        self
    }
}

impl SubstType for Term {
    type Target = Self;
    type Lang = UntypedArithmetic;
    fn subst_type(self, _: &TypeVar, _: &Untyped<UntypedArithmetic>) -> Self::Target {
        self
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
        }
    }
}

impl Typecheck for Term {
    type Lang = UntypedArithmetic;
    fn check(
        &self,
        _: Environment<UntypedArithmetic>,
    ) -> Result<Derivation<UntypedArithmetic>, CheckError> {
        Err(NoTyping::new(UntypedArithmetic.describe()).into())
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
