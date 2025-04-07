use super::{Type, TypeVar};
use crate::traits::SubstTy;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universal {
    pub var: TypeVar,
    pub sup_ty: Box<Type>,
    pub ty: Box<Type>,
}

impl Universal {
    pub fn new<T: Into<Type>, U: Into<Type>>(v: &str, sup_ty: T, ty: U) -> Universal {
        Universal {
            var: v.to_owned(),
            sup_ty: Box::new(sup_ty.into()),
            ty: Box::new(ty.into()),
        }
    }
}

impl SubstTy for Universal {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let sup_subst = self.sup_ty.subst_ty(v, ty.clone());
        if self.var == *v {
            Universal {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: self.ty,
            }
        } else {
            Universal {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: Box::new(self.ty.subst_ty(v, ty)),
            }
        }
    }
}

impl From<Universal> for Type {
    fn from(uni: Universal) -> Type {
        Type::Universal(uni)
    }
}

impl fmt::Display for Universal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}
