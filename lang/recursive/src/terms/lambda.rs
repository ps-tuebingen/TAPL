use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl IsValue for Lambda {
    fn is_value(&self) -> bool {
        true
    }
}

impl IsValue for App {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Lambda {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Lambda {
            var: self.var,
            annot: self.annot.subst_ty(v.clone(), ty.clone()),
            body: Box::new(self.body.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for App {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        App {
            fun: Box::new(self.fun.subst_ty(v.clone(), ty.clone())),
            arg: Box::new(self.arg.subst_ty(v.clone(), ty.clone())),
        }
    }
}

impl SubstTerm for Lambda {
    fn subst(self, v: Var, t: Term) -> Self {
        if self.var == v {
            self
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
        }
    }
}

impl SubstTerm for App {
    fn subst(self, v: Var, t: Term) -> Self {
        App {
            fun: Box::new(self.fun.subst(v.clone(), t.clone())),
            arg: Box::new(self.arg.subst(v, t)),
        }
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

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({}) ({}))", self.fun, self.arg)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:({}).({})", self.var, self.annot, self.body)
    }
}
