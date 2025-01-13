use crate::{
    terms::{Term, Var},
    types::{Type, TypeVar},
};

pub trait SubstTerm: Sized {
    fn subst(self, v: Var, t: Term) -> Self;
}

pub trait SubstTy: Sized {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self;
}
