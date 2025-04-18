use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Fun, Nat, Reference, Unit},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit(Unit),
    Nat(Nat),
    Fun(Fun<Type>),
    Ref(Reference<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

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
            Type::Fun(fun) => fun.fmt(f),
            Type::Ref(rf) => rf.fmt(f),
        }
    }
}

impl From<Unit> for Type {
    fn from(u: Unit) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat> for Type {
    fn from(n: Nat) -> Type {
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
