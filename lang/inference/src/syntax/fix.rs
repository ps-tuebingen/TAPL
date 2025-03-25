use super::Term;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub term: Box<Term>,
}

impl From<Fix> for Term {
    fn from(fix: Fix) -> Term {
        Term::Fix(fix)
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
