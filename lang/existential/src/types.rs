use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Exists, Fun, Nat, Record, TypeVariable, Unit},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable),
    Unit(Unit),
    Nat(Nat),
    Bool(Bool),
    Fun(Fun<Type>),
    Exists(Exists<Type>),
    Record(Record<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(v) => f.subst_type(v, ty),
            Type::Unit(u) => u.subst_type(v, ty),
            Type::Nat(nat) => nat.subst_type(v, ty),
            Type::Bool(b) => b.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Exists(exists) => exists.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => v.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Exists(exists) => exists.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}
impl From<Exists<Type>> for Type {
    fn from(ex: Exists<Type>) -> Type {
        Type::Exists(ex)
    }
}

impl From<TypeVariable> for Type {
    fn from(v: TypeVariable) -> Type {
        Type::Var(v)
    }
}
impl From<Unit> for Type {
    fn from(u: Unit) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool> for Type {
    fn from(b: Bool) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat> for Type {
    fn from(n: Nat) -> Type {
        Type::Nat(n)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}
