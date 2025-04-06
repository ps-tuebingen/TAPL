use super::Term;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Fix {
    pub term: Box<Term>,
}

impl From<Fix> for Term {
    fn from(f: Fix) -> Term {
        Term::Fix(f)
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
