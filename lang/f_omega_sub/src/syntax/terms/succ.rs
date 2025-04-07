use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Succ {
    pub term: Box<Term>,
}

impl Succ {
    pub fn new<T: Into<Term>>(term: T) -> Succ {
        Succ {
            term: Box::new(term.into()),
        }
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

impl SubstTy for Succ {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Succ {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl From<Succ> for Term {
    fn from(succ: Succ) -> Term {
        Term::Succ(succ)
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}
