use crate::syntax::{
    terms::{Term, Var},
    types::{Type, TypeVar},
};

pub trait SubstTerm {
    fn subst(self, var: &Var, t: Term) -> Term;
}

pub trait SubstTy {
    fn subst_ty(self, var: &TypeVar, ty: Type) -> Self;
}
