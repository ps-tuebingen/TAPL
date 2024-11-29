use super::Term;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succ {
    pub term: Box<Term>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pred {
    pub term: Box<Term>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsZero {
    pub term: Box<Term>,
}

impl From<Zero> for Term {
    fn from(z: Zero) -> Term {
        Term::Zero(z)
    }
}

impl From<Pred> for Term {
    fn from(p: Pred) -> Term {
        Term::Pred(p)
    }
}

impl From<Succ> for Term {
    fn from(s: Succ) -> Term {
        Term::Succ(s)
    }
}

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
    }
}

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("0")
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

impl fmt::Display for IsZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}
