use crate::{
    kinds::Kind,
    types::{Type, TypeVar},
};
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

    pub fn subst_ty(self, v: &TypeVar, ty: Type) -> Term {
        match self {
            Term::Var(v) => Term::Var(v),
            Term::Unit => Term::Unit,
            Term::Lambda { var, annot, body } => {
                let annot_subst = annot.subst(v, ty.clone());
                let body_subst = body.subst_ty(v, ty);
                Term::Lambda {
                    var,
                    annot: annot_subst,
                    body: Box::new(body_subst),
                }
            }
            Term::App { fun, arg } => {
                let fun_subst = fun.subst_ty(v, ty.clone());
                let arg_subst = arg.subst_ty(v, ty);
                Term::App {
                    fun: Box::new(fun_subst),
                    arg: Box::new(arg_subst),
                }
            }
            Term::TyLambda { var, kind, body } => {
                if *v == var {
                    Term::TyLambda { var, kind, body }
                } else {
                    let body_subst = body.subst_ty(v, ty);
                    Term::TyLambda {
                        var,
                        kind,
                        body: Box::new(body_subst),
                    }
                }
            }
            Term::TyApp { fun, arg } => {
                let fun_subst = fun.subst_ty(v, ty.clone());
                let arg_subst = arg.subst(v, ty);
                Term::TyApp {
                    fun: Box::new(fun_subst),
                    arg: arg_subst,
                }
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Lambda { var, annot, body } => {
                write!(f, "\\{}:{}.{}", var, annot, body)
            }
            Term::App { fun, arg } => write!(f, "({fun}) ({arg})"),
            Term::Unit => f.write_str("unit"),
            Term::TyLambda { var, kind, body } => write!(f, "\\{var}::{kind}.{body}"),
            Term::TyApp { fun, arg } => write!(f, "({fun})[{arg}]"),
        }
    }
}
