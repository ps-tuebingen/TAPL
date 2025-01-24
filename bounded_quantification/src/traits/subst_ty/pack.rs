use super::SubstTy;
use crate::{
    syntax::{Pack, Unpack},
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
        let bound_subst = Box::new(self.bound_term.subst_ty(v, ty.clone()));
        if *v == self.ty_var {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: bound_subst,
                in_term: self.in_term,
            }
        } else {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: bound_subst,
                in_term: Box::new(self.in_term.subst_ty(v, ty)),
            }
        }
    }
}
