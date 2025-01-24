use super::Term;
use std::fmt;

#[derive(Debug, Clone)]
pub struct True;

#[derive(Debug, Clone)]
pub struct False;

#[derive(Debug, Clone)]
pub struct If {
    pub ifc: Box<Term>,
    pub thenc: Box<Term>,
    pub elsec: Box<Term>,
}

impl From<True> for Term {
    fn from(t: True) -> Term {
        Term::True(t)
    }
}

impl From<False> for Term {
    fn from(f: False) -> Term {
        Term::False(f)
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
        write!(f, "If {} then {} else {}", self.ifc, self.thenc, self.elsec)
    }
}
