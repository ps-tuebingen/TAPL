use super::{Type, TypeVar};
use crate::{syntax::kinds::Kind, traits::SubstTy};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universal {
    pub var: TypeVar,
    pub kind: Kind,
    pub ty: Box<Type>,
}

impl Universal {
    pub fn new<T: Into<Type>>(var: &str, kind: Kind, ty: T) -> Universal {
        Universal {
            var: var.to_owned(),
            kind,
            ty: Box::new(ty.into()),
        }
    }
}

impl SubstTy for Universal {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        if *v == self.var {
            self
        } else {
            Universal {
                var: self.var,
                kind: self.kind,
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
        write!(f, "forall {}::{}.({})", self.var, self.kind, self.ty)
    }
}
