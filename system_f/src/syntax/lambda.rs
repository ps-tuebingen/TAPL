use super::{Subst, Term, Var};
use crate::types::{TyVar, Type};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

impl Lambda {
    pub fn new(var: &str, annot: Type, body: Term) -> Lambda {
        Lambda {
            var: var.to_owned(),
            annot,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub fun: Box<Term>,
    pub arg: Box<Term>,
}

impl App {
    pub fn new(fun: Term, arg: Term) -> App {
        App {
            fun: Box::new(fun),
            arg: Box::new(arg),
        }
    }
}

impl Subst for Lambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        if *v == self.var {
            self.into()
        } else {
            Lambda::new(&self.var, self.annot, self.body.subst(v, t)).into()
        }
    }

    fn subst_ty(self, v: &TyVar, ty: Type) -> Term {
        Lambda::new(
            &self.var,
            self.annot.subst(v, ty.clone()),
            self.body.subst_ty(v, ty),
        )
        .into()
    }
}

impl Subst for App {
    fn subst(self, v: &Var, t: Term) -> Term {
        App::new(self.fun.subst(v, t.clone()), self.arg.subst(v, t)).into()
    }

    fn subst_ty(self, v: &TyVar, ty: Type) -> Term {
        App::new(self.fun.subst_ty(v, ty.clone()), self.arg.subst_ty(v, ty)).into()
    }
}

impl From<Lambda> for Term {
    fn from(lam: Lambda) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App> for Term {
    fn from(app: App) -> Term {
        Term::App(app)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "λ{}:{}.{}", self.var, self.annot, self.body)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
