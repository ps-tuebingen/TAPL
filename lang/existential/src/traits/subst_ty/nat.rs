use super::SubstTy;
use crate::{
    terms::{IsZero, Pred, Succ, Zero},
    types::{Type, TypeVar},
};

impl SubstTy for Zero {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Zero {
        Zero
    }
}

impl SubstTy for Succ {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Succ {
        Succ(Box::new(self.0.subst_ty(v, ty)))
    }
}

impl SubstTy for Pred {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Pred {
        Pred(Box::new(self.0.subst_ty(v, ty)))
    }
}

impl SubstTy for IsZero {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> IsZero {
        IsZero(Box::new(self.0.subst_ty(v, ty)))
    }
}
