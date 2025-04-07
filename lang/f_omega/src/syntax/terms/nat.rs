use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Zero;

#[derive(Debug, Clone)]
pub struct Succ {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Pred {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct IsZero {
    pub term: Box<Term>,
}

impl SubstTerm for Zero {
    fn subst(self, _: &Var, _: Term) -> Term {
        Zero.into()
    }
}

impl SubstTerm for Succ {
    fn subst(self, v: &Var, t: Term) -> Term {
        Succ {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTerm for Pred {
    fn subst(self, v: &Var, t: Term) -> Term {
        Pred {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTerm for IsZero {
    fn subst(self, v: &Var, t: Term) -> Term {
        IsZero {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for Zero {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        self
    }
}

impl SubstTy for Succ {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Succ {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for Pred {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Pred {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for IsZero {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        IsZero {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl From<Zero> for Term {
    fn from(z: Zero) -> Term {
        Term::Zero(z)
    }
}

impl From<Succ> for Term {
    fn from(s: Succ) -> Term {
        Term::Succ(s)
    }
}

impl From<Pred> for Term {
    fn from(p: Pred) -> Term {
        Term::Pred(p)
    }
}

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
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
