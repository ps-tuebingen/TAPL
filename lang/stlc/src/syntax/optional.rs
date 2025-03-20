use super::Term;
use crate::{types::Type, Var};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nothing {
    pub inner_type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Something {
    pub term: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SomeCase {
    pub bound_term: Box<Term>,
    pub none_rhs: Box<Term>,
    pub some_var: Var,
    pub some_rhs: Box<Term>,
}

impl From<Nothing> for Term {
    fn from(not: Nothing) -> Term {
        Term::Nothing(not)
    }
}

impl From<Something> for Term {
    fn from(some: Something) -> Term {
        Term::Something(some)
    }
}

impl From<SomeCase> for Term {
    fn from(somecase: SomeCase) -> Term {
        Term::SomeCase(somecase)
    }
}

impl fmt::Display for Nothing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.inner_type)
    }
}

impl fmt::Display for Something {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something({})", self.term)
    }
}

impl fmt::Display for SomeCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nothing => {} | Something({}) => {} }}",
            self.bound_term, self.none_rhs, self.some_var, self.some_rhs
        )
    }
}
