use super::Term;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Zero;

#[derive(Debug, Clone)]
pub struct Succ(pub Box<Term>);

impl Succ {
    pub fn new<T: Into<Term>>(t: T) -> Succ {
        Succ(Box::new(t.into()))
    }
}

#[derive(Debug, Clone)]
pub struct Pred(pub Box<Term>);

impl Pred {
    pub fn new<T: Into<Term>>(t: T) -> Pred {
        Pred(Box::new(t.into()))
    }
}

#[derive(Debug, Clone)]
pub struct IsZero(pub Box<Term>);

impl IsZero {
    pub fn new<T: Into<Term>>(t: T) -> IsZero {
        IsZero(Box::new(t.into()))
    }
}

impl From<Zero> for Term {
    fn from(zero: Zero) -> Term {
        Term::Zero(zero)
    }
}
impl From<Succ> for Term {
    fn from(succ: Succ) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred> for Term {
    fn from(pred: Pred) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        match i {
            0 => Zero.into(),
            i if i > 0 => Succ::new(i - 1).into(),
            _ => Pred::new(i + 1).into(),
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
        write!(f, "succ({})", self.0)
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.0)
    }
}

impl fmt::Display for IsZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.0)
    }
}
