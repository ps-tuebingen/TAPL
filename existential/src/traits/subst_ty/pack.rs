use super::SubstTy;
use crate::{
    terms::{Pack, Unpack},
    types::{Type, TypeVar},
};

impl SubstTy for Pack {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Pack {
            inner_ty: self.inner_ty.subst_ty(v, ty.clone()),
            term: Box::new(self.term.subst_ty(v, ty.clone())),
            outer_ty: self.outer_ty.subst_ty(v, ty),
        }
    }
}

impl SubstTy for Unpack {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        if *v == self.ty_var {
            self
        } else {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(self.bound_term.subst_ty(v, ty.clone())),
                in_term: Box::new(self.in_term.subst_ty(v, ty)),
            }
        }
    }
}
