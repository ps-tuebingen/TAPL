use super::{Subst, Term, Var};
use crate::types::{TyVar, Type};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TyLambda {
    pub var: TyVar,
    pub term: Box<Term>,
}

impl TyLambda {
    pub fn new(var: &str, term: Term) -> TyLambda {
        TyLambda {
            var: var.to_owned(),
            term: Box::new(term),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TyApp {
    pub term: Box<Term>,
    pub ty: Type,
}

impl TyApp {
    pub fn new(term: Term, ty: Type) -> TyApp {
        TyApp {
            term: Box::new(term),
            ty,
        }
    }
}

impl Subst for TyLambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyLambda::new(&self.var, self.term.subst(v, t)).into()
    }
    fn subst_ty(self, v: &TyVar, ty: Type) -> Term {
        if *v == self.var {
            self.into()
        } else {
            TyLambda::new(&self.var, self.term.subst_ty(v, ty)).into()
        }
    }
}

impl Subst for TyApp {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyApp::new(self.term.subst(v, t), self.ty).into()
    }
    fn subst_ty(self, v: &TyVar, ty: Type) -> Term {
        TyApp::new(self.term.subst_ty(v, ty.clone()), self.ty.subst(v, ty)).into()
    }
}

impl From<TyLambda> for Term {
    fn from(lam: TyLambda) -> Term {
        Term::TyLambda(lam)
    }
}

impl From<TyApp> for Term {
    fn from(app: TyApp) -> Term {
        Term::TyApp(app)
    }
}

impl fmt::Display for TyLambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.term)
    }
}

impl fmt::Display for TyApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) [{}]", self.term, self.ty)
    }
}
