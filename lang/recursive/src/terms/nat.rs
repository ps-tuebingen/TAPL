use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zero;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succ {
    pub term: Box<Term>,
}

impl Succ {
    pub fn new(t: Term) -> Succ {
        Succ { term: Box::new(t) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    pub term: Box<Term>,
}

impl Pred {
    pub fn new(t: Term) -> Pred {
        Pred { term: Box::new(t) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsZero {
    pub term: Box<Term>,
}

impl IsZero {
    pub fn new(t: Term) -> IsZero {
        IsZero { term: Box::new(t) }
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        match i {
            0 => Zero.into(),
            i if i > 0 => Succ {
                term: Box::new((i - 1).into()),
            }
            .into(),
            _ => Pred {
                term: Box::new((i + 1).into()),
            }
            .into(),
        }
    }
}

impl From<Zero> for Term {
    fn from(zero: Zero) -> Term {
        Term::Zero(zero)
    }
}

impl From<Succ> for Term {
    fn from(succ: Succ) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred> for Term {
    fn from(pred: Pred) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
    }
}

impl IsValue for Zero {
    fn is_value(&self) -> bool {
        true
    }
}

impl IsValue for Succ {
    fn is_value(&self) -> bool {
        self.term.is_value() && !matches!(*self.term, Term::Pred(_))
    }
}

impl IsValue for Pred {
    fn is_value(&self) -> bool {
        self.term.is_value() && !matches!(*self.term, Term::Succ(_))
    }
}

impl IsValue for IsZero {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Zero {
    fn subst_ty(self, _: TypeVar, _: Type) -> Self {
        Zero
    }
}

impl SubstTy for Succ {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Succ {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for Pred {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Pred {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for IsZero {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        IsZero::new(self.term.subst_ty(v, ty))
    }
}

impl SubstTerm for Zero {
    fn subst(self, _: Var, _: Term) -> Self {
        Zero
    }
}

impl SubstTerm for Succ {
    fn subst(self, v: Var, t: Term) -> Self {
        Succ {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl SubstTerm for Pred {
    fn subst(self, v: Var, t: Term) -> Self {
        Pred {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl SubstTerm for IsZero {
    fn subst(self, v: Var, t: Term) -> Self {
        IsZero::new(self.term.subst(v, t))
    }
}

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("zero")
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}

impl fmt::Display for IsZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}
