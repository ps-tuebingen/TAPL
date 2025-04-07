use super::{Type, TypeVar};
use crate::{syntax::kinds::Kind, traits::SubstTy};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpLambda {
    pub var: TypeVar,
    pub annot: Kind,
    pub body: Box<Type>,
}

impl OpLambda {
    pub fn new<T: Into<Type>>(v: &str, annot: Kind, body: T) -> OpLambda {
        OpLambda {
            var: v.to_owned(),
            annot,
            body: Box::new(body.into()),
        }
    }
}

impl SubstTy for OpLambda {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        if *v == self.var {
            self
        } else {
            OpLambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst_ty(v, ty)),
            }
        }
    }
}

impl From<OpLambda> for Type {
    fn from(lam: OpLambda) -> Type {
        Type::OpLambda(lam)
    }
}

impl fmt::Display for OpLambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
