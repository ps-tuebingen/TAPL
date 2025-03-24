use super::Term;
use std::fmt;

#[derive(Debug)]
pub struct Zero;

#[derive(Debug)]
pub struct Succ {
    pub term: Box<Term>,
}

impl Succ {
    pub fn new(t: Term) -> Succ {
        Succ { term: Box::new(t) }
    }
}

#[derive(Debug)]
pub struct Pred {
    pub term: Box<Term>,
}

impl Pred {
    pub fn new(t: Term) -> Pred {
        Pred { term: Box::new(t) }
    }
}

impl From<Zero> for Term {
    fn from(z: Zero) -> Term {
        Term::Zero(z)
    }
}

impl From<Succ> for Term {
    fn from(sc: Succ) -> Term {
        Term::Succ(sc)
    }
}

impl From<Pred> for Term {
    fn from(prd: Pred) -> Term {
        Term::Pred(prd)
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        match i {
            0 => Zero.into(),
            i if i < 0 => Pred::new((i + 1).into()).into(),
            _ => Succ::new((i - 1).into()).into(),
        }
    }
}

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Zero")
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Succ({})", self.term)
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pred({})", self.term)
    }
}
