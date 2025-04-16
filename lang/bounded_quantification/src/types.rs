use common::{
    errors::ErrorKind,
    language::LanguageType,
    subst::SubstType,
    types::{Exists, ForallBounded, Fun, Nat, Record, Top, TypeVariable},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable),
    Top(Top),
    Nat(Nat),
    Fun(Fun<Type>),
    Forall(ForallBounded<Type>),
    Exists(Exists<Type>),
    Record(Record<Type>),
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

impl From<TypeVariable> for Type {
    fn from(v: TypeVariable) -> Type {
        Type::Var(v)
    }
}
impl From<Top> for Type {
    fn from(t: Top) -> Type {
        Type::Top(t)
    }
}

impl From<Nat> for Type {
    fn from(nat: Nat) -> Type {
        Type::Nat(nat)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<ForallBounded<Type>> for Type {
    fn from(forall: ForallBounded<Type>) -> Type {
        Type::Forall(forall)
    }
}

impl From<Exists<Type>> for Type {
    fn from(exists: Exists<Type>) -> Type {
        Type::Exists(exists)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl SubstType<Self> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Var(v) => v.subst_type(v, ty),
            Type::Top(t) => t.subst_type(v, ty),
            Type::Nat(n) => n.subst_type(v, ty),
            Type::Fun(f) => f.subst_type(v, ty),
            Type::Forall(f) => f.subst_type(v, ty),
            Type::Exists(e) => e.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => v.fmt(f),
            Type::Top(t) => t.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Fun(f) => f.fmt(f),
            Type::Forall(f) => f.fmt(f),
            Type::Exists(e) => e.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}
