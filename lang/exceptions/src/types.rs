use common::{
    errors::ErrorKind,
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Fun, Nat, Unit},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit(Unit),
    Nat(Nat),
    Bool(Bool),
    Fun(Fun<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {
    fn into_nat(self) -> Result<Nat, ErrorKind> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Nat".to_owned(),
            })
        }
    }

    fn into_bool(self) -> Result<Bool, ErrorKind> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Bool".to_owned(),
            })
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, ErrorKind> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }
}

impl From<Unit> for Type {
    fn from(u: Unit) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat> for Type {
    fn from(nat: Nat) -> Type {
        Type::Nat(nat)
    }
}

impl From<Bool> for Type {
    fn from(b: Bool) -> Type {
        Type::Bool(b)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl SubstType<Self> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Unit(u) => u.subst_type(v, ty),
            Type::Nat(n) => n.subst_type(v, ty),
            Type::Bool(b) => b.subst_type(v, ty),
            Type::Fun(f) => f.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
        }
    }
}
