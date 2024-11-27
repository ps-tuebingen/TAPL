use std::fmt;

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn subst(self, var: &Var, t: Term) -> Term {
        match self {
            Term::Var(v) => {
                if v == *var {
                    t
                } else {
                    Term::Var(v)
                }
            }
            Term::Lambda(v, b) => {
                if v == *var {
                    Term::Lambda(v, b)
                } else {
                    Term::Lambda(v, Box::new(b.subst(var, t)))
                }
            }
            Term::App(t1, t2) => Term::App(
                Box::new(t1.subst(var, t.clone())),
                Box::new(t2.subst(var, t)),
            ),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Lambda(v, body) => write!(f, "\\{v}.{body}"),
            Term::App(t1, t2) => write!(f, "({t1} {t2})"),
        }
    }
}
