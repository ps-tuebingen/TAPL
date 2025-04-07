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
    pub sup_ty: Type,
    pub body: Box<Term>,
}

impl TyLambda {
    pub fn new<T: Into<Type>, U: Into<Term>>(v: &str, sup_ty: T, body: U) -> TyLambda {
        TyLambda {
            var: v.to_owned(),
            sup_ty: sup_ty.into(),
            body: Box::new(body.into()),
        }
    }

    pub fn new_unbounded<U: Into<Term>>(v: &str, kind: Kind, body: U) -> TyLambda {
        TyLambda {
            var: v.to_owned(),
            sup_ty: Type::Top(kind),
            body: Box::new(body.into()),
        }
    }
}

impl SubstTerm for TyLambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyLambda {
            var: self.var,
            sup_ty: self.sup_ty,
            body: Box::new(self.body.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for TyLambda {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let sup_subst = self.sup_ty.subst_ty(v, ty.clone());
        if *v == self.var {
            TyLambda {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
            }
        } else {
            TyLambda {
                var: self.var,
                sup_ty: sup_subst,
                body: Box::new(self.body.subst_ty(v, ty)),
            }
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
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.body)
    }
}
