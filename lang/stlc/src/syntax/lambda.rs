use super::Term;
use crate::types::Type;
use common::Var;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub fun: Box<Term>,
    pub arg: Box<Term>,
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
        write!(f, "\\{}:{}.({})", self.var, self.annot, self.body)
    }
}
impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
