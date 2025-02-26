use crate::types::{TyVar, Type};
use std::fmt;

pub mod lambda;
pub mod tylambda;
pub use lambda::{App, Lambda};
pub use tylambda::{TyApp, TyLambda};

pub type Var = String;

#[derive(Debug, Clone)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    TyLambda(TyLambda),
    TyApp(TyApp),
}

pub trait Subst {
    fn subst(self, v: &Var, t: Term) -> Term;
    fn subst_ty(self, v: &TyVar, ty: Type) -> Term;
}

impl Subst for Term {
    fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(v1) => {
                if *v == v1 {
                    t
                } else {
                    Term::Var(v1)
                }
            }
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::TyLambda(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
        }
    }

    fn subst_ty(self, v: &TyVar, ty: Type) -> Term {
        match self {
            Term::Var(v1) => Term::Var(v1),
            Term::Lambda(lam) => lam.subst_ty(v, ty),
            Term::App(app) => app.subst_ty(v, ty),
            Term::TyLambda(lam) => lam.subst_ty(v, ty),
            Term::TyApp(app) => app.subst_ty(v, ty),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::TyLambda(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
        }
    }
}

impl From<Var> for Term {
    fn from(v: Var) -> Term {
        Term::Var(v)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}
