use check::{Typecheck, errors::CheckError};
use derivation::TypingDerivation;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    env::Environment,
    subst::{SubstTerm, SubstType},
    terms::{False, If, IsZero, Num, Pred, Succ, True},
    untyped::Untyped,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            True::<Term>::rule(),
            False::<Term>::rule(),
            If::<Term>::rule(),
            Num::<Term>::rule(),
            Succ::<Term>::rule(),
            Pred::<Term>::rule(),
            IsZero::<Term>::rule(),
        ])
    }
}

impl SubstTerm<Term> for Term {
    type Target = Term;
    fn subst(self, _: &Var, _: &Term) -> Self::Target {
        self
    }
}

impl SubstType<Untyped> for Term {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Untyped) -> Self::Target {
        self
    }
}

impl Typecheck for Term {
    type Term = Term;
    type Type = Untyped;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, _: Environment<Untyped>) -> Result<Self::Deriv, CheckError> {
        Ok(TypingDerivation::empty(self.clone()))
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

impl From<True<Term>> for Term {
    fn from(tru: True<Term>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Term>> for Term {
    fn from(fls: False<Term>) -> Term {
        Term::False(fls)
    }
}

impl From<If<Term>> for Term {
    fn from(ift: If<Term>) -> Term {
        Term::If(ift)
    }
}

impl From<Num<Term>> for Term {
    fn from(num: Num<Term>) -> Term {
        Term::Num(num)
    }
}

impl From<Succ<Term>> for Term {
    fn from(succ: Succ<Term>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<Term>> for Term {
    fn from(pred: Pred<Term>) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero<Term>> for Term {
    fn from(isz: IsZero<Term>) -> Term {
        Term::IsZero(isz)
    }
}
