use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Try {
    pub term: Box<Term>,
    pub handler: Box<Term>,
}

impl Try {
    pub fn new(term: Term, handler: Term) -> Try {
        Try {
            term: Box::new(term),
            handler: Box::new(handler),
        }
    }
}

impl From<Try> for Term {
    fn from(t: Try) -> Term {
        Term::Try(t)
    }
}

impl From<Error> for Term {
    fn from(err: Error) -> Term {
        Term::Error(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}

impl fmt::Display for Try {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try ({}) with ({})", self.term, self.handler)
    }
}
