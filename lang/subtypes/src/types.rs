use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Unit, Variant},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Top(Top<Type>),
    Bot(Bot),
    Fun(Fun<Type>),
    Record(Record<Type>),
    Variant(Variant<Type>),
    List(List<Type>),
    Ref(Reference<Type>),
    Source(Source<Type>),
    Sink(Sink<Type>),
    Nat(Nat<Type>),
    Unit(Unit<Type>),
    Bool(Bool<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Top(top) => top.fmt(f),
            Type::Bot(bot) => bot.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Variant(variant) => variant.fmt(f),
            Type::List(list) => list.fmt(f),
            Type::Ref(refty) => refty.fmt(f),
            Type::Source(src) => src.fmt(f),
            Type::Sink(snk) => snk.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
            Type::Unit(unit) => unit.fmt(f),
            Type::Bool(b) => b.fmt(f),
        }
    }
}

impl From<Source<Type>> for Type {
    fn from(src: Source<Type>) -> Type {
        Type::Source(src)
    }
}
impl From<Sink<Type>> for Type {
    fn from(sink: Sink<Type>) -> Type {
        Type::Sink(sink)
    }
}

impl From<Reference<Type>> for Type {
    fn from(reft: Reference<Type>) -> Type {
        Type::Ref(reft)
    }
}
impl From<Bot> for Type {
    fn from(b: Bot) -> Type {
        Type::Bot(b)
    }
}
impl From<Top<Type>> for Type {
    fn from(t: Top<Type>) -> Type {
        Type::Top(t)
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

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl From<Variant<Type>> for Type {
    fn from(var: Variant<Type>) -> Type {
        Type::Variant(var)
    }
}

impl From<List<Type>> for Type {
    fn from(ls: List<Type>) -> Type {
        Type::List(ls)
    }
}
