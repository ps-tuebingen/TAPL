use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Unit, Variant},
    TypeVar, Var,
};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Unit(Unit),
    Fun(Fun<Type>),
    Bool(Bool),
    Nat(Nat),
    Prod(Product<Type>),
    Tup(Tuple<Type>),
    Record(Record<Type>),
    Sum(Sum<Type>),
    Variant(Variant<Type>),
    Optional(Optional<Type>),
    List(List<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Prod(prod) => prod.fmt(f),
            Type::Tup(tup) => tup.fmt(f),
            Type::Record(recs) => recs.fmt(f),
            Type::Sum(sum) => sum.fmt(f),
            Type::Variant(vars) => vars.fmt(f),
            Type::Optional(option) => option.fmt(f),
            Type::List(list) => list.fmt(f),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Type;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}
