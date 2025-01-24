use super::SubstTy;
use crate::{
    terms::{False, If, True},
    types::{Type, TypeVar},
};

impl SubstTy for True {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        True
    }
}
impl SubstTy for False {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        False
    }
}

impl SubstTy for If {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> If {
        If {
            ifc: Box::new(self.ifc.subst_ty(v, ty.clone())),
            thenc: Box::new(self.thenc.subst_ty(v, ty.clone())),
            elsec: Box::new(self.elsec.subst_ty(v, ty)),
        }
    }
}
