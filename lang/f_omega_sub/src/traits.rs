use crate::syntax::{
    terms::{Term, Var},
    types::{Type, TypeVar},
};

pub trait SubstTerm {
    fn subst(self, v: &Var, t: Term) -> Term;
}

pub trait SubstTy {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self;
}
