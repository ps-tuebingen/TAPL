use super::SubstTy;
use crate::{
    syntax::{App, Lambda},
    types::{Type, TypeVar},
};

impl SubstTy for Lambda {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Lambda {
            var: self.var,
            annot: self.annot.subst_ty(v, ty.clone()),
            body: Box::new(self.body.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for App {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        App {
            fun: Box::new(self.fun.subst_ty(v, ty.clone())),
            arg: Box::new(self.arg.subst_ty(v, ty)),
        }
    }
}
