use super::Term;
use std::fmt;

#[derive(Debug)]
pub struct Fix {
    pub term: Box<Term>,
}

impl Fix {
    pub fn new(t: Term) -> Fix {
        Fix { term: Box::new(t) }
    }
}

impl From<Fix> for Term {
    fn from(fix: Fix) -> Term {
        Term::Fix(fix)
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix ({})", self.term)
    }
}
