use super::{Term, Var};
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct LambdaSub {
    pub var: Var,
    pub sup_ty: Type,
    pub body: Box<Term>,
}

impl LambdaSub {
    pub fn new<T: Into<Term>>(var: &str, sup_ty: Type, body: T) -> LambdaSub {
        LambdaSub {
            var: var.to_owned(),
            sup_ty,
            body: Box::new(body.into()),
        }
    }

    pub fn new_unbounded<T: Into<Term>>(var: &str, body: T) -> LambdaSub {
        LambdaSub {
            var: var.to_owned(),
            sup_ty: Type::Top,
            body: Box::new(body.into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TyApp {
    pub term: Box<Term>,
    pub ty: Type,
}

impl TyApp {
    pub fn new<T: Into<Term>>(term: T, ty: Type) -> TyApp {
        TyApp {
            term: Box::new(term.into()),
            ty,
        }
    }
}

impl From<LambdaSub> for Term {
    fn from(lam: LambdaSub) -> Term {
        Term::LambdaSub(lam)
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

impl fmt::Display for LambdaSub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "λ{}<:{}.{}", self.var, self.sup_ty, self.body)
    }
}
