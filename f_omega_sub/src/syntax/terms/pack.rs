use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Pack {
    pub inner_ty: Type,
    pub term: Box<Term>,
    pub outer_ty: Type,
}

impl Pack {
    pub fn new<T: Into<Type>, U: Into<Term>, V: Into<Type>>(inner: T, term: U, outer: V) -> Pack {
        Pack {
            inner_ty: inner.into(),
            term: Box::new(term.into()),
            outer_ty: outer.into(),
        }
    }
}

impl SubstTerm for Pack {
    fn subst(self, v: &Var, t: Term) -> Term {
        Pack {
            inner_ty: self.inner_ty,
            term: Box::new(self.term.subst(v, t)),
            outer_ty: self.outer_ty,
        }
        .into()
    }
}

impl SubstTy for Pack {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Pack {
            inner_ty: self.inner_ty.subst_ty(v, ty.clone()),
            term: Box::new(self.term.subst_ty(v, ty.clone())),
            outer_ty: self.outer_ty,
        }
    }
}

impl From<Pack> for Term {
    fn from(pack: Pack) -> Term {
        Term::Pack(pack)
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
