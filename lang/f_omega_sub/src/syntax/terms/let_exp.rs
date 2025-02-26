use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Let {
    pub var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl Let {
    pub fn new<T: Into<Term>, U: Into<Term>>(v: &str, bound_term: T, in_term: U) -> Let {
        Let {
            var: v.to_owned(),
            bound_term: Box::new(bound_term.into()),
            in_term: Box::new(in_term.into()),
        }
    }
}

impl SubstTerm for Let {
    fn subst(self, v: &Var, t: Term) -> Term {
        let bound_subst = self.bound_term.subst(v, t.clone());
        if *v == self.var {
            Let {
                var: self.var,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Let {
                var: self.var,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl SubstTy for Let {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Let {
            var: self.var,
            bound_term: Box::new(self.bound_term.subst_ty(v, ty.clone())),
            in_term: Box::new(self.in_term.subst_ty(v, ty)),
        }
    }
}

impl From<Let> for Term {
    fn from(lt: Let) -> Term {
        Term::Let(lt)
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Let {}={} in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
