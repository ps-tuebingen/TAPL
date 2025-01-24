use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Unpack {
    pub ty_var: TypeVar,
    pub bound_var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl Unpack {
    pub fn new<T: Into<Term>, U: Into<Term>>(
        tyv: &str,
        var: &str,
        bound_term: T,
        in_term: U,
    ) -> Unpack {
        Unpack {
            ty_var: tyv.to_owned(),
            bound_var: var.to_owned(),
            bound_term: Box::new(bound_term.into()),
            in_term: Box::new(in_term.into()),
        }
    }
}

impl SubstTerm for Unpack {
    fn subst(self, v: &Var, t: Term) -> Term {
        let bound_subst = self.bound_term.subst(v, t.clone());
        if *v == self.bound_var {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl SubstTy for Unpack {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        if self.ty_var == *v {
            self
        } else {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(self.bound_term.subst_ty(v, ty.clone())),
                in_term: Box::new(self.in_term.subst_ty(v, ty)),
            }
        }
    }
}

impl From<Unpack> for Term {
    fn from(unpack: Unpack) -> Term {
        Term::Unpack(unpack)
    }
}

impl fmt::Display for Unpack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_var, self.bound_var, self.bound_term, self.in_term
        )
    }
}
