use super::{Type, TypeVar};
use crate::{syntax::kinds::Kind, traits::SubstTy};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Existential {
    pub ty_var: TypeVar,
    pub kind: Kind,
    pub ty: Box<Type>,
}

impl Existential {
    pub fn new<T: Into<Type>>(var: &str, kind: Kind, ty: T) -> Existential {
        Existential {
            ty_var: var.to_owned(),
            kind,
            ty: Box::new(ty.into()),
        }
    }
}

impl SubstTy for Existential {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        if *v == self.ty_var {
            self
        } else {
            Existential {
                ty_var: self.ty_var,
                kind: self.kind,
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
        write!(f, "∃{}::{}.{}", self.ty_var, self.kind, self.ty)
    }
}
