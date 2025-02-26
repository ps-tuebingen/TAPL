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
pub struct Fold {
    pub ty: Type,
    pub term: Box<Term>,
}

impl Fold {
    pub fn new(t: Term, ty: Type) -> Fold {
        Fold {
            ty,
            term: Box::new(t),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unfold {
    pub ty: Type,
    pub term: Box<Term>,
}

impl Unfold {
    pub fn new(t: Term, ty: Type) -> Unfold {
        Unfold {
            ty,
            term: Box::new(t),
        }
    }
}

impl IsValue for Fold {
    fn is_value(&self) -> bool {
        self.term.is_value()
    }
}

impl IsValue for Unfold {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Fold {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Fold {
            ty: self.ty.subst_ty(v.clone(), ty.clone()),
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for Unfold {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Unfold {
            ty: self.ty.subst_ty(v.clone(), ty.clone()),
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTerm for Fold {
    fn subst(self, v: Var, t: Term) -> Self {
        Fold {
            ty: self.ty,
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl SubstTerm for Unfold {
    fn subst(self, v: Var, t: Term) -> Self {
        Unfold {
            ty: self.ty,
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl From<Fold> for Term {
    fn from(fold: Fold) -> Term {
        Term::Fold(fold)
    }
}

impl From<Unfold> for Term {
    fn from(unfold: Unfold) -> Term {
        Term::Unfold(unfold)
    }
}

impl fmt::Display for Fold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold [{}] {}", self.ty, self.term)
    }
}

impl fmt::Display for Unfold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold [{}] {}", self.ty, self.term)
    }
}
