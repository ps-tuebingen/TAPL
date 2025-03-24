use super::Term;
use std::fmt;

#[derive(Debug)]
pub struct True;

#[derive(Debug)]
pub struct False;

#[derive(Debug)]
pub struct If {
    pub ifc: Box<Term>,
    pub thenc: Box<Term>,
    pub elsec: Box<Term>,
}

impl If {
    pub fn new(ifc: Term, thenc: Term, elsec: Term) -> If {
        If {
            ifc: Box::new(ifc),
            thenc: Box::new(thenc),
            elsec: Box::new(elsec),
        }
    }
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
        f.write_str("true")
    }
}

impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }} ",
            self.ifc, self.thenc, self.elsec
        )
    }
}
