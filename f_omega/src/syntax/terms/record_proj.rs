use super::{Term, Var};
use crate::{
    syntax::{
        types::{Type, TypeVar},
        Label,
    },
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct RecordProj {
    pub label: Label,
    pub term: Box<Term>,
}

impl SubstTerm for RecordProj {
    fn subst(self, v: &Var, t: Term) -> Term {
        RecordProj {
            label: self.label,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for RecordProj {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        RecordProj {
            label: self.label,
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl From<RecordProj> for Term {
    fn from(proj: RecordProj) -> Term {
        Term::RecordProj(proj)
    }
}

impl fmt::Display for RecordProj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.term, self.label)
    }
}
