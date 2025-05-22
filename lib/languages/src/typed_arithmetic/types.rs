use common::{
    errors::ErrorKind,
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Nat},
    TypeVar,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Nat(Nat<Type>),
    Bool(Bool<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {
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

    fn into_bool(self) -> Result<Bool<Type>, ErrorKind> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Bool".to_owned(),
            })
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Type;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
        }
    }
}

impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}
