use common::{
    errors::ErrorKind,
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Fun, Nat, Reference, Unit},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit(Unit<Type>),
    Nat(Nat<Type>),
    Bool(Bool<Type>),
    Fun(Fun<Type>),
    Ref(Reference<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {
    fn into_unit(self) -> Result<Unit<Type>, ErrorKind> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Nat".to_owned(),
            })
        }
    }
    fn into_nat(self) -> Result<Nat<Type>, ErrorKind> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Nat".to_owned(),
            })
        }
    }

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

    fn into_bool(self) -> Result<Bool<Self>, ErrorKind> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Bool".to_owned(),
            })
        }
    }

    fn into_ref(self) -> Result<Reference<Self>, ErrorKind> {
        if let Type::Ref(reft) = self {
            Ok(reft)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Reference".to_owned(),
            })
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Ref(rf) => rf.fmt(f),
        }
    }
}

impl From<Unit<Type>> for Type {
    fn from(u: Unit<Type>) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Reference<Type>> for Type {
    fn from(rf: Reference<Type>) -> Type {
        Type::Ref(rf)
    }
}

impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}
