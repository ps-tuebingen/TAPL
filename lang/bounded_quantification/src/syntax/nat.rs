use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Const {
    pub i: i64,
}

#[derive(Clone, Debug)]
pub struct Succ {
    pub term: Box<Term>,
}

#[derive(Clone, Debug)]
pub struct Pred {
    pub term: Box<Term>,
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.i)
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}

impl From<Const> for Term {
    fn from(c: Const) -> Term {
        Term::Const(c)
    }
}

impl From<Succ> for Term {
    fn from(s: Succ) -> Term {
        Term::Succ(s)
    }
}

impl From<Pred> for Term {
    fn from(p: Pred) -> Term {
        Term::Pred(p)
    }
}
