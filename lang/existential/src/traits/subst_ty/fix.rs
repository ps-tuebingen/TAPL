use super::SubstTy;
use crate::{
    terms::Fix,
    types::{Type, TypeVar},
};

impl SubstTy for Fix {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Fix {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}
