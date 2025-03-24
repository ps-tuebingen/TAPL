use super::Term;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub ift: Box<Term>,
    pub thent: Box<Term>,
    pub elset: Box<Term>,
}

impl From<If> for Term {
    fn from(ift: If) -> Term {
        Term::If(ift)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.ift, self.thent, self.elset
        )
    }
}
