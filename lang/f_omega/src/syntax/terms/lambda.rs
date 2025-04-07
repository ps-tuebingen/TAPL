use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

impl Lambda {
    pub fn new<T: Into<Type>, U: Into<Term>>(var: &str, annot: T, body: U) -> Lambda {
        Lambda {
            var: var.to_owned(),
            annot: annot.into(),
            body: Box::new(body.into()),
        }
    }
}

impl SubstTerm for Lambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        if *v == self.var {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl SubstTy for Lambda {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Lambda {
            var: self.var,
            annot: self.annot.subst_ty(v, ty.clone()),
            body: Box::new(self.body.subst_ty(v, ty)),
        }
    }
}

impl From<Lambda> for Term {
    fn from(lam: Lambda) -> Term {
        Term::Lambda(lam)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}
