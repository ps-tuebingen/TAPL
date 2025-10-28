use super::{TypedArithmetic, types::Type};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            True::<TypedArithmetic>::rule(),
            False::<TypedArithmetic>::rule(),
            If::<TypedArithmetic>::rule(),
            Num::<TypedArithmetic>::rule(),
            Succ::<TypedArithmetic>::rule(),
            Pred::<TypedArithmetic>::rule(),
            IsZero::<TypedArithmetic>::rule(),
        ])
    }
}

impl SubstType for Term {
    type Lang = TypedArithmetic;
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm for Term {
    type Lang = TypedArithmetic;
    type Target = Term;
    fn subst(self, _: &Var, _: &Term) -> Self::Target {
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

impl From<True<TypedArithmetic>> for Term {
    fn from(tru: True<TypedArithmetic>) -> Term {
        Term::True(tru)
    }
}
impl From<False<TypedArithmetic>> for Term {
    fn from(fls: False<TypedArithmetic>) -> Term {
        Term::False(fls)
    }
}
impl From<If<TypedArithmetic>> for Term {
    fn from(ift: If<TypedArithmetic>) -> Term {
        Term::If(ift)
    }
}
impl From<Num<TypedArithmetic>> for Term {
    fn from(num: Num<TypedArithmetic>) -> Term {
        Term::Num(num)
    }
}
impl From<Succ<TypedArithmetic>> for Term {
    fn from(succ: Succ<TypedArithmetic>) -> Term {
        Term::Succ(succ)
    }
}
impl From<Pred<TypedArithmetic>> for Term {
    fn from(pred: Pred<TypedArithmetic>) -> Term {
        Term::Pred(pred)
    }
}
impl From<IsZero<TypedArithmetic>> for Term {
    fn from(isz: IsZero<TypedArithmetic>) -> Term {
        Term::IsZero(isz)
    }
}
