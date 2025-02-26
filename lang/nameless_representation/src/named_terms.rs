use super::Var;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn var(v: &str) -> Term {
        Term::Var(v.to_owned())
    }

    pub fn lam(v: &str, t: Term) -> Term {
        Term::Lambda(v.to_owned(), Box::new(t))
    }

    pub fn app(t1: Term, t2: Term) -> Term {
        Term::App(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Lambda(v, body) => write!(f, "\\{v}.{body}"),
            Term::App(t1, t2) => write!(f, "({t1} {t2})"),
        }
    }
}
