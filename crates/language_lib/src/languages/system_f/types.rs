use common::errors::ErrorKind;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{Forall, Fun, TypeGroup, TypeVariable},
    TypeVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
}

impl syntax::types::Type for Type {}

impl TypeGroup for Type {
    fn into_fun(self) -> Result<Fun<Self>, ErrorKind> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }

    fn into_forall(self) -> Result<Forall<Self>, ErrorKind> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Universal Type".to_owned(),
            })
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;

    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
        }
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(v: TypeVariable<Type>) -> Type {
        Type::Var(v)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Forall<Type>> for Type {
    fn from(forall: Forall<Type>) -> Type {
        Type::Forall(forall)
    }
}
