use super::{Term, Var};
use std::fmt;

#[derive(Debug)]
pub struct Let {
    pub var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl Let {
    pub fn new(var: &str, bound: Term, int: Term) -> Let {
        Let {
            var: var.to_owned(),
            bound_term: Box::new(bound),
            in_term: Box::new(int),
        }
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}

impl From<Let> for Term {
    fn from(lt: Let) -> Term {
        Term::Let(lt)
    }
}
