use super::Term;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct True;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct False;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub ifc: Box<Term>,
    pub thenc: Box<Term>,
    pub elsec: Box<Term>,
}

impl From<True> for Term {
    fn from(tru: True) -> Term {
        Term::True(tru)
    }
}
impl From<False> for Term {
    fn from(fls: False) -> Term {
        Term::False(fls)
    }
}

impl From<If> for Term {
    fn from(ift: If) -> Term {
        Term::If(ift)
    }
}

impl fmt::Display for True {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("True")
    }
}

impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("False")
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.ifc, self.thenc, self.elsec
        )
    }
}
