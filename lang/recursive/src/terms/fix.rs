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
pub struct Fix {
    pub term: Box<Term>,
}

impl Fix {
    pub fn new(t: Term) -> Fix {
        Fix { term: Box::new(t) }
    }
}

impl From<Fix> for Term {
    fn from(fix: Fix) -> Term {
        Term::Fix(fix)
    }
}

impl IsValue for Fix {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Fix {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Fix::new(self.term.subst_ty(v, ty))
    }
}

impl SubstTerm for Fix {
    fn subst(self, v: Var, t: Term) -> Self {
        Fix::new(self.term.subst(v, t))
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix ({})", self.term)
    }
}
