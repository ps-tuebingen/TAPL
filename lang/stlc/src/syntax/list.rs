use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nil {
    pub inner_type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cons {
    pub inner_type: Type,
    pub fst: Box<Term>,
    pub rst: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsNil {
    pub inner_type: Type,
    pub list: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    pub inner_type: Type,
    pub list: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub inner_type: Type,
    pub list: Box<Term>,
}

impl From<Nil> for Term {
    fn from(nil: Nil) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons> for Term {
    fn from(cons: Cons) -> Term {
        Term::Cons(cons)
    }
}

impl From<IsNil> for Term {
    fn from(isnil: IsNil) -> Term {
        Term::IsNil(isnil)
    }
}

impl From<Head> for Term {
    fn from(hd: Head) -> Term {
        Term::Head(hd)
    }
}

impl From<Tail> for Term {
    fn from(tl: Tail) -> Term {
        Term::Tail(tl)
    }
}

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.inner_type)
    }
}
impl fmt::Display for Cons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.inner_type, self.fst, self.rst)
    }
}
impl fmt::Display for IsNil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isnil[{}]({})", self.inner_type, self.list)
    }
}
impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head[{}]({})", self.inner_type, self.list)
    }
}
impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tail[{}]({})", self.inner_type, self.list)
    }
}
