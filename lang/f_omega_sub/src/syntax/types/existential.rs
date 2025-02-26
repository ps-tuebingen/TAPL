use super::{Type, TypeVar};
use crate::{syntax::kinds::Kind, traits::SubstTy};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Existential {
    pub var: TypeVar,
    pub sup_ty: Box<Type>,
    pub ty: Box<Type>,
}

impl Existential {
    pub fn new<T: Into<Type>, U: Into<Type>>(var: &str, sup_ty: T, ty: U) -> Existential {
        Existential {
            var: var.to_owned(),
            sup_ty: Box::new(sup_ty.into()),
            ty: Box::new(ty.into()),
        }
    }

    pub fn new_unbounded<T: Into<Type>>(var: &str, kind: Kind, ty: T) -> Existential {
        Existential {
            var: var.to_owned(),
            sup_ty: Box::new(Type::Top(kind)),
            ty: Box::new(ty.into()),
        }
    }
}

impl SubstTy for Existential {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let sup_subst = self.sup_ty.subst_ty(v, ty.clone());
        if *v == self.var {
            Existential {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: self.ty,
            }
        } else {
            Existential {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: Box::new(self.ty.subst_ty(v, ty)),
            }
        }
    }
}

impl From<Existential> for Type {
    fn from(ex: Existential) -> Type {
        Type::Existential(ex)
    }
}

impl fmt::Display for Existential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "∃{}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}
