use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ascribe {
    pub term: Box<Term>,
    pub ty: Type,
}

impl From<Ascribe> for Term {
    fn from(asc: Ascribe) -> Term {
        Term::Ascribe(asc)
    }
}

impl fmt::Display for Ascribe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} as {}", self.term, self.ty)
    }
}
