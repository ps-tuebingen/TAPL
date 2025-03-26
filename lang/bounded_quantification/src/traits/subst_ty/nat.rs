use super::SubstTy;
use crate::{
    syntax::{Const, Pred, Succ},
    types::{Type, TypeVar},
};

impl SubstTy for Const {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        self
    }
}

impl SubstTy for Succ {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let inner_subst = self.term.subst_ty(v, ty);
        Succ {
            term: Box::new(inner_subst),
        }
    }
}

impl SubstTy for Pred {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let inner_subst = self.term.subst_ty(v, ty);
        Pred {
            term: Box::new(inner_subst),
        }
    }
}
