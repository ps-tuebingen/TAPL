use super::{Term, Var};
use crate::{traits::free_vars::FreeVars, types::Type};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    pub var: Var,
    pub annot: Type,
    pub body: Box<Term>,
}

impl Lambda {
    pub fn new(v: &str, annot: Type, body: Term) -> Lambda {
        Lambda {
            var: v.to_owned(),
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
    pub fn new(t1: Term, t2: Term) -> App {
        App {
            fun: Box::new(t1),
            arg: Box::new(t2),
        }
    }
    pub fn seq(self, t2: Term) -> App {
        let var = self.fresh_var();
        App {
            fun: Box::new(
                Lambda {
                    var,
                    annot: Type::Unit,
                    body: Box::new(t2),
                }
                .into(),
            ),
            arg: Box::new(self.into()),
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
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
