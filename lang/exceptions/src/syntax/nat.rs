use super::Term;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succ {
    pub term: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    pub term: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsZero {
    pub term: Box<Term>,
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

impl fmt::Display for IsZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
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

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
    }
}
