use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Unit, Variant},
    TypeVar,
};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Unit(Unit<Type>),
    Fun(Fun<Type>),
    Bool(Bool<Type>),
    Nat(Nat<Type>),
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

impl From<Unit<Type>> for Type {
    fn from(u: Unit<Type>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

impl From<Product<Type>> for Type {
    fn from(prod: Product<Type>) -> Type {
        Type::Prod(prod)
    }
}

impl From<Tuple<Type>> for Type {
    fn from(tup: Tuple<Type>) -> Type {
        Type::Tup(tup)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl From<Sum<Type>> for Type {
    fn from(sum: Sum<Type>) -> Type {
        Type::Sum(sum)
    }
}

impl From<Variant<Type>> for Type {
    fn from(var: Variant<Type>) -> Type {
        Type::Variant(var)
    }
}

impl From<Optional<Type>> for Type {
    fn from(opt: Optional<Type>) -> Type {
        Type::Optional(opt)
    }
}

impl From<List<Type>> for Type {
    fn from(ls: List<Type>) -> Type {
        Type::List(ls)
    }
}
