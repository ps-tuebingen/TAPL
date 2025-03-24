use super::{Loc, Term};
use std::fmt;

#[derive(Debug)]
pub struct Ref {
    pub term: Box<Term>,
}

impl Ref {
    pub fn new(t: Term) -> Ref {
        Ref { term: Box::new(t) }
    }
}

#[derive(Debug)]
pub struct Deref {
    pub term: Box<Term>,
}

impl Deref {
    pub fn new(t: Term) -> Deref {
        Deref { term: Box::new(t) }
    }
}

#[derive(Debug)]
pub struct Assign {
    pub to: Box<Term>,
    pub content: Box<Term>,
}

impl Assign {
    pub fn new(to: Term, content: Term) -> Assign {
        Assign {
            to: Box::new(to),
            content: Box::new(content),
        }
    }
}

#[derive(Debug)]
pub struct Location {
    pub loc: Loc,
}

impl From<Loc> for Location {
    fn from(loc: Loc) -> Location {
        Location { loc }
    }
}

impl From<Ref> for Term {
    fn from(rf: Ref) -> Term {
        Term::Ref(rf)
    }
}

impl From<Deref> for Term {
    fn from(deref: Deref) -> Term {
        Term::Deref(deref)
    }
}

impl From<Assign> for Term {
    fn from(assign: Assign) -> Term {
        Term::Assign(assign)
    }
}

impl From<Location> for Term {
    fn from(loc: Location) -> Term {
        Term::Loc(loc)
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref ({})", self.term)
    }
}

impl fmt::Display for Deref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!({})", self.term)
    }
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) := ({})", self.to, self.content)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
