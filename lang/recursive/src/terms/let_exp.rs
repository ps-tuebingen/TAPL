use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl Let {
    pub fn new(v: &str, bound: Term, int: Term) -> Let {
        Let {
            var: v.to_owned(),
            bound_term: Box::new(bound),
            in_term: Box::new(int),
        }
    }
}

impl IsValue for Let {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Let {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Let::new(
            &self.var,
            self.bound_term.subst_ty(v.clone(), ty.clone()),
            self.in_term.subst_ty(v, ty),
        )
    }
}

impl SubstTerm for Let {
    fn subst(self, v: Var, t: Term) -> Self {
        let bound_subst = self.bound_term.subst(v.clone(), t.clone());
        if v == self.var {
            Let::new(&self.var, bound_subst, *self.in_term)
        } else {
            Let::new(&self.var, bound_subst, self.in_term.subst(v, t))
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
            "let ({}={}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
