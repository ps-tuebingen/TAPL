use super::{Term, Var};
use crate::{
    syntax::{
        kinds::Kind,
        types::{Type, TypeVar},
    },
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TyLambda {
    pub var: TypeVar,
    pub annot: Kind,
    pub body: Box<Term>,
}

impl TyLambda {
    pub fn new<T: Into<Term>>(var: &str, annot: Kind, body: T) -> TyLambda {
        TyLambda {
            var: var.to_owned(),
            annot,
            body: Box::new(body.into()),
        }
    }
}

impl SubstTerm for TyLambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyLambda {
            var: self.var,
            annot: self.annot,
            body: Box::new(self.body.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for TyLambda {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        TyLambda {
            var: self.var,
            annot: self.annot,
            body: Box::new(self.body.subst_ty(v, ty)),
        }
    }
}

impl From<TyLambda> for Term {
    fn from(lam: TyLambda) -> Term {
        Term::TyLambda(lam)
    }
}

impl fmt::Display for TyLambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "λ{}::{}.{}", self.var, self.annot, self.body)
    }
}
