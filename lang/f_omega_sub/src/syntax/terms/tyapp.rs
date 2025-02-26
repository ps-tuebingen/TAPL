use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TyApp {
    pub term: Box<Term>,
    pub ty: Type,
}

impl SubstTerm for TyApp {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyApp {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl SubstTy for TyApp {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        TyApp {
            term: Box::new(self.term.subst_ty(v, ty.clone())),
            ty: self.ty.subst_ty(v, ty),
        }
    }
}

impl From<TyApp> for Term {
    fn from(app: TyApp) -> Term {
        Term::TyApp(app)
    }
}

impl fmt::Display for TyApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) [{}]", self.term, self.ty)
    }
}
