use super::{Term, Var};
use crate::types::Type;
use std::fmt;

#[derive(Debug)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

impl Lambda {
    pub fn new(v: &str, annot: Type, t: Term) -> Lambda {
        Lambda {
            var: v.to_owned(),
            annot,
            body: Box::new(t),
        }
    }
}

#[derive(Debug)]
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
