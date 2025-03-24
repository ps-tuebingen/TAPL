use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Debug)]
pub struct Cast {
    pub term: Box<Term>,
    pub ty: Type,
}

impl Cast {
    pub fn new(t: Term, ty: Type) -> Cast {
        Cast {
            term: Box::new(t),
            ty,
        }
    }
}

impl From<Cast> for Term {
    fn from(cast: Cast) -> Term {
        Term::Cast(cast)
    }
}

impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) as ({})", self.term, self.ty)
    }
}
