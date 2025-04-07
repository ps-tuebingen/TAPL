use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Fix {
    pub term: Box<Term>,
}

impl SubstTerm for Fix {
    fn subst(self, v: &Var, t: Term) -> Term {
        Fix {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for Fix {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Fix {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl From<Fix> for Term {
    fn from(f: Fix) -> Term {
        Term::Fix(f)
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
