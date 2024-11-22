use super::Term;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Pair {
    pub fst: Box<Term>,
    pub snd: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Proj1 {
    pub pair: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Proj2 {
    pub pair: Box<Term>,
}

impl From<Pair> for Term {
    fn from(p: Pair) -> Term {
        Term::Pair(p)
    }
}

impl From<Proj1> for Term {
    fn from(proj: Proj1) -> Term {
        Term::Proj1(proj)
    }
}

impl From<Proj2> for Term {
    fn from(proj: Proj2) -> Term {
        Term::Proj2(proj)
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
impl fmt::Display for Proj1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.1", self.pair)
    }
}

impl fmt::Display for Proj2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.2", self.pair)
    }
}
