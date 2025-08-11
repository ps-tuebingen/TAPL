use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Type as TypeTrait,
        TypeGroup, Unit, Variant,
    },
};

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

impl TypeTrait for Type {}

impl TypeGroup for Type {
    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<Type>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_bot(self) -> Result<Bot, TypeMismatch> {
        if let Type::Bot(bot) = self {
            Ok(bot)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bot".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Type>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Type>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_list(self) -> Result<List<Type>, TypeMismatch> {
        if let Type::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
        }
    }

    fn into_ref(self) -> Result<Reference<Type>, TypeMismatch> {
        if let Type::Ref(reft) = self {
            Ok(reft)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
        }
    }

    fn into_source(self) -> Result<Source<Type>, TypeMismatch> {
        if let Type::Source(src) = self {
            Ok(src)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Source".to_owned()))
        }
    }

    fn into_sink(self) -> Result<Sink<Type>, TypeMismatch> {
        if let Type::Sink(sink) = self {
            Ok(sink)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Sink".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Type>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Top::<Type>::rule(),
            Bot::rule(),
            Fun::<Type>::rule(),
            Record::<Type>::rule(),
            Variant::<Type>::rule(),
            List::<Type>::rule(),
            Reference::<Type>::rule(),
            Source::<Type>::rule(),
            Sink::<Type>::rule(),
            Nat::<Type>::rule(),
            Unit::<Type>::rule(),
            Bool::<Type>::rule(),
        ])
    }
}

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

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Top(top) => top.to_latex(conf),
            Type::Bot(bot) => bot.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
            Type::Variant(variant) => variant.to_latex(conf),
            Type::List(list) => list.to_latex(conf),
            Type::Ref(refty) => refty.to_latex(conf),
            Type::Source(src) => src.to_latex(conf),
            Type::Sink(snk) => snk.to_latex(conf),
            Type::Nat(nat) => nat.to_latex(conf),
            Type::Unit(unit) => unit.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
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
