use super::{Type, TypeVar};
use crate::traits::SubstTy;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub from: Box<Type>,
    pub to: Box<Type>,
}

impl Fun {
    pub fn new<T: Into<Type>, U: Into<Type>>(from: T, to: U) -> Fun {
        Fun {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }
}

impl SubstTy for Fun {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Fun {
            from: Box::new(self.from.subst_ty(v, ty.clone())),
            to: Box::new(self.to.subst_ty(v, ty)),
        }
    }
}

impl From<Fun> for Type {
    fn from(fun: Fun) -> Type {
        Type::Fun(fun)
    }
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) → ({})", self.from, self.to)
    }
}
