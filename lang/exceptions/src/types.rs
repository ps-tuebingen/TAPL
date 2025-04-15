use common::types::{Bool, Fun, Nat, Unit};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit(Unit),
    Nat(Nat),
    Bool(Bool),
    Fun(Fun<Type>),
}

impl common::types::Type for Type {}

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
