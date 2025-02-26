use super::SubstTy;
use crate::{
    syntax::{LambdaSub, TyApp},
    types::{Type, TypeVar},
};

impl SubstTy for LambdaSub {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        LambdaSub {
            var: self.var,
            sup_ty: self.sup_ty.subst_ty(v, ty.clone()),
            body: Box::new(self.body.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for TyApp {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        TyApp {
            term: Box::new(self.term.subst_ty(v, ty.clone())),
            ty: self.ty.subst_ty(v, ty),
        }
    }
}
