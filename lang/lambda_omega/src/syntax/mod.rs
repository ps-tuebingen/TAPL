use crate::{kinds::Kind, types::Type};
use std::fmt;

pub type Var = String;

#[derive(Debug, Clone)]
pub enum Term {
    Var(Var),
    Unit,
    Lambda {
        var: Var,
        annot: Type,
        body: Box<Term>,
    },
    TyLambda {
        var: Var,
        kind: Kind,
        body: Box<Term>,
    },
    App {
        fun: Box<Term>,
        arg: Box<Term>,
    },
    TyApp {
        fun: Box<Term>,
        arg: Type,
    },
}

impl Term {
    pub fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(var) => {
                if var == *v {
                    t
                } else {
                    Term::Var(var)
                }
            }
            Term::Lambda { var, annot, body } => {
                if var == *v {
                    Term::Lambda { var, annot, body }
                } else {
                    Term::Lambda {
                        var,
                        annot,
                        body: Box::new(body.subst(v, t)),
                    }
                }
            }
            Term::App { fun, arg } => Term::App {
                fun: Box::new(fun.subst(v, t.clone())),
                arg: Box::new(arg.subst(v, t)),
            },
            Term::Unit => Term::Unit,
            Term::TyLambda { var, kind, body } => Term::TyLambda {
                var,
                kind,
                body: Box::new(body.subst(v, t)),
            },
            Term::TyApp { fun, arg } => Term::TyApp {
                fun: Box::new(fun.subst(v, t)),
                arg,
            },
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Lambda { var, annot, body } => {
                write!(f, "λ{}:{}.{}", var, annot, body)
            }
            Term::App { fun, arg } => write!(f, "({fun}) ({arg})"),
            Term::Unit => f.write_str("unit"),
            Term::TyLambda { var, kind, body } => write!(f, "\\{var}::{kind}.{body}"),
            Term::TyApp { fun, arg } => write!(f, "{fun}[{arg}]"),
        }
    }
}
