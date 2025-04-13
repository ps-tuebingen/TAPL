use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use common::Label;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RecordProj {
    pub term: Box<Term>,
    pub label: Label,
}

impl RecordProj {
    pub fn new<T: Into<Term>>(term: T, label: &str) -> RecordProj {
        RecordProj {
            term: Box::new(term.into()),
            label: label.to_owned(),
        }
    }
}

impl SubstTerm for RecordProj {
    fn subst(self, v: &Var, t: Term) -> Term {
        RecordProj {
            term: Box::new(self.term.subst(v, t)),
            label: self.label,
        }
        .into()
    }
}

impl SubstTy for RecordProj {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        RecordProj {
            term: Box::new(self.term.subst_ty(v, ty)),
            label: self.label,
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
        write!(f, "({}).{}", self.term, self.label)
    }
}
