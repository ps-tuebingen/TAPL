use super::Term;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Tup {
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Proj {
    pub tup: Box<Term>,
    pub ind: usize,
}

impl From<Tup> for Term {
    fn from(tup: Tup) -> Term {
        Term::Tup(tup)
    }
}

impl From<Proj> for Term {
    fn from(proj: Proj) -> Term {
        Term::Proj(proj)
    }
}

impl fmt::Display for Tup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.terms
                .iter()
                .map(|t| format!("{t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Proj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.tup, self.ind)
    }
}
