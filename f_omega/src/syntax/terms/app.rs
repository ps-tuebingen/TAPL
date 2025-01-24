use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct App {
    pub fun: Box<Term>,
    pub arg: Box<Term>,
}

impl App {
    pub fn new<T: Into<Term>, U: Into<Term>>(fun: T, arg: U) -> App {
        App {
            fun: Box::new(fun.into()),
            arg: Box::new(arg.into()),
        }
    }
}

impl SubstTerm for App {
    fn subst(self, v: &Var, t: Term) -> Term {
        App {
            fun: Box::new(self.fun.subst(v, t.clone())),
            arg: Box::new(self.arg.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for App {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        App {
            fun: Box::new(self.fun.subst_ty(v, ty.clone())),
            arg: Box::new(self.arg.subst_ty(v, ty)),
        }
    }
}

impl From<App> for Term {
    fn from(app: App) -> Term {
        Term::App(app)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
