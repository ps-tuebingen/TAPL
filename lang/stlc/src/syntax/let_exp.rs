use super::Term;
use crate::Var;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl From<Let> for Term {
    fn from(lt: Let) -> Term {
        Term::Let(lt)
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({}={}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
