use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Pred {
    pub term: Box<Term>,
}

impl SubstTerm for Pred {
    fn subst(self, v: &Var, t: Term) -> Term {
        Pred {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for Pred {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Pred {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}
impl From<Pred> for Term {
    fn from(pred: Pred) -> Term {
        Term::Pred(pred)
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}
