use super::Var;
use std::fmt;
use untyped_lambda::terms::Term as UnTerm;

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

impl From<UnTerm> for Term {
    fn from(t: UnTerm) -> Term {
        match t {
            UnTerm::Var(v) => Term::Var(v),
            UnTerm::Lambda(v, t) => Term::Lambda(v, Box::new((*t).into())),
            UnTerm::App(t1, t2) => Term::App(Box::new((*t1).into()), Box::new((*t2).into())),
        }
    }
}
