use super::{Term, Var};
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

impl Lambda {
    pub fn new<T: Into<Term>>(var: &str, annot: Type, body: T) -> Lambda {
        Lambda {
            var: var.to_owned(),
            annot,
            body: Box::new(body.into()),
        }
    }
}

#[derive(Clone, Debug)]
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
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "λ{}:{}.{}", self.var, self.annot, self.body)
    }
}
