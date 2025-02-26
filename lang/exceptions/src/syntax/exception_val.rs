use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raise {
    pub exception: Box<Term>,
    pub ex_ty: Type,
    pub cont_ty: Type,
}

impl Raise {
    pub fn new(exception: Term, exception_type: Type, continuation_type: Type) -> Raise {
        Raise {
            exception: Box::new(exception),
            ex_ty: exception_type,
            cont_ty: continuation_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryWithVal {
    pub term: Box<Term>,
    pub handler: Box<Term>,
}

impl TryWithVal {
    pub fn new(term: Term, handler: Term) -> TryWithVal {
        TryWithVal {
            term: Box::new(term),
            handler: Box::new(handler),
        }
    }
}

impl From<Raise> for Term {
    fn from(r: Raise) -> Term {
        Term::Raise(r)
    }
}

impl From<TryWithVal> for Term {
    fn from(t: TryWithVal) -> Term {
        Term::TryWithVal(t)
    }
}

impl fmt::Display for Raise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "raise ({}:{})", self.exception, self.ex_ty)
    }
}

impl fmt::Display for TryWithVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try ({}) with ({})", self.term, self.handler)
    }
}
