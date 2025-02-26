use super::{Type, TypeVar};
use crate::traits::SubstTy;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpApp {
    pub fun: Box<Type>,
    pub arg: Box<Type>,
}

impl OpApp {
    pub fn new<T: Into<Type>, U: Into<Type>>(fun: T, arg: U) -> OpApp {
        OpApp {
            fun: Box::new(fun.into()),
            arg: Box::new(arg.into()),
        }
    }
}

impl SubstTy for OpApp {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        OpApp {
            fun: Box::new(self.fun.subst_ty(v, ty.clone())),
            arg: Box::new(self.arg.subst_ty(v, ty)),
        }
    }
}

impl From<OpApp> for Type {
    fn from(app: OpApp) -> Type {
        Type::OpApp(app)
    }
}

impl fmt::Display for OpApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
