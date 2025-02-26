use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TyApp {
    pub fun: Box<Term>,
    pub arg: Type,
}

impl TyApp {
    pub fn new<T: Into<Term>, U: Into<Type>>(fun: T, arg: U) -> TyApp {
        TyApp {
            fun: Box::new(fun.into()),
            arg: arg.into(),
        }
    }
}

impl SubstTerm for TyApp {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyApp {
            fun: Box::new(self.fun.subst(v, t)),
            arg: self.arg,
        }
        .into()
    }
}

impl SubstTy for TyApp {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        TyApp {
            fun: Box::new(self.fun.subst_ty(v, ty.clone())),
            arg: self.arg.subst_ty(v, ty),
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
        write!(f, "({}) [{}]", self.fun, self.arg)
    }
}
